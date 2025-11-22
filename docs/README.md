# AgentSight 项目复现文档

## 文档概述

本文档集提供了从零开始复现 AgentSight 项目的完整指南。每个文档代表一个独立的任务,任务之间有清晰的依赖关系,按照顺序完成可以逐步构建出完整的系统。

## 文档使用说明

### 文档结构

每个任务文档包含以下部分:
- **任务目标**: 清晰描述要实现什么
- **前置条件**: 需要先完成哪些任务
- **涉及的文件**: 会创建或修改哪些文件
- **实现步骤**: 详细的实现思路和关键点(不包含具体代码)
- **验收标准**: 如何判断任务完成
- **测试方法**: 包括单元测试、集成测试和手动测试
- **常见问题和排查**: 可能遇到的问题和解决方案
- **注意事项**: 重要的技术要点
- **学习要点**: 通过此任务可以学到什么
- **下一步**: 推荐的后续任务

### 如何使用

1. **按顺序阅读**: 文档按依赖顺序编号,建议按序完成
2. **理解原理**: 重点理解设计思路和技术原理,而非照搬代码
3. **动手实践**: 根据文档描述自己实现功能
4. **验证结果**: 使用提供的测试方法验证实现正确性
5. **解决问题**: 遇到问题时参考"常见问题和排查"部分

## 项目学习路线

### 学习路径图

```
[环境搭建] → [eBPF 基础] → [进程监控] → [SSL 监控]
                                                    ↓
[最终集成] ← [Web 界面] ← [CLI 实现] ← [二进制嵌入]
    ↑                                        ↓
[过滤器] ← [HTTP 解析] ← [Analyzer 架构] ← [Runner 架构]
    ↑                                        ↓
[事件系统] ←←←←←←←←←←←←←←←←←←←←←←←←←←←←←←
```

### 技能树

完成本项目将掌握以下技能:

**系统编程**
- eBPF 编程和 CO-RE 技术
- Linux 内核追踪技术
- 进程和网络监控

**Rust 开发**
- 异步编程(async/await)
- 流式处理(Stream)
- Trait 设计和实现
- 错误处理最佳实践

**Web 开发**
- HTTP 服务器实现
- React/Next.js 前端开发
- 实时数据展示
- 静态资源嵌入

**软件工程**
- 模块化设计
- 测试驱动开发
- 性能优化
- 文档编写

## 文档索引

### 第一阶段: 基础设施 (任务 00-02)

#### [00. 项目分析](./00-project-analysis.md)
**时间**: 阅读 1 小时
**内容**: 项目整体架构、技术栈、模块划分、难点分析
**前置**: 无
**产出**: 对项目的全面理解

#### [01. 环境搭建和项目初始化](./01-environment-setup.md)
**时间**: 1-2 小时
**内容**: 安装所有依赖,初始化开发环境,验证工具链
**前置**: 00
**产出**: 可用的开发环境

#### [02. eBPF 基础和简单程序开发](./02-ebpf-basics.md)
**时间**: 2-3 小时
**内容**: 学习 eBPF 编程模型,编写 hello world 程序
**前置**: 01
**产出**: 简单的 eBPF 追踪程序

### 第二阶段: eBPF 数据收集层 (任务 03-04)

#### [03. 进程监控 eBPF 程序实现](./03-process-monitoring-ebpf.md)
**时间**: 2-3 小时
**内容**: 实现进程生命周期监控,JSON 输出,工具函数和测试
**前置**: 02
**产出**: 完整的进程监控程序 (bpf/process)

**关键技术**:
- tracepoint 追踪
- task_struct 数据读取
- JSON 格式化
- 单元测试

#### [04. SSL/TLS 流量监控 eBPF 程序实现](./04-ssl-traffic-monitoring-ebpf.md)
**时间**: 2-3 小时
**内容**: 使用 uprobe 追踪 SSL 库函数,捕获明文数据
**前置**: 03
**产出**: SSL 流量监控程序 (bpf/sslsniff)

**关键技术**:
- uprobe/uretprobe
- 用户态内存读取
- 多 SSL 库支持
- 数据编码

### 第三阶段: Rust 框架核心 (任务 05-07)

#### [05. Rust 框架核心 - 事件系统实现](./05-rust-framework-core-events.md)
**时间**: 1-2 小时
**内容**: 设计统一的事件结构,时间戳处理,序列化
**前置**: 01
**产出**: core/events.rs 模块

**关键技术**:
- Serde 序列化
- 时间戳转换
- 类型设计

#### [06. Rust 框架 - Runner Trait 和基础架构](./06-rust-framework-runner-trait.md)
**时间**: 2-3 小时
**内容**: 定义 Runner trait,实现进程管理和 JSON 解析
**前置**: 05
**产出**: runners/mod.rs 和基础 Runner 实现

**关键技术**:
- Async trait
- tokio::process
- Stream 生成
- Builder 模式

#### [07. 实现 SSL Runner 和 Process Runner](./07-ssl-and-process-runners.md)
**时间**: 2-3 小时
**内容**: 基于 Runner trait 实现具体的数据收集器
**前置**: 06, 03, 04
**产出**: ssl_runner.rs 和 process_runner.rs

**关键技术**:
- 命令行参数构建
- 事件类型映射
- 数据解码

### 第四阶段: 数据处理管道 (任务 08-09)

#### [08. Analyzer Trait 和 HTTP 解析器实现](./08-analyzer-trait-and-http-parser.md)
**时间**: 2-3 小时
**内容**: 设计 Analyzer 架构,实现 HTTP 协议解析
**前置**: 07
**产出**: analyzers/http_parser.rs 和 chunk_merger.rs

**关键技术**:
- Stream 转换
- HTTP 协议解析
- Chunked encoding
- Gzip/Deflate 解压

#### [09. 过滤器和文件日志 Analyzer 实现](./09-filters-and-logger-analyzers.md)
**时间**: 1-2 小时
**内容**: 实现各种过滤器和日志记录器
**前置**: 08
**产出**: ssl_filter.rs, http_filter.rs, file_logger.rs 等

**关键技术**:
- 表达式解析
- 过滤逻辑
- 日志轮转
- 敏感信息移除

### 第五阶段: 系统集成 (任务 10-12)

#### [10. 二进制提取器和嵌入系统实现](./10-binary-extractor-and-embed.md)
**时间**: 1-2 小时
**内容**: 实现 eBPF 二进制嵌入和运行时提取
**前置**: 03, 04
**产出**: binary_extractor.rs 和 build.rs

**关键技术**:
- rust-embed
- 临时文件管理
- RAII 资源管理
- 构建脚本

#### [11. CLI 命令行接口实现](./11-cli-implementation.md)
**时间**: 2-3 小时
**内容**: 实现完整的 CLI,整合所有组件
**前置**: 10
**产出**: main.rs 和 cli/ 模块

**关键技术**:
- Clap 参数解析
- 子命令设计
- 组件整合
- 信号处理

#### [12. Web 服务器和前端界面实现](./12-web-server-and-frontend.md)
**时间**: 3-4 小时
**内容**: 实现 Web 服务器和 React 前端界面
**前置**: 11
**产出**: server/ 模块和 frontend/ 目录

**关键技术**:
- Hyper HTTP 服务器
- Next.js/React
- 实时数据展示
- 静态资源服务

### 第六阶段: 测试和发布 (任务 13)

#### [13. 最终集成、测试和发布准备](./13-integration-testing-and-release.md)
**时间**: 2-3 小时
**内容**: 端到端测试,性能优化,文档完善,发布准备
**前置**: 12
**产出**: 可发布的完整系统

**关键任务**:
- 集成测试
- 性能测试
- 文档完善
- CI/CD 配置

## 任务时间估算

| 阶段 | 任务 | 预计时间 | 累计时间 |
|------|------|----------|----------|
| 基础 | 00-02 | 3-6 小时 | 3-6 小时 |
| eBPF | 03-04 | 4-6 小时 | 7-12 小时 |
| 框架核心 | 05-07 | 5-8 小时 | 12-20 小时 |
| 数据处理 | 08-09 | 3-5 小时 | 15-25 小时 |
| 系统集成 | 10-12 | 6-9 小时 | 21-34 小时 |
| 测试发布 | 13 | 2-3 小时 | 23-37 小时 |

**总计**: 约 23-37 小时 (3-5 个工作日)

## 快速导航

### 按技术栈分类

**eBPF/C 编程**
- [02. eBPF 基础](./02-ebpf-basics.md)
- [03. 进程监控](./03-process-monitoring-ebpf.md)
- [04. SSL 监控](./04-ssl-traffic-monitoring-ebpf.md)

**Rust 后端**
- [05. 事件系统](./05-rust-framework-core-events.md)
- [06. Runner 架构](./06-rust-framework-runner-trait.md)
- [07. SSL/Process Runner](./07-ssl-and-process-runners.md)
- [08. HTTP 解析](./08-analyzer-trait-and-http-parser.md)
- [09. 过滤器](./09-filters-and-logger-analyzers.md)
- [10. 二进制嵌入](./10-binary-extractor-and-embed.md)
- [11. CLI 实现](./11-cli-implementation.md)
- [12. Web 服务器](./12-web-server-and-frontend.md#第一部分-web-服务器实现)

**Web 前端**
- [12. 前端界面](./12-web-server-and-frontend.md#第二部分-前端实现)

**测试和部署**
- [13. 集成测试](./13-integration-testing-and-release.md)

### 按难度分类

**入门级** (适合初学者)
- 01. 环境搭建
- 05. 事件系统
- 09. 过滤器和日志
- 10. 二进制嵌入

**中级** (需要一定经验)
- 02. eBPF 基础
- 06. Runner 架构
- 07. SSL/Process Runner
- 11. CLI 实现
- 12. Web 界面

**高级** (需要深入理解)
- 03. 进程监控 eBPF
- 04. SSL 监控 eBPF
- 08. HTTP 解析
- 13. 集成测试

## 学习建议

### 对于初学者

1. **先理解概念**: 仔细阅读 00. 项目分析
2. **不要跳跃**: 严格按照顺序完成任务
3. **多查资料**: 遇到不懂的概念立即查阅
4. **手动实现**: 不要复制现有代码,自己实现
5. **充分测试**: 确保每个任务完全通过测试

### 对于有经验的开发者

1. **快速浏览**: 可以快速浏览前几个任务
2. **关注难点**: 重点学习 eBPF 和异步流处理
3. **深入优化**: 在实现的基础上进行性能优化
4. **扩展功能**: 尝试添加文档中未提到的功能

### 常见学习路径

**路径 1: 全栈开发者**
- 重点: 11-12 (CLI 和 Web)
- 可选择性学习 eBPF 部分

**路径 2: 系统程序员**
- 重点: 02-04 (eBPF)
- 深入理解内核追踪

**路径 3: Rust 开发者**
- 重点: 05-10 (Rust 框架)
- 学习异步和流式处理

## 额外资源

### 推荐阅读

**eBPF**
- [eBPF 官方文档](https://ebpf.io/what-is-ebpf/)
- [BPF Performance Tools (Book)](http://www.brendangregg.com/bpf-performance-tools-book.html)
- [libbpf-bootstrap](https://github.com/libbpf/libbpf-bootstrap)

**Rust**
- [Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

**HTTP**
- [HTTP/1.1 RFC 7230-7235](https://tools.ietf.org/html/rfc7230)
- [Chunked Transfer Encoding](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Transfer-Encoding)

### 相关项目

- [bcc](https://github.com/iovisor/bcc) - eBPF 工具集
- [Falco](https://falco.org/) - 云原生运行时安全
- [Pixie](https://px.dev/) - Kubernetes 可观测性

## 贡献指南

如果发现文档中的问题或有改进建议:

1. **报告问题**: 在 GitHub 上创建 Issue
2. **改进文档**: 提交 Pull Request
3. **分享经验**: 在社区讨论你的实践经验

## 许可证

本文档集遵循项目的开源许可证。

## 总结

通过完成这 14 个任务,你将从零开始构建一个完整的、生产级的 AI Agent 可观测性框架。这不仅是一个代码复现过程,更是一次深入学习系统编程、Rust 开发、eBPF 技术和 Web 开发的旅程。

祝你学习愉快!如有任何问题,欢迎查阅各个任务文档中的"常见问题和排查"部分。

---

**文档版本**: 1.0
**最后更新**: 2025-11-22
**维护者**: AgentSight 项目团队
