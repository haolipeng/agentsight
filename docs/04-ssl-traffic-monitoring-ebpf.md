# 任务 04: SSL/TLS 流量监控 eBPF 程序实现

## 任务目标

实现 SSL/TLS 加密流量监控 eBPF 程序,通过 uprobe 追踪 SSL 库函数调用,捕获明文数据,支持多种 SSL 库(OpenSSL, BoringSSL, GnuTLS),输出结构化的 JSON 数据。

## 前置条件

- **任务 01** 已完成(环境搭建)
- **任务 02** 已完成(eBPF 基础)
- **任务 03** 已完成(进程监控)
- 理解 SSL/TLS 协议基础
- 了解动态链接和符号解析

## 涉及的文件

- `bpf/sslsniff.bpf.c` (新建 - SSL 监控 eBPF 程序)
- `bpf/sslsniff.c` (新建 - 用户态加载器和 JSON 输出)
- `bpf/Makefile` (修改 - 添加构建规则)

## 实现步骤

### 1. 理解 SSL 库函数拦截

#### 1.1 目标函数识别
- OpenSSL: SSL_write, SSL_read, SSL_write_ex, SSL_read_ex
- BoringSSL: 与 OpenSSL 兼容的 API
- GnuTLS: gnutls_record_send, gnutls_record_recv
- 理解函数参数和返回值

#### 1.2 Uprobe 机制
- 理解 uprobe 如何挂载到用户态函数
- 理解 uretprobe 捕获返回值
- 理解如何获取函数参数(通过 pt_regs)

#### 1.3 库路径查找
- 在 /proc/[pid]/maps 中查找 SSL 库路径
- 处理不同系统的库路径(如 /lib, /usr/lib, /usr/local/lib)
- 处理库版本号后缀

### 2. 设计数据结构

#### 2.1 事件类型
- SSL 写入事件(明文数据发送)
- SSL 读取事件(明文数据接收)
- 连接信息事件
- 包含函数调用上下文

#### 2.2 事件结构体
- 时间戳(纳秒精度)
- 进程 ID、线程 ID、进程名
- 函数名称(SSL_write, SSL_read 等)
- 数据长度和实际拷贝的字节数
- 数据缓冲区(限制大小,如 4KB 或 8KB)
- 连接标识(SSL 对象指针)
- 函数延迟(进入和退出的时间差)

#### 2.3 临时存储 Maps
- 使用 BPF_MAP_TYPE_HASH 存储进入时的上下文
- Key: 线程 ID
- Value: 函数参数(SSL 对象、buffer 指针、长度等)

### 3. 实现 eBPF 内核态程序

#### 3.1 挂载 uprobe 到函数入口
- 为每个目标函数定义 uprobe 处理函数
- 从 pt_regs 读取函数参数
- 保存参数到临时 map 中(用于 uretprobe 使用)
- 记录进入时间戳

#### 3.2 挂载 uretprobe 到函数返回
- 从临时 map 中获取函数参数
- 读取返回值(实际传输的字节数)
- 计算函数延迟
- 从用户态地址空间读取数据缓冲区

#### 3.3 安全读取用户态数据
- 使用 bpf_probe_read_user 读取缓冲区数据
- 处理读取失败的情况
- 限制读取大小,避免 eBPF 栈溢出
- 处理 NULL 指针和无效地址

#### 3.4 数据截断处理
- 对于大数据,只捕获前 N 字节
- 记录实际数据长度和捕获长度
- 在事件中标记是否截断

#### 3.5 过滤逻辑
- 支持按进程名过滤
- 支持按进程 ID 过滤
- 支持按数据类型过滤(二进制或文本)
- 在内核态过滤可减少用户态负担

### 4. 实现用户态加载器

#### 4.1 查找 SSL 库路径
- 扫描常见的库路径
- 使用 ldconfig 查找库
- 或允许用户指定库路径
- 处理符号链接

#### 4.2 附加 uprobe
- 对每个目标进程附加 uprobe
- 或全局附加(监控所有使用 SSL 库的进程)
- 处理函数符号解析失败的情况
- 提供详细的调试信息

#### 4.3 命令行参数
- 支持按进程名过滤(-c 或 --comm)
- 支持按进程 ID 过滤(-p 或 --pid)
- 支持指定 SSL 库路径(--ssl-lib)
- 支持数据截断大小配置(--max-data-size)
- 支持详细模式(-v 或 --verbose)

#### 4.4 事件处理和 JSON 输出
- 从 ring buffer 接收事件
- 格式化为 JSON 输出
- 处理二进制数据的编码(如 base64 或十六进制)
- 包含所有相关元数据

#### 4.5 数据编码
- 对于文本数据,直接输出(转义特殊字符)
- 对于二进制数据,使用 base64 或十六进制编码
- 自动检测数据类型(是否为可打印 ASCII)
- 标记数据编码类型

### 5. 处理多 SSL 库支持

#### 5.1 OpenSSL 函数
- SSL_write(SSL *ssl, const void *buf, int num)
- SSL_read(SSL *ssl, void *buf, int num)
- SSL_write_ex(SSL *ssl, const void *buf, size_t num, size_t *written)
- SSL_read_ex(SSL *ssl, void *buf, size_t num, size_t *readbytes)

#### 5.2 GnuTLS 函数
- gnutls_record_send(gnutls_session_t session, const void *data, size_t data_size)
- gnutls_record_recv(gnutls_session_t session, void *data, size_t data_size)

#### 5.3 函数签名差异处理
- 不同库的函数参数顺序和类型可能不同
- 需要为每个库编写特定的参数读取逻辑
- 或使用配置文件定义函数签名

### 6. 性能优化

#### 6.1 减少数据拷贝
- 只拷贝必要的数据量
- 对于大数据,采样而不是全量捕获
- 提供配置选项

#### 6.2 高效的过滤
- 在 eBPF 内核态尽早过滤
- 避免不必要的数据读取和传输
- 使用 map 缓存过滤结果

#### 6.3 Ring buffer 大小调优
- 根据流量大小调整 buffer
- 监控 buffer 丢失事件
- 提供配置参数

## 验收标准

### 功能验收
1. eBPF 程序成功附加到 SSL 库函数
2. 能够捕获 SSL 读写操作
3. 明文数据正确提取
4. JSON 输出格式正确
5. 支持 OpenSSL 库
6. 过滤功能正常工作
7. 性能开销小于 5%

### 数据质量
1. 捕获的数据完整且准确
2. 二进制数据正确编码
3. 时间戳准确
4. 连接标识一致(同一 SSL 连接的读写可关联)

## 测试方法

### 手动测试

#### 测试 1: 编译程序
```bash
cd bpf
make sslsniff
```
**预期结果**: 编译成功,生成 sslsniff 可执行文件

#### 测试 2: 监控 curl 命令
```bash
sudo ./bpf/sslsniff -c curl &
curl https://www.example.com
```
**预期结果**:
- 捕获到 curl 的 SSL 写入和读取事件
- 输出 HTTP 请求和响应的明文数据
- JSON 格式正确

#### 测试 3: 监控 Python 脚本
创建测试脚本:
```python
import urllib.request
urllib.request.urlopen('https://www.example.com').read()
```
运行监控:
```bash
sudo ./bpf/sslsniff -c python &
python test_ssl.py
```
**预期结果**: 捕获到 Python 的 SSL 流量

#### 测试 4: JSON 验证
```bash
sudo ./bpf/sslsniff -c curl > /tmp/ssl_events.json &
curl https://www.example.com
# 验证 JSON
python3 -m json.tool < /tmp/ssl_events.json
```
**预期结果**: JSON 格式合法,可以解析

#### 测试 5: 数据完整性验证
使用 tcpdump 对比:
```bash
sudo tcpdump -i any -w /tmp/traffic.pcap &
sudo ./bpf/sslsniff -c curl > /tmp/ssl_plain.json &
curl https://www.example.com
```
**预期结果**:
- eBPF 捕获的明文数据对应加密流量
- 数据包数量一致

#### 测试 6: 压力测试
```bash
sudo ./bpf/sslsniff &
# 生成大量 HTTPS 请求
for i in {1..100}; do curl -s https://www.example.com > /dev/null & done
wait
```
**预期结果**:
- 程序稳定运行
- 捕获所有请求
- 无崩溃或数据丢失

#### 测试 7: 性能影响测试
不使用监控时:
```bash
time curl https://www.example.com > /dev/null
```
使用监控时:
```bash
sudo ./bpf/sslsniff -c curl &
time curl https://www.example.com > /dev/null
```
**预期结果**: 性能差异小于 5%

### 集成测试

#### 测试 8: 监控实际应用
监控一个真实的 Python 或 Node.js 应用:
```bash
sudo ./bpf/sslsniff -c python > /tmp/app_traffic.json &
# 运行应用
python your_app.py
# 分析捕获的数据
cat /tmp/app_traffic.json | grep "SSL_write"
```
**预期结果**: 捕获到应用的所有 HTTPS 流量

## 常见问题和排查

### 问题 1: 找不到 SSL 库
**现象**: "failed to find SSL library"
**排查**:
- 检查系统是否安装了 OpenSSL
- 使用 ldconfig -p | grep ssl 查找库路径
- 手动指定库路径

### 问题 2: 符号解析失败
**现象**: "failed to resolve symbol SSL_write"
**排查**:
- 使用 nm -D 查看库的导出符号
- 检查库是否被 strip
- 某些静态链接的程序可能无法追踪

### 问题 3: 捕获不到数据
**现象**: 程序运行但无输出
**排查**:
- 检查 uprobe 是否成功附加
- 使用 bpftool prog list 查看加载的程序
- 检查过滤条件是否过于严格
- 确认目标程序确实使用了目标 SSL 库

### 问题 4: 数据乱码
**现象**: 输出的数据不可读
**排查**:
- 检查是否读取了正确的内存地址
- 确认数据编码方式
- 某些压缩或二进制数据需要特殊处理

### 问题 5: 性能影响大
**现象**: 系统明显变慢
**排查**:
- 减少捕获的数据量
- 增加过滤条件
- 只监控特定进程

### 问题 6: eBPF 验证器错误
**现象**: "R2 min value is negative"
**排查**:
- 检查数组访问的边界检查
- 确保所有指针访问前都有 NULL 检查
- 限制循环和栈使用

## 注意事项

1. **隐私和安全**: SSL 流量包含敏感信息,务必谨慎处理
2. **法律合规**: 只监控授权的系统和应用
3. **性能**: 大流量场景下注意性能影响
4. **兼容性**: 不同 SSL 库版本可能有 API 差异
5. **数据大小**: 注意 eBPF 栈和 ring buffer 限制

## 下一步

完成 SSL 流量监控后,可以继续:
- **任务 05**: 实现 Rust 框架核心事件系统
- 开始构建数据处理框架
