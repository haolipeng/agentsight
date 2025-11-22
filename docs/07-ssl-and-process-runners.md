# 任务 07: 实现 SSL Runner 和 Process Runner

## 任务目标

基于 Runner trait 实现 SSL 流量监控 Runner 和进程监控 Runner,支持配置化的二进制路径、命令行参数、事件解析等功能。

## 前置条件

- **任务 06** 已完成(Runner trait 和基础架构)
- **任务 03** 已完成(进程监控 eBPF)
- **任务 04** 已完成(SSL 监控 eBPF)

## 涉及的文件

- `collector/src/framework/runners/ssl_runner.rs` (新建)
- `collector/src/framework/runners/process_runner.rs` (新建)
- `collector/src/framework/runners/mod.rs` (修改 - 导出新模块)

## 实现步骤

### 1. 实现 SSL Runner

#### 1.1 SSLRunner 结构设计
- 包含 eBPF 二进制路径
- 包含过滤配置(进程名、PID 等)
- 包含 SSL 库路径配置
- 包含数据大小限制配置

#### 1.2 SSLRunnerBuilder
- 实现 Builder 模式
- 提供方法设置各项配置
- comm(): 设置进程名过滤
- pid(): 设置 PID 过滤
- ssl_lib(): 设置 SSL 库路径
- max_data_size(): 设置最大数据捕获大小
- build(): 创建 SSLRunner 实例

#### 1.3 实现 Runner trait
- 构建命令行参数数组
- 启动 sslsniff 二进制
- 解析 JSON 输出
- 创建 SSL 事件(SSLWrite, SSLRead)
- 处理 base64 编码的数据

#### 1.4 事件类型映射
- 识别 function 字段(SSL_write, SSL_read 等)
- 创建对应的事件类型
- 提取数据字段并解码

### 2. 实现 Process Runner

#### 2.1 ProcessRunner 结构设计
- 包含 eBPF 二进制路径
- 包含过滤配置
- 包含详细模式开关

#### 2.2 ProcessRunnerBuilder
- comm(): 设置进程名过滤
- pid(): 设置 PID 过滤
- binary_path(): 设置二进制路径过滤
- verbose(): 设置详细模式
- build(): 创建 ProcessRunner 实例

#### 2.3 实现 Runner trait
- 构建命令行参数
- 启动 process 二进制
- 解析 JSON 输出
- 创建进程事件(ProcessFork, ProcessExec, ProcessExit, SystemInfo)

#### 2.4 事件类型识别
- 根据 JSON 中的 type 字段识别事件类型
- fork, exec, exit, system_info 等
- 创建对应的 Event 对象

### 3. 实现通用辅助函数

#### 3.1 命令行参数构建
- 将配置转换为命令行参数数组
- 处理可选参数
- 正确转义特殊字符

#### 3.2 JSON 字段提取
- 安全地从 JSON Value 中提取字段
- 提供默认值处理
- 类型转换辅助函数

#### 3.3 数据解码
- base64 解码函数
- 十六进制解码函数
- 自动检测编码类型

### 4. 错误处理

#### 4.1 Runner 特定错误
- 二进制不存在
- 权限不足(需要 sudo)
- 配置无效

#### 4.2 运行时错误
- eBPF 加载失败
- 进程意外退出
- 输出格式错误

#### 4.3 优雅降级
- 记录错误但继续处理后续事件
- 提供重试机制
- 通知用户错误情况

### 5. 测试实现

#### 5.1 单元测试
- 测试 Builder 构建
- 测试参数构建
- 测试 JSON 解析
- 测试事件创建

#### 5.2 模拟测试
- 使用 mock 数据测试事件流
- 不依赖实际的 eBPF 程序
- 快速验证逻辑正确性

## 验收标准

### 功能验收
1. SSL Runner 可以启动 sslsniff 程序
2. Process Runner 可以启动 process 程序
3. 事件正确解析并创建
4. 过滤功能正常工作
5. 错误处理完善
6. Builder 模式易用

### 数据质量
1. 所有 JSON 字段正确映射到 Event
2. 数据解码正确(base64 等)
3. 时间戳准确
4. 事件类型正确识别

## 测试方法

### 单元测试

#### 测试 1: 测试 Builder
```bash
cd collector
cargo test ssl_runner::test_builder
cargo test process_runner::test_builder
```
**预期结果**: Builder 功能正常

#### 测试 2: 测试 JSON 解析
```bash
cd collector
cargo test ssl_runner::test_parse
cargo test process_runner::test_parse
```
**预期结果**: JSON 正确解析为 Event

### 集成测试

#### 测试 3: 运行 SSL Runner
```bash
cd collector
cargo test test_ssl_runner_integration -- --nocapture
```
**预期结果**:
- SSL Runner 启动成功
- 可以捕获事件
- 事件格式正确

#### 测试 4: 运行 Process Runner
```bash
cd collector
cargo test test_process_runner_integration -- --nocapture
```
**预期结果**:
- Process Runner 启动成功
- 可以捕获事件
- 事件格式正确

### 手动测试

#### 测试 5: 使用 SSL Runner 监控 curl
创建测试程序:
```rust
// 在 src/bin/ 中创建测试二进制
```
运行:
```bash
sudo ./target/debug/test_ssl_runner
```
在另一终端:
```bash
curl https://www.example.com
```
**预期结果**: 捕获到 curl 的 SSL 流量

#### 测试 6: 使用 Process Runner 监控进程
```bash
sudo ./target/debug/test_process_runner
```
在另一终端启动进程:
```bash
ls /tmp
```
**预期结果**: 捕获到 ls 的进程事件

## 常见问题和排查

### 问题 1: 找不到 eBPF 二进制
**现象**: "binary not found"
**排查**:
- 确认二进制已编译(make build)
- 检查路径配置
- 使用绝对路径

### 问题 2: 权限错误
**现象**: "permission denied"
**排查**:
- 使用 sudo 运行
- 检查文件权限
- 检查 capabilities

### 问题 3: JSON 解析失败
**现象**: 部分事件丢失
**排查**:
- 查看 stderr 输出
- 添加调试日志
- 验证 eBPF 程序输出格式

### 问题 4: 事件类型错误
**现象**: 事件类型不匹配
**排查**:
- 检查 JSON type 字段
- 验证事件类型映射逻辑
- 添加日志输出

## 注意事项

1. **权限**: eBPF 程序需要 root 权限
2. **路径**: 使用绝对路径避免找不到二进制
3. **错误日志**: 记录 stderr 用于调试
4. **资源清理**: 确保子进程正确终止

## 下一步

完成 Runner 实现后,可以继续:
- **任务 08**: 实现 Analyzer trait 和基础架构
- 开始构建数据处理管道
