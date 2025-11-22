# 任务 12: Web 服务器和前端界面实现

## 任务目标

实现嵌入式 Web 服务器用于提供 API 端点和静态资源服务,实现 Next.js 前端界面展示时间线和事件日志。

## 前置条件

- **任务 11** 已完成(CLI 实现)
- 熟悉 HTTP 服务器开发
- 熟悉 React 和 Next.js
- 理解前后端交互

## 涉及的文件

### 后端
- `collector/src/server/mod.rs` (新建 - Web 服务器)
- `collector/src/server/routes.rs` (新建 - 路由处理)
- `collector/Cargo.toml` (修改 - 添加 hyper 等依赖)

### 前端
- `frontend/package.json` (新建)
- `frontend/pages/index.tsx` (新建 - 首页)
- `frontend/pages/timeline.tsx` (新建 - 时间线页面)
- `frontend/components/` (新建 - React 组件)
- `frontend/next.config.js` (新建 - Next.js 配置)

## 实现步骤

### 第一部分: Web 服务器实现

#### 1. 服务器架构设计

##### 1.1 技术选择
- 使用 hyper 作为 HTTP 服务器
- 使用 tokio 作为异步运行时
- 使用 rust-embed 嵌入前端资源

##### 1.2 API 端点设计
```
GET  /                    # 前端首页(重定向到 /timeline)
GET  /timeline            # 时间线页面
GET  /api/events          # 获取事件日志
GET  /api/assets          # 获取资源列表
GET  /_next/*             # Next.js 静态资源
GET  /static/*            # 其他静态资源
```

##### 1.3 实时通信
- 使用 HTTP 轮询或 Server-Sent Events(SSE)
- 前端定期请求新事件
- 或使用 WebSocket(可选,更复杂)

#### 2. 实现 HTTP 服务器

##### 2.1 服务器结构
```rust
pub struct WebServer {
    port: u16,
    log_file: Option<PathBuf>,
    event_tx: broadcast::Sender<Event>,
}
```

##### 2.2 启动服务器
- 绑定到指定端口
- 创建 hyper service
- 处理请求路由
- 优雅关闭

##### 2.3 请求路由
- 解析请求路径
- 分发到相应处理函数
- 设置响应头
- 返回响应

#### 3. 实现 API 端点

##### 3.1 GET /api/events
- 读取日志文件
- 支持分页(offset, limit)
- 支持实时更新(tail -f 模式)
- 返回 JSON 数组

##### 3.2 GET /api/assets
- 列出可用的日志文件
- 返回文件元数据(大小、修改时间)
- JSON 格式响应

##### 3.3 静态文件服务
- 从嵌入资源读取文件
- 设置正确的 Content-Type
- 支持缓存(Cache-Control 头)
- 处理 404 错误

#### 4. 嵌入前端资源

##### 4.1 构建前端
- 执行 npm run build
- 生成静态文件到 out/ 目录
- 在 build.rs 中自动化

##### 4.2 嵌入资源
```rust
#[derive(RustEmbed)]
#[folder = "../frontend/out"]
struct FrontendAssets;
```

##### 4.3 MIME 类型处理
- .html → text/html
- .js → application/javascript
- .css → text/css
- .json → application/json
- 等等

#### 5. 实时事件广播

##### 5.1 事件通道
- 使用 tokio::sync::broadcast
- Analyzer 链发送事件到通道
- Web 服务器订阅通道

##### 5.2 事件缓冲
- 缓冲最近的 N 个事件
- 新连接时发送历史事件
- 避免内存无限增长

### 第二部分: 前端实现

#### 6. Next.js 项目初始化

##### 6.1 创建项目
```bash
cd frontend
npm init -y
npm install next react react-dom
npm install -D typescript @types/react @types/node
```

##### 6.2 配置 TypeScript
- 创建 tsconfig.json
- 启用严格模式
- 配置路径别名

##### 6.3 配置 Next.js
- next.config.js 设置输出为静态
- 配置 basePath(如果需要)
- 优化构建

#### 7. 实现时间线页面

##### 7.1 页面结构
```
Timeline Page
├── Header (标题、过滤器)
├── Timeline Component
│   ├── Event List
│   │   ├── Event Item (SSL)
│   │   ├── Event Item (Process)
│   │   ├── Event Item (HTTP)
│   │   └── ...
│   └── Details Panel
└── Footer (统计信息)
```

##### 7.2 数据获取
- useEffect 加载初始事件
- setInterval 定期拉取新事件
- 或使用 EventSource(SSE)

##### 7.3 事件展示
- 按时间倒序排列
- 不同事件类型不同样式
- 可展开查看详情
- 语法高亮(JSON)

#### 8. 实现 React 组件

##### 8.1 EventList 组件
- 接收事件数组
- 虚拟滚动(大量事件时)
- 过滤和搜索功能

##### 8.2 EventItem 组件
- 显示事件摘要
- 点击展开详情
- 根据类型显示图标
- 时间戳格式化

##### 8.3 HTTPViewer 组件
- 专门展示 HTTP 请求/响应
- 头部和 Body 分开显示
- 语法高亮
- 折叠/展开

##### 8.4 ProcessTree 组件
- 进程树状图(可选)
- 显示父子关系
- 进程生命周期可视化

#### 9. 实现数据处理

##### 9.1 日志解析
- 逐行解析 JSON
- 处理格式错误
- 提取关键字段

##### 9.2 事件过滤
- 按类型过滤
- 按进程名过滤
- 按时间范围过滤
- 搜索关键词

##### 9.3 数据聚合
- 统计事件数量
- 计算请求延迟
- 生成图表数据

#### 10. 样式实现

##### 10.1 使用 Tailwind CSS
```bash
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p
```

##### 10.2 设计系统
- 颜色方案(亮色/暗色主题)
- 排版规范
- 组件样式

##### 10.3 响应式设计
- 移动端适配
- 平板适配
- 桌面端优化

#### 11. 测试前端

##### 11.1 开发模式测试
```bash
cd frontend
npm run dev
```
访问 http://localhost:3000

##### 11.2 生产构建测试
```bash
cd frontend
npm run build
npm run start
```

##### 11.3 集成测试
```bash
cd collector
sudo cargo run -- record --comm curl --server
```
打开浏览器访问 http://localhost:7395

## 验收标准

### 后端验收
1. Web 服务器成功启动并监听端口
2. API 端点返回正确数据
3. 静态文件正确服务
4. 前端资源成功嵌入

### 前端验收
1. 页面正确加载和渲染
2. 事件正确显示
3. 实时更新工作正常
4. 过滤和搜索功能正常
5. 响应式设计适配各种屏幕

### 用户体验
1. 界面美观直观
2. 加载速度快
3. 交互流畅
4. 错误处理友好

## 测试方法

### 后端测试

#### 测试 1: 启动服务器
```bash
cd collector
sudo cargo run -- record --comm curl --server-port 8080
```
**预期结果**: 服务器启动在 8080 端口

#### 测试 2: 测试 API
```bash
curl http://localhost:8080/api/events
curl http://localhost:8080/api/assets
```
**预期结果**: 返回 JSON 数据

#### 测试 3: 测试静态文件
```bash
curl http://localhost:8080/
curl http://localhost:8080/_next/static/...
```
**预期结果**: 返回 HTML 和资源

### 前端测试

#### 测试 4: 开发模式
```bash
cd frontend
npm run dev
```
访问 http://localhost:3000
**预期结果**: 页面显示,但无实际数据(需要后端)

#### 测试 5: 生产构建
```bash
cd frontend
npm run build
ls -la out/
```
**预期结果**: 生成静态文件

### 集成测试

#### 测试 6: 完整流程
启动监控:
```bash
cd collector
sudo cargo run -- record --comm curl --server
```
在浏览器打开: http://localhost:7395
在另一终端:
```bash
curl https://www.example.com
```
**预期结果**:
- 浏览器显示时间线
- 新事件自动出现
- 事件详情可查看

#### 测试 7: 长时间运行
```bash
cd collector
sudo cargo run -- record --comm python --server &
# 运行 30 分钟
# 生成各种请求
# 检查内存使用和性能
```
**预期结果**:
- 服务器稳定运行
- 内存使用稳定
- 页面持续更新

## 常见问题和排查

### 问题 1: 服务器启动失败
**现象**: "address already in use"
**排查**:
- 检查端口是否被占用
- 使用不同端口
- 杀掉占用端口的进程

### 问题 2: 前端无法加载
**现象**: 404 错误
**排查**:
- 检查前端是否构建
- 检查资源嵌入配置
- 查看服务器日志

### 问题 3: API 无数据
**现象**: /api/events 返回空
**排查**:
- 检查日志文件是否存在
- 检查是否有捕获事件
- 查看文件权限

### 问题 4: 实时更新不工作
**现象**: 新事件不显示
**排查**:
- 检查轮询逻辑
- 检查事件广播
- 查看浏览器控制台错误

### 问题 5: 页面样式错乱
**现象**: CSS 不加载
**排查**:
- 检查 MIME 类型
- 检查资源路径
- 清除浏览器缓存

## 注意事项

1. **CORS**: 开发时可能需要配置 CORS
2. **安全**: 只监听 localhost 避免外部访问
3. **性能**: 大量事件时注意内存和渲染性能
4. **错误处理**: 网络错误、解析错误等
5. **浏览器兼容**: 测试主流浏览器

## 学习要点

1. **Hyper 框架**: Rust 的 HTTP 服务器
2. **Next.js**: 现代 React 框架
3. **实时通信**: HTTP 轮询 vs SSE vs WebSocket
4. **静态资源嵌入**: 单一二进制部署

## 下一步

完成 Web 界面后,可以继续:
- **任务 13**: 最终集成、测试和文档
- 准备发布
