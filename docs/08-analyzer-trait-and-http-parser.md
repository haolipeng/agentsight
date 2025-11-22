# 任务 08: Analyzer Trait 和 HTTP 解析器实现

## 任务目标

设计和实现 Analyzer trait 定义数据处理管道的通用接口,实现 HTTP 协议解析器用于解析 SSL 流量中的 HTTP 请求和响应。

## 前置条件

- **任务 05** 已完成(核心事件系统)
- **任务 06** 已完成(Runner trait)
- 理解 HTTP/1.1 协议
- 理解 chunked transfer encoding 和压缩

## 涉及的文件

- `collector/src/framework/analyzers/mod.rs` (新建 - Analyzer trait 定义)
- `collector/src/framework/analyzers/http_parser.rs` (新建 - HTTP 解析器)
- `collector/src/framework/analyzers/chunk_merger.rs` (新建 - 数据块合并)
- `collector/Cargo.toml` (修改 - 添加 HTTP 相关依赖)

## 实现步骤

### 1. 设计 Analyzer 架构

#### 1.1 Analyzer 职责
- 接收事件流作为输入
- 处理和转换事件
- 输出新的事件流
- 支持链式组合多个 Analyzer

#### 1.2 流式处理设计
- 输入: `Stream<Item = Result<Event>>`
- 输出: `Stream<Item = Result<Event>>`
- 支持过滤、转换、合并、分发等操作

#### 1.3 状态管理
- Analyzer 可以有内部状态
- 状态在多个事件之间保持
- 支持异步处理

### 2. 定义 Analyzer Trait

#### 2.1 核心方法
```rust
#[async_trait]
pub trait Analyzer: Send + Sync {
    async fn analyze(&mut self, stream: EventStream) -> Result<EventStream>;
}
```

#### 2.2 辅助 trait 方法
- name(): 返回 Analyzer 名称
- description(): 返回描述
- 可选的配置方法

#### 2.3 Analyzer 链
- 支持多个 Analyzer 串联
- 提供便捷的链式构建方法
- 自动处理流的传递

### 3. 实现 ChunkMerger Analyzer

#### 3.1 功能描述
- 将同一连接的多个 SSL 数据块合并
- 按方向分组(发送/接收)
- 识别消息边界

#### 3.2 状态管理
- 使用 HashMap 存储每个连接的缓冲区
- Key: (进程 ID, SSL 对象指针, 方向)
- Value: 累积的数据缓冲区

#### 3.3 合并逻辑
- 接收 SSLWrite/SSLRead 事件
- 追加数据到对应缓冲区
- 检测消息完整性
- 当消息完整时,发出合并后的事件

#### 3.4 消息边界检测
- 对于 HTTP,检测请求/响应结束标记
- 支持 Content-Length 头
- 支持 chunked encoding 结束标记
- 超时机制(避免无限缓冲)

### 4. 实现 HTTP Parser Analyzer

#### 4.1 功能描述
- 解析合并后的 SSL 数据
- 识别 HTTP 请求和响应
- 提取 HTTP 头和 body
- 处理压缩和编码

#### 4.2 HTTP 请求解析
- 解析请求行(方法、路径、版本)
- 解析请求头(逐行)
- 提取 body(根据 Content-Length 或 chunked)
- 创建 HTTPRequest 事件

#### 4.3 HTTP 响应解析
- 解析状态行(版本、状态码、原因短语)
- 解析响应头
- 处理 Transfer-Encoding: chunked
- 处理 Content-Encoding: gzip/deflate
- 创建 HTTPResponse 事件

#### 4.4 Chunked Encoding 处理
- 解析 chunk size 行
- 读取 chunk data
- 处理 chunk extensions
- 识别结束 chunk(size 0)
- 重组完整 body

#### 4.5 压缩处理
- 检测 Content-Encoding 头
- gzip 解压缩
- deflate 解压缩
- 处理解压失败情况

#### 4.6 字符编码处理
- 检测 Content-Type 中的 charset
- UTF-8 解码
- 处理非 UTF-8 数据(如二进制)
- 使用 lossy 转换避免错误

### 5. 错误处理

#### 5.1 解析错误
- HTTP 格式错误
- 不完整的消息
- 编码错误

#### 5.2 容错设计
- 记录错误但继续处理
- 跳过无法解析的数据
- 输出原始事件作为备选

### 6. 实现依赖库

#### 6.1 添加依赖
- flate2: gzip/deflate 解压缩
- httparse: HTTP 头解析(可选)
- 或手动实现解析器

#### 6.2 优化考虑
- 避免不必要的内存分配
- 使用零拷贝技术
- 流式解析而非加载整个消息到内存

### 7. 测试实现

#### 7.1 单元测试
- 测试 HTTP 请求解析
- 测试 HTTP 响应解析
- 测试 chunked encoding 解析
- 测试压缩处理
- 测试边界情况

#### 7.2 集成测试
- 使用真实的 HTTP 数据
- 测试完整的解析流程
- 验证输出事件正确

## 验收标准

### 功能验收
1. Analyzer trait 定义清晰
2. ChunkMerger 正确合并数据块
3. HTTP Parser 正确解析请求和响应
4. Chunked encoding 正确处理
5. 压缩数据正确解压
6. 错误处理完善

### 数据质量
1. 解析后的 HTTP 消息完整
2. 头字段准确提取
3. Body 完整且正确解码
4. 压缩数据正确还原

## 测试方法

### 单元测试

#### 测试 1: 测试 HTTP 请求解析
```bash
cd collector
cargo test http_parser::test_parse_request
```
**预期结果**: 各种 HTTP 请求正确解析

#### 测试 2: 测试 HTTP 响应解析
```bash
cd collector
cargo test http_parser::test_parse_response
```
**预期结果**: 各种 HTTP 响应正确解析

#### 测试 3: 测试 Chunked Encoding
```bash
cd collector
cargo test http_parser::test_chunked
```
**预期结果**: Chunked 数据正确解析

#### 测试 4: 测试压缩
```bash
cd collector
cargo test http_parser::test_compression
```
**预期结果**: gzip 和 deflate 正确解压

### 集成测试

#### 测试 5: 端到端测试
使用模拟的 SSL 事件流:
```bash
cd collector
cargo test test_http_pipeline -- --nocapture
```
**预期结果**:
- 事件流经 ChunkMerger
- 再经 HTTP Parser
- 输出正确的 HTTP 事件

### 手动测试

#### 测试 6: 实际 HTTP 流量
使用真实的 SSL Runner 捕获流量并解析:
```bash
cd collector
cargo run --bin test_http_analyzer
```
在另一终端:
```bash
curl -H "Accept-Encoding: gzip" https://www.example.com
```
**预期结果**: 正确解析并显示 HTTP 请求和响应

#### 测试 7: 测试各种 HTTP 场景
- GET 请求
- POST 请求(带 body)
- Chunked response
- Gzip 压缩响应
- 大文件下载

**预期结果**: 所有场景正确处理

## 常见问题和排查

### 问题 1: HTTP 解析失败
**现象**: 某些请求无法解析
**排查**:
- 查看原始数据
- 检查 HTTP 格式是否标准
- 添加调试日志

### 问题 2: Chunked 解码错误
**现象**: Body 不完整或乱码
**排查**:
- 检查 chunk size 解析
- 验证 CRLF 处理
- 检查结束 chunk

### 问题 3: 解压失败
**现象**: gzip 数据解压报错
**排查**:
- 确认数据完整
- 检查是否是 gzip 格式
- 尝试 deflate 格式

### 问题 4: 内存使用过高
**现象**: 大文件导致内存暴涨
**排查**:
- 限制缓冲区大小
- 实现流式处理
- 添加内存限制配置

### 问题 5: UTF-8 解码错误
**现象**: Body 包含非 UTF-8 数据
**排查**:
- 使用 lossy 转换
- 或保留原始二进制数据
- 检测并标记数据类型

## 注意事项

1. **内存管理**: 大 HTTP body 需要注意内存使用
2. **错误容忍**: 不完整或格式错误的 HTTP 不应导致崩溃
3. **性能**: 解析是热点路径,需要优化
4. **安全**: 解压炸弹等攻击需要防护
5. **编码**: 正确处理各种字符编码

## 学习要点

1. **HTTP 协议**: 深入理解 HTTP/1.1 规范
2. **流式处理**: 如何高效处理数据流
3. **状态机**: HTTP 解析器是状态机的典型应用
4. **错误处理**: 如何设计容错的解析器

## 下一步

完成 Analyzer 系统后,可以继续:
- **任务 09**: 实现过滤器和文件日志 Analyzer
- **任务 10**: 实现二进制提取和嵌入系统
