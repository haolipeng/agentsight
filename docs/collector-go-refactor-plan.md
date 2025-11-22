# AgentSight Collector: Rust → Go 重构计划

## 一、项目结构设计

```
collector-go/
├── cmd/
│   └── agentsight/
│       └── main.go              # CLI 入口
├── internal/
│   ├── core/
│   │   ├── event.go             # Event 结构体
│   │   └── timestamp.go         # 时间戳转换
│   ├── runner/
│   │   ├── runner.go            # Runner 接口定义
│   │   ├── common.go            # BinaryExecutor 实现
│   │   ├── ssl.go               # SslRunner
│   │   ├── process.go           # ProcessRunner
│   │   ├── system.go            # SystemRunner
│   │   ├── agent.go             # AgentRunner (组合 runner)
│   │   └── fake.go              # FakeRunner (测试用)
│   ├── analyzer/
│   │   ├── analyzer.go          # Analyzer 接口定义
│   │   ├── output.go            # 控制台输出
│   │   ├── file_logger.go       # 文件日志 + 轮转
│   │   ├── sse_processor.go     # SSE 合并
│   │   ├── http_parser.go       # HTTP 解析
│   │   ├── http_filter.go       # HTTP 过滤
│   │   ├── ssl_filter.go        # SSL 过滤
│   │   ├── auth_remover.go      # 敏感头移除
│   │   └── timestamp_norm.go    # 时间戳标准化
│   ├── server/
│   │   ├── server.go            # HTTP 服务器
│   │   └── assets.go            # 前端资源嵌入
│   └── extractor/
│       └── binary.go            # eBPF 二进制提取
├── embed/                       # 嵌入资源
│   ├── binaries/                # eBPF 二进制
│   └── frontend/                # 前端资源
├── go.mod
└── go.sum
```

---

## 二、分阶段实施计划

### 阶段 1: 核心基础设施

**1.1 Event 核心结构**
- 定义 `Event` struct (timestamp, source, pid, comm, data)
- 实现 JSON 序列化/反序列化
- 实现 `datetime()` 方法转换显示时间

**1.2 时间戳工具**
- 读取 `/proc/uptime` 获取启动时间
- 读取 `/proc/stat` 获取 boot time
- 实现 `bootNsToEpochMs()` 转换函数

**1.3 Runner 接口**
```go
type EventStream <-chan Event

type Runner interface {
    Run(ctx context.Context) (EventStream, error)
    AddAnalyzer(analyzer Analyzer) Runner
    Name() string
    ID() string
}
```

**1.4 Analyzer 接口**
```go
type Analyzer interface {
    Process(ctx context.Context, in EventStream) (EventStream, error)
    Name() string
}
```

---

### 阶段 2: Runner 实现

**2.1 BinaryExecutor 通用组件**
- 使用 `exec.CommandContext` 启动子进程
- 逐行读取 stdout 并解析 JSON
- 后台 goroutine 处理 stderr 日志
- 支持优雅关闭

**2.2 SslRunner**
- 嵌入或提取 sslsniff 二进制
- 构建命令行参数 (-p, -u, -c, --handshake, --binary-path)
- 解析 JSON 输出为 Event 流
- Builder 模式配置: `WithArgs()`, `TLSVersion()`

**2.3 ProcessRunner**
- 嵌入或提取 process 二进制
- 构建命令行参数 (-c, -d, -m)
- Builder 模式配置: `WithArgs()`, `PID()`, `MemoryThreshold()`

**2.4 SystemRunner**
- 纯 Go 实现，无外部二进制
- 定时读取 `/proc/[pid]/stat` 和 `/proc/stat`
- 计算 CPU/内存使用率
- Builder 模式: `Interval()`, `PID()`, `Comm()`, `IncludeChildren()`

**2.5 AgentRunner**
- 组合多个 Runner 的事件流
- 使用 `sync.WaitGroup` + channel 合并
- 全局 Analyzer 链处理合并后的流

**2.6 FakeRunner**
- 生成模拟 SSL 请求/响应数据
- 可配置事件数量和延迟

---

### 阶段 3: Analyzer 实现

**3.1 TimestampNormalizer**
- 转换 ns-since-boot → ms-since-epoch
- 通常作为 Analyzer 链第一个

**3.2 OutputAnalyzer**
- JSON 格式化输出到 stdout
- 二进制数据转 hex 字符串

**3.3 FileLogger**
- 追加写入 JSON 到文件
- 支持日志轮转 (按大小，最大文件数)
- 使用 `sync.Mutex` 保证并发安全

**3.4 SSEProcessor**
- 累积 SSE 分片 (event:, data:)
- 按消息 ID 合并
- 30秒超时处理不完整流

**3.5 HTTPParser**
- 解析 SSL 数据为 HTTP 请求/响应
- 检测 HTTP 方法 (GET, POST 等)
- 提取: first-line, headers, body, method, path, status_code
- 配置: `DisableRawData()`

**3.6 SSLFilter / HTTPFilter**
- 表达式过滤: `request.method=POST | response.status_code=200`
- 支持 OR (`|`) 和 AND (`&`) 逻辑
- 原子计数器统计过滤指标

**3.7 AuthHeaderRemover**
- 移除敏感 header: authorization, x-api-key, cookie 等
- 大小写不敏感匹配

---

### 阶段 4: CLI 实现

**4.1 命令行框架**
- 使用 `cobra` 或 `urfave/cli` 库
- 定义子命令: ssl, process, trace, record, system

**4.2 ssl 命令**
- 参数: `--sse-merge`, `--http-parser`, `--ssl-filter`, `--http-filter`, `--server`, `--log-file` 等

**4.3 process 命令**
- 参数: `--quiet`, `--rotate-logs`, `--server`, `--log-file` 等

**4.4 trace 命令 (组合监控)**
- SSL 选项: `--ssl`, `--ssl-uid`, `--ssl-filter`
- Process 选项: `--process`, `-c/--comm`, `-p/--pid`
- System 选项: `--system`, `--system-interval`
- 共享: `--http-filter`, `--server`, `--server-port`

**4.5 record 命令 (优化预设)**
- 简化接口: `-c/--comm` (必需)
- 预配置过滤规则
- 默认启用 system 监控和 server

---

### 阶段 5: Web Server 实现

**5.1 HTTP 服务器**
- 使用 `net/http` 标准库
- 默认端口 7395

**5.2 API 端点**
- `GET /api/events`: 日志文件内容
- `GET /api/assets`: 资源列表
- `GET /*`: 静态资源 (catch-all)

**5.3 前端资源嵌入**
- 使用 `go:embed` 嵌入 `frontend/dist/`
- 正确的 MIME 类型检测
- 缓存头设置

---

### 阶段 6: 二进制嵌入与提取

**6.1 BinaryExtractor**
- 使用 `go:embed` 嵌入 eBPF 二进制
- 提取到临时目录
- 设置可执行权限
- 程序退出时清理

**6.2 资源管理**
- 实现 `Close()` 方法清理临时文件
- 使用 `defer` 确保清理

---

## 三、技术选型

| Rust 组件 | Go 替代方案 |
|-----------|-------------|
| tokio async | goroutine + channel |
| async_trait | interface |
| serde_json | encoding/json |
| clap | cobra / urfave/cli |
| hyper | net/http |
| rust-embed | go:embed |
| Arc<Mutex<T>> | sync.Mutex |
| futures::Stream | <-chan Event |
| tempfile | os.MkdirTemp |

---

## 四、关键迁移策略

1. **异步模型**: Rust async/await → Go goroutine + channel
2. **流处理**: Stream<Item=Event> → `<-chan Event`
3. **错误处理**: Result<T, E> → `(T, error)`
4. **Builder 模式**: 保持 fluent API，使用方法链
5. **资源清理**: Drop trait → `defer` + `Close()` 方法
6. **并发安全**: Arc<Mutex> → sync.Mutex / sync.RWMutex

---

## 五、实施优先级

1. **高优先级**: core/event, runner 接口, BinaryExecutor
2. **中优先级**: SslRunner, ProcessRunner, 基础 Analyzer
3. **低优先级**: Web Server, 高级过滤器, SystemRunner

---

## 六、原 Rust Collector 架构参考

### 数据流架构

```
eBPF Binaries (process, sslsniff)
        ↓
JSON stdout (JSON lines format)
        ↓
BinaryExecutor (subprocess + JSON line reader)
        ↓
JsonStream (serde_json::Value items)
        ↓
Runner (converts to Event stream)
        ↓
[AnalyzerChain]
  - TimestampNormalizer (convert ns→ms)
  - SSLFilter/HTTPFilter (exclude events)
  - SSEProcessor (merge chunks)
  - HTTPParser (parse SSL→HTTP)
  - AuthHeaderRemover (clean sensitive data)
  - FileLogger (persist)
  - OutputAnalyzer (print)
        ↓
Event output (console, file, web server)
```

### AgentRunner 组合模式

```
[SSL Runner + Analyzers] ──┐
                           ├→ select_all() (merge) → [Global Analyzers] → Output
[Process Runner + Analyzers] ──┘
```

### Event 结构

```rust
pub struct Event {
    pub timestamp: u64,        // milliseconds since epoch (after normalization)
    pub source: String,        // "ssl", "process", "system"
    pub pid: u32,
    pub comm: String,
    pub data: serde_json::Value,
}
```

对应 Go 结构:

```go
type Event struct {
    Timestamp uint64          `json:"timestamp"`
    Source    string          `json:"source"`
    PID       uint32          `json:"pid"`
    Comm      string          `json:"comm"`
    Data      json.RawMessage `json:"data"`
}
```
