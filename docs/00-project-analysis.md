# AgentSight 项目分析

## 项目概述

AgentSight 是一个专门为 AI Agent 行为监控设计的综合可观测性框架。它通过 SSL/TLS 流量拦截和进程监控,在系统边界层面观察 AI Agent 的行为,而无需修改应用程序代码。

## 核心功能和目标

### 主要功能
1. **SSL/TLS 流量监控**: 使用 eBPF 技术拦截和分析加密流量,监控 AI Agent 与外部服务的交互
2. **进程生命周期追踪**: 监控进程的创建、执行、文件操作等系统级事件
3. **实时流式分析**: 提供基于 Rust 的流式分析框架,支持插件化的数据处理管道
4. **Web 可视化界面**: 内置 Web 服务器和 React 前端,提供时间线视图和实时日志展示
5. **零侵入式监控**: 在系统层面监控,无需修改目标应用程序或添加任何代码

### 项目目标
- 提供低开销(<3%)的系统级监控能力
- 实现 AI Agent 行为的全面可观测性
- 支持多种部署方式(独立二进制、开发模式、生产模式)
- 提供灵活的过滤和分析能力

## 技术栈和版本要求

### 核心技术栈

#### 1. eBPF 层 (C)
- **libbpf** v1.0+: eBPF 程序加载和管理
- **clang** v10+: eBPF 程序编译
- **LLVM**: eBPF 字节码生成
- **libelf**: ELF 文件处理
- **Linux Kernel** 4.1+: 需要 eBPF 支持和 BTF 信息

#### 2. 数据收集和分析层 (Rust)
- **Rust** 1.82.0+, Edition 2021
- **tokio**: 异步运行时和并发处理
- **serde/serde_json**: JSON 序列化和反序列化
- **clap**: 命令行参数解析
- **async-trait**: 异步 trait 支持
- **chrono**: 时间处理和时间戳转换
- **hyper/hyper-util**: HTTP 服务器
- **rust-embed**: 静态资源嵌入
- **env_logger**: 日志记录
- **tempfile**: 临时文件管理
- **uuid**: 唯一标识符生成

#### 3. 前端层 (TypeScript/JavaScript)
- **Next.js** 15.3+: React 框架
- **React** 18+: UI 组件库
- **TypeScript** 5+: 类型安全
- **Tailwind CSS**: 样式框架
- **ESLint**: 代码质量检查
- **PostCSS**: CSS 处理

#### 4. 辅助工具
- **Python** 3.x: 数据分析脚本(可选)
- **Git Submodules**: libbpf 和 bpftool 依赖管理

### 系统要求
- Linux 操作系统(Ubuntu/Debian 推荐)
- Root 权限或 CAP_BPF/CAP_SYS_ADMIN 能力
- 支持的架构: x86_64, ARM64, RISC-V

## 整体架构设计

### 分层架构

```
┌─────────────────────────────────────────────────────────────┐
│                     用户界面层                                │
│  - Web UI (Next.js/React)                                   │
│  - CLI (Rust/Clap)                                          │
└─────────────────────────────────────────────────────────────┘
                            ↑
┌─────────────────────────────────────────────────────────────┐
│                   应用服务层                                  │
│  - Embedded Web Server (Hyper)                              │
│  - Static Asset Serving                                     │
│  - API Endpoints (/api/events, /api/assets)                │
└─────────────────────────────────────────────────────────────┘
                            ↑
┌─────────────────────────────────────────────────────────────┐
│                  流式分析框架层 (Rust)                         │
│  - Runners (SSL, Process, Fake, Combined)                  │
│  - Analyzers (HTTP Parser, Chunk Merger, File Logger)      │
│  - Core Events (标准化事件系统)                               │
│  - Binary Extractor (eBPF 二进制管理)                        │
└─────────────────────────────────────────────────────────────┘
                            ↑
┌─────────────────────────────────────────────────────────────┐
│                  eBPF 数据收集层 (C)                          │
│  - process.bpf.c (进程监控)                                  │
│  - sslsniff.bpf.c (SSL/TLS 流量监控)                         │
│  - JSON 输出格式化                                            │
└─────────────────────────────────────────────────────────────┘
                            ↑
┌─────────────────────────────────────────────────────────────┐
│                    Linux 内核层                               │
│  - eBPF Verifier & JIT Compiler                             │
│  - Kernel Hooks (tracepoints, kprobes, uprobes)            │
│  - BTF (BPF Type Format)                                    │
└─────────────────────────────────────────────────────────────┘
```

### 数据流

```
内核事件 → eBPF 程序 → JSON 输出 → Runner 解析 →
Analyzer 链处理 → 输出(控制台/文件/Web) → 前端展示
```

### 核心设计原则

1. **流式架构**: 实时事件处理,最小化内存使用
2. **插件系统**: 可扩展的 Analyzer 链,支持灵活的数据处理
3. **容错设计**: 优雅处理畸形数据、进程失败和分析器错误
4. **资源管理**: 自动清理临时文件、进程和内核资源
5. **类型安全**: Rust 类型系统确保内存安全
6. **零侵入**: 系统级监控,不修改目标应用

## 主要模块划分和依赖关系

### 模块组织

#### 1. eBPF 程序模块 (`bpf/`)
- **process.bpf.c**: 进程生命周期监控 eBPF 程序
- **process.c**: 进程监控用户态加载器
- **sslsniff.bpf.c**: SSL/TLS 流量监控 eBPF 程序
- **sslsniff.c**: SSL 流量监控用户态加载器
- **test_process_utils.c**: 单元测试
- **Makefile**: 构建配置,支持 AddressSanitizer

#### 2. Rust 收集器模块 (`collector/`)

##### 2.1 框架核心 (`src/framework/`)
- **core/events.rs**: 标准化事件系统
- **binary_extractor.rs**: eBPF 二进制提取和管理

##### 2.2 Runners (`src/framework/runners/`)
- **ssl_runner.rs**: SSL 流量数据收集
- **process_runner.rs**: 进程事件收集
- **fake_runner.rs**: 测试数据生成
- **agent_runner.rs**: Agent 特定收集
- **combined_runner.rs**: 组合多个 Runner

##### 2.3 Analyzers (`src/framework/analyzers/`)
- **http_parser.rs**: HTTP 协议解析
- **chunk_merger.rs**: 数据块合并
- **file_logger.rs**: 文件日志记录
- **output.rs**: 输出处理
- **http_filter.rs**: HTTP 流量过滤
- **ssl_filter.rs**: SSL 流量过滤
- **auth_header_remover.rs**: 敏感信息移除

##### 2.4 服务器 (`src/server/`)
- **mod.rs**: Web 服务器实现
- 静态资源服务
- API 端点实现
- 实时事件广播

##### 2.5 CLI (`src/main.rs`, `src/cli/`)
- 命令行参数解析
- 子命令实现(ssl, process, trace, record)

#### 3. 前端模块 (`frontend/`)
- **pages/**: Next.js 页面组件
- **components/**: React 可复用组件
- **api/**: API 路由处理
- **public/**: 静态资源
- **styles/**: CSS 样式

#### 4. 分析工具模块 (`script/`)
- Python 数据分析脚本
- 时间线生成工具
- 流量分析工具

#### 5. 文档模块 (`docs/`)
- 架构设计文档
- 使用指南
- 案例研究

### 依赖关系

```
前端 (Next.js)
    ↓ (HTTP API)
Web 服务器 (Hyper)
    ↓ (事件流)
Analyzers (可插拔)
    ↓ (事件链)
Runners (数据源)
    ↓ (JSON 输出)
eBPF 程序 (C)
    ↓ (系统调用)
Linux 内核
```

### 关键依赖

1. **eBPF → Rust**: JSON 格式数据传输
2. **Rust Framework → Analyzers**: 异步流式处理
3. **Collectors → Web Server**: tokio broadcast channels
4. **Web Server → Frontend**: HTTP/REST API
5. **Binary Extractor → eBPF Programs**: 临时文件系统

## 项目的难点和重点

### 技术难点

#### 1. eBPF 编程复杂性
- **eBPF Verifier 限制**: 需要满足严格的验证器要求(循环限制、栈大小、指令数量)
- **内核版本兼容性**: 不同内核版本的 eBPF 特性差异
- **BTF 依赖**: 需要内核 BTF 信息才能实现 CO-RE
- **用户态和内核态交互**: ringbuf/perfbuf 的正确使用

#### 2. SSL/TLS 流量拦截
- **多种 SSL 库支持**: OpenSSL, BoringSSL, GnuTLS 等
- **函数符号解析**: 不同版本的库函数名称和签名差异
- **数据完整性**: 确保捕获的数据块顺序正确
- **性能开销**: 保持 <3% 的性能影响

#### 3. 异步流式处理
- **背压处理**: 当下游处理速度慢于上游时的流控
- **错误传播**: 异步链中的错误处理和恢复
- **并发安全**: 多个 Analyzer 同时处理事件的同步
- **资源泄漏**: 确保异步任务正确清理

#### 4. 时间戳同步
- **多数据源时间对齐**: SSL 和 Process 事件的时间戳统一
- **启动时间转换**: 从 boot time 转换为 wall clock time
- **纳秒精度**: 保持高精度时间戳用于关联分析

#### 5. HTTP 协议解析
- **分块传输编码**: chunked transfer encoding 的解析
- **压缩处理**: gzip/deflate 解压缩
- **畸形数据处理**: 处理不完整或格式错误的 HTTP 消息
- **编码问题**: UTF-8 和其他编码的正确处理

#### 6. 二进制嵌入和分发
- **跨平台编译**: 支持多架构的 eBPF 程序编译
- **临时文件安全**: 确保提取的 eBPF 二进制安全执行
- **权限管理**: 临时文件的执行权限处理
- **清理机制**: 进程退出时的资源清理

### 开发重点

#### 1. 核心功能
- eBPF 程序的稳定性和正确性(最高优先级)
- 流式框架的性能和可靠性
- HTTP 解析的准确性和完整性
- 时间戳系统的一致性

#### 2. 用户体验
- CLI 的易用性和清晰的错误提示
- Web UI 的响应性和可读性
- 文档的完整性和准确性
- 安装和部署的简便性

#### 3. 可维护性
- 模块化设计,清晰的接口
- 完善的测试覆盖(单元测试、集成测试)
- 代码注释和文档
- 错误处理和日志记录

#### 4. 性能优化
- eBPF 程序的内存和 CPU 使用
- Rust 异步处理的效率
- 前端渲染的性能
- 大量事件时的处理能力

### 常见陷阱

1. **eBPF Verifier 错误**: R2 min value is negative 等错误需要仔细分析数据流
2. **UTF-8 解码失败**: HTTP body 可能包含非 UTF-8 数据
3. **时间戳不一致**: 混用不同的时间源导致事件顺序错乱
4. **资源泄漏**: 异步任务没有正确取消或临时文件没有清理
5. **权限问题**: eBPF 程序需要 root 权限或特定 capabilities
6. **内核兼容性**: 某些 eBPF 特性在旧内核上不可用

## 开发建议

### 循序渐进的开发路径

1. **环境搭建**: 确保开发环境满足所有依赖要求
2. **eBPF 基础**: 先实现简单的 eBPF 程序,理解 CO-RE 和 libbpf
3. **Rust 框架**: 实现基本的 Runner 和 Analyzer 框架
4. **数据流通**: 打通从 eBPF 到前端的完整数据链路
5. **功能增强**: 添加过滤、解析、可视化等高级功能
6. **优化和测试**: 性能优化、错误处理、测试覆盖

### 测试策略

1. **单元测试**: C 工具函数、Rust 模块、前端组件
2. **集成测试**: 使用 FakeRunner 测试完整的数据流
3. **手动测试**: 实际运行监控真实应用
4. **性能测试**: 测量 CPU 和内存开销
5. **兼容性测试**: 在不同内核版本和架构上测试

### 调试技巧

1. **eBPF 调试**: 使用 bpftool 查看加载的程序和 maps
2. **Rust 调试**: 使用 RUST_LOG 环境变量启用详细日志
3. **内存调试**: 使用 AddressSanitizer 检测内存错误
4. **流量抓包**: 对比 eBPF 捕获和 tcpdump 的结果
5. **时间线分析**: 使用前端可视化辅助理解事件顺序

## 总结

AgentSight 是一个技术栈丰富、架构复杂的可观测性项目。它结合了低级的 eBPF 编程、高性能的 Rust 异步处理、现代化的 Web 前端,提供了一个完整的 AI Agent 监控解决方案。

成功复现此项目需要:
- 扎实的 Linux 系统编程基础
- eBPF 技术的理解和实践经验
- Rust 异步编程能力
- Web 开发技能
- 耐心的调试和问题解决能力

通过分步骤的任务拆解和循序渐进的开发,可以逐步理解和掌握项目的各个方面。
