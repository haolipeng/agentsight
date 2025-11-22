# 任务 11: CLI 命令行接口实现

## 任务目标

实现完整的命令行接口,支持多个子命令(ssl, process, trace, record),提供友好的参数解析和帮助信息,整合所有框架组件。

## 前置条件

- **任务 05-10** 已完成(框架所有核心组件)
- 理解 clap 库的使用
- 理解 CLI 设计最佳实践

## 涉及的文件

- `collector/src/main.rs` (修改 - CLI 入口点)
- `collector/src/cli/mod.rs` (新建 - CLI 模块)
- `collector/src/cli/ssl_command.rs` (新建 - ssl 子命令)
- `collector/src/cli/process_command.rs` (新建 - process 子命令)
- `collector/src/cli/trace_command.rs` (新建 - trace 子命令)
- `collector/src/cli/record_command.rs` (新建 - record 子命令)
- `collector/Cargo.toml` (修改 - 添加 clap 依赖)

## 实现步骤

### 1. 设计 CLI 架构

#### 1.1 命令结构
```
agentsight
├── ssl       # 监控 SSL 流量
├── process   # 监控进程事件
├── trace     # 组合监控(SSL + Process)
└── record    # 优化的 Agent 记录模式
```

#### 1.2 通用选项
- --verbose, -v: 详细输出
- --log-level: 日志级别
- --help, -h: 帮助信息
- --version, -V: 版本信息

#### 1.3 子命令特定选项
每个子命令有自己的选项集

### 2. 使用 Clap 定义 CLI

#### 2.1 添加依赖
```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
```

#### 2.2 定义主 CLI 结构
```rust
#[derive(Parser)]
#[command(name = "agentsight")]
#[command(about = "AI Agent observability framework")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
```

#### 2.3 定义子命令枚举
```rust
#[derive(Subcommand)]
enum Commands {
    Ssl(SslArgs),
    Process(ProcessArgs),
    Trace(TraceArgs),
    Record(RecordArgs),
}
```

### 3. 实现 SSL 子命令

#### 3.1 参数定义
- --comm, -c: 进程名过滤
- --pid, -p: 进程 ID 过滤
- --ssl-lib: SSL 库路径
- --max-data-size: 最大数据捕获大小
- --ssl-filter: SSL 过滤表达式
- --http-filter: HTTP 过滤表达式
- --sse-merge: 启用 SSE 数据块合并
- --log-file: 输出日志文件
- --server: 启动 Web 服务器
- --server-port: Web 服务器端口

#### 3.2 执行逻辑
- 创建 SSLRunner
- 配置过滤器和 Analyzer 链
- 启动事件处理
- 可选启动 Web 服务器

#### 3.3 Analyzer 链构建
- SSLRunner → ChunkMerger → HTTPParser
  → SSLFilter → HTTPFilter → AuthRemover
  → FileLogger → Output

### 4. 实现 Process 子命令

#### 4.1 参数定义
- --comm, -c: 进程名过滤
- --pid, -p: 进程 ID 过滤
- --binary-path: 二进制路径过滤
- --verbose, -v: 详细模式
- --log-file: 输出日志文件
- --server: 启动 Web 服务器
- --server-port: 端口

#### 4.2 执行逻辑
- 创建 ProcessRunner
- 配置过滤选项
- 启动事件处理

#### 4.3 Analyzer 链
- ProcessRunner → FileLogger → Output

### 5. 实现 Trace 子命令

#### 5.1 参数定义
- 组合 SSL 和 Process 的所有选项
- --ssl: 启用 SSL 监控
- --process: 启用进程监控
- 其他选项同上

#### 5.2 执行逻辑
- 根据选项创建多个 Runner
- 合并事件流
- 按时间戳排序
- 应用 Analyzer 链

#### 5.3 事件流合并
- 使用 tokio::select! 或 futures::stream::select
- 保持时间顺序
- 处理并发

### 6. 实现 Record 子命令

#### 6.1 优化配置
- 预设的 Analyzer 链优化 AI Agent 监控
- 默认启用 HTTP 解析
- 默认移除敏感头
- 默认启用 Web 服务器

#### 6.2 参数简化
- --comm, -c: 必需参数
- --binary-path: 可选,支持 "auto" 自动检测
- --server-port: 服务器端口,默认 7395
- --log-file: 日志文件

#### 6.3 自动配置
- 自动检测 Node.js 路径(如果使用 nvm)
- 自动打开浏览器(可选)
- 友好的输出信息

### 7. 实现主函数

#### 7.1 初始化
- 设置日志(env_logger)
- 解析命令行参数
- 检查权限(eBPF 需要 root)

#### 7.2 分发命令
- 根据子命令调用相应处理函数
- 统一的错误处理
- 优雅的退出

#### 7.3 信号处理
- 捕获 SIGINT(Ctrl+C)
- 优雅关闭所有组件
- 清理资源

### 8. 实现帮助和文档

#### 8.1 帮助文本
- 每个命令有清晰的描述
- 参数有详细说明
- 提供使用示例

#### 8.2 示例
```
# 监控 curl 的 SSL 流量
agentsight ssl --comm curl

# 监控进程并启动 Web UI
agentsight process --comm python --server

# 组合监控
agentsight trace --ssl --process --comm node --server

# Agent 记录模式
agentsight record --comm claude --server-port 7395
```

### 9. 错误处理

#### 9.1 友好的错误消息
- 权限错误:提示需要 sudo
- 文件不存在:提示路径
- 参数冲突:说明原因

#### 9.2 退出码
- 0: 成功
- 1: 一般错误
- 2: 参数错误
- 130: 用户中断(Ctrl+C)

### 10. 测试实现

#### 10.1 单元测试
- 测试参数解析
- 测试配置构建
- 测试错误处理

#### 10.2 集成测试
- 测试每个子命令
- 测试参数组合
- 测试错误场景

## 验收标准

### 功能验收
1. 所有子命令正常工作
2. 参数解析正确
3. 帮助信息清晰
4. 错误处理友好
5. 支持 --help 和 --version

### 用户体验
1. 命令直观易用
2. 错误消息有帮助
3. 示例完整可运行
4. 文档准确

## 测试方法

### 单元测试

#### 测试 1: 测试参数解析
```bash
cd collector
cargo test cli::test_parse_args
```
**预期结果**: 各种参数组合正确解析

### 集成测试

#### 测试 2: 测试 help 命令
```bash
cd collector
cargo run -- --help
cargo run -- ssl --help
cargo run -- process --help
cargo run -- trace --help
cargo run -- record --help
```
**预期结果**: 显示清晰的帮助信息

#### 测试 3: 测试 version
```bash
cd collector
cargo run -- --version
```
**预期结果**: 显示版本号

### 手动测试

#### 测试 4: 测试 SSL 监控
```bash
cd collector
sudo cargo run -- ssl --comm curl &
curl https://www.example.com
```
**预期结果**: 捕获并显示 SSL 流量

#### 测试 5: 测试 Process 监控
```bash
cd collector
sudo cargo run -- process --comm ls
ls /tmp
```
**预期结果**: 显示 ls 的进程事件

#### 测试 6: 测试 Trace 命令
```bash
cd collector
sudo cargo run -- trace --ssl --process --comm python --server
```
**预期结果**: 同时监控 SSL 和进程,Web 服务器启动

#### 测试 7: 测试 Record 命令
```bash
cd collector
sudo cargo run -- record --comm curl
```
**预期结果**: 优化配置,自动启用各种功能

#### 测试 8: 测试错误处理
```bash
cd collector
cargo run -- ssl --comm curl  # 没有 sudo
```
**预期结果**: 友好的权限错误提示

#### 测试 9: 测试参数冲突
```bash
cd collector
sudo cargo run -- ssl --pid 123 --comm test
```
**预期结果**: 警告或错误提示

## 常见问题和排查

### 问题 1: 权限错误
**现象**: "permission denied"
**排查**:
- 确认使用 sudo 运行
- 检查 eBPF 支持

### 问题 2: 参数不识别
**现象**: "unknown option"
**排查**:
- 检查参数拼写
- 查看 --help
- 确认子命令正确

### 问题 3: 子命令不工作
**现象**: 命令执行但无效果
**排查**:
- 检查是否有匹配的进程
- 查看日志输出
- 使用 --verbose 模式

## 注意事项

1. **权限检查**: 启动时检查权限
2. **参数验证**: 验证参数合法性
3. **用户体验**: 提供清晰的反馈
4. **向后兼容**: 保持 CLI 稳定
5. **文档同步**: 文档与代码一致

## 学习要点

1. **Clap 库**: 强大的 CLI 框架
2. **CLI 设计**: 用户友好的命令行设计
3. **错误处理**: 如何提供有用的错误信息
4. **异步集成**: 在 CLI 中使用异步代码

## 下一步

完成 CLI 后,可以继续:
- **任务 12**: 实现 Web 服务器和前端集成
- 添加可视化界面
