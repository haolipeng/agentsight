# 任务 06: Rust 框架 - Runner Trait 和基础架构

## 任务目标

设计和实现 Runner trait,定义数据收集器的通用接口,实现进程管理、输出解析、流式事件生成的基础架构。

## 前置条件

- **任务 05** 已完成(核心事件系统)
- 理解 Rust trait 和 async/await
- 熟悉 tokio 异步运行时
- 了解 Unix 进程管理

## 涉及的文件

- `collector/src/framework/runners/mod.rs` (新建 - Runner trait 定义)
- `collector/src/framework/runners/base.rs` (新建 - 基础实现)
- `collector/Cargo.toml` (修改 - 添加 tokio 等依赖)

## 实现步骤

### 1. 设计 Runner 架构

#### 1.1 Runner 职责
- 执行 eBPF 二进制程序
- 解析 stdout 输出为 JSON
- 将 JSON 转换为 Event 对象
- 生成事件流供下游消费

#### 1.2 异步流式设计
- 使用 tokio::process 管理子进程
- 使用 tokio::io 异步读取输出
- 使用 async Stream 生成事件
- 支持背压和流控

#### 1.3 配置和构建
- 使用 Builder 模式配置 Runner
- 链式调用设置参数
- 类型安全的配置

### 2. 定义 Runner Trait

#### 2.1 核心方法
- `async fn run(&mut self) -> Result<EventStream>`: 启动并返回事件流
- `async fn stop(&mut self) -> Result<()>`: 停止 Runner
- 使用 async_trait 宏支持异步 trait

#### 2.2 事件流类型
- 使用 `Pin<Box<dyn Stream<Item = Result<Event>> + Send>>`
- 或定义类型别名简化使用
- 确保线程安全(Send + Sync)

#### 2.3 错误处理
- 定义 RunnerError 枚举
- 包含进程错误、IO 错误、解析错误等
- 实现 std::error::Error trait

### 3. 实现基础进程管理

#### 3.1 进程启动
- 使用 tokio::process::Command 创建子进程
- 配置 stdin, stdout, stderr 重定向
- 设置环境变量和工作目录
- 处理启动失败

#### 3.2 进程监控
- 监控进程状态
- 处理进程异常退出
- 捕获 stderr 输出用于调试
- 实现优雅关闭

#### 3.3 资源清理
- 确保进程正确终止
- 清理临时文件
- 释放文件句柄
- 处理僵尸进程

### 4. 实现输出解析

#### 4.1 逐行读取
- 使用 BufReader 包装 stdout
- 使用 lines() 异步迭代器
- 处理 UTF-8 解码错误
- 处理不完整的行

#### 4.2 JSON 解析
- 每行解析为 serde_json::Value
- 处理解析错误,记录日志但继续运行
- 验证 JSON 结构
- 提取必要字段

#### 4.3 Event 转换
- 从 JSON 创建 Event 对象
- 填充元数据
- 设置事件类型
- 处理缺失字段

### 5. 实现事件流生成

#### 5.1 使用 async-stream
- 使用 async_stream::stream! 宏
- 在流中 yield 事件
- 处理异常和错误传播

#### 5.2 背压处理
- 当下游处理慢时,自动缓冲
- 配置缓冲区大小
- 避免内存无限增长

#### 5.3 错误传播
- 将解析错误包装在 Result 中
- 允许下游决定如何处理错误
- 记录错误日志

### 6. 实现 Builder 模式

#### 6.1 RunnerBuilder 结构
- 包含所有配置选项
- 使用 Option 类型存储可选配置
- 提供默认值

#### 6.2 Builder 方法
- 每个配置项一个方法
- 返回 &mut Self 支持链式调用
- 类型安全的参数

#### 6.3 Build 方法
- 验证配置完整性
- 创建 Runner 实例
- 返回 Result 处理错误

### 7. 添加依赖

#### 7.1 Tokio 相关
- tokio: 异步运行时,启用 full features
- async-trait: 异步 trait 支持
- async-stream: 简化异步流创建

#### 7.2 其他依赖
- futures: Stream trait 和工具
- log: 日志记录
- thiserror: 错误定义

## 验收标准

### 功能验收
1. Runner trait 定义清晰,接口合理
2. 可以启动和停止子进程
3. 可以解析 JSON 输出
4. 可以生成事件流
5. Builder 模式工作正常
6. 错误处理完善

### 代码质量
1. 异步代码正确使用(无阻塞操作)
2. 资源正确清理(无泄漏)
3. 有完整的文档注释
4. 通过 clippy 检查

## 测试方法

### 单元测试

#### 测试 1: 测试 Builder
```bash
cd collector
cargo test runners::test_builder
```
**预期结果**: Builder 功能正常

#### 测试 2: 测试进程管理
```bash
cd collector
cargo test runners::test_process
```
**预期结果**: 可以启动和停止进程

#### 测试 3: 测试 JSON 解析
```bash
cd collector
cargo test runners::test_json_parse
```
**预期结果**: JSON 正确解析为 Event

### 集成测试

#### 测试 4: 使用 echo 命令测试
创建简单的测试 Runner 执行 echo 命令:
```bash
cd collector
cargo test test_echo_runner -- --nocapture
```
**预期结果**:
- 可以执行命令
- 可以读取输出
- 输出正确解析

#### 测试 5: 测试错误处理
测试命令不存在、命令失败等情况:
```bash
cd collector
cargo test test_runner_errors
```
**预期结果**: 错误正确处理和传播

### 手动测试

#### 测试 6: 编译检查
```bash
cd collector
cargo build
```
**预期结果**: 编译成功

#### 测试 7: 异步运行时检查
确保所有异步代码在 tokio 运行时中执行:
```bash
cd collector
cargo test --lib
```
**预期结果**: 所有测试通过,无运行时错误

## 常见问题和排查

### 问题 1: 异步函数编译错误
**现象**: "async trait methods are unstable"
**排查**:
- 添加 async-trait 依赖
- 在 trait 定义上使用 #[async_trait]

### 问题 2: 进程僵尸
**现象**: 进程退出后变成僵尸进程
**排查**:
- 确保调用 child.wait()
- 或使用 tokio::process 自动处理

### 问题 3: 管道阻塞
**现象**: 读取 stdout 时程序hang住
**排查**:
- 确保异步读取
- 检查是否有死锁
- 验证 tokio 运行时正确配置

### 问题 4: UTF-8 解码错误
**现象**: 某些输出无法解码
**排查**:
- 使用 lossy 转换
- 或跳过无效字符
- 记录警告日志

### 问题 5: JSON 解析失败
**现象**: 某些行解析失败
**排查**:
- 检查 eBPF 程序输出格式
- 添加调试日志
- 验证 JSON 格式

## 注意事项

1. **异步编程**: 避免阻塞操作,使用异步 IO
2. **资源管理**: 确保所有资源在 Drop 时清理
3. **错误处理**: 不要 panic,返回 Result
4. **线程安全**: Runner 可能在多线程环境中使用
5. **性能**: 避免不必要的内存分配和拷贝

## 学习要点

1. **Tokio 异步编程**: 理解 async/await 和异步 IO
2. **Stream**: 理解 Rust 中的异步流
3. **进程管理**: 理解 Unix 进程模型
4. **Builder 模式**: 理解 Rust 中的构建器模式
5. **Trait 设计**: 如何设计通用的 trait 接口

## 下一步

完成 Runner 基础架构后,可以继续:
- **任务 07**: 实现 SSL Runner
- **任务 08**: 实现 Process Runner
- 实现具体的数据收集器
