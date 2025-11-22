# 任务 09: 过滤器和文件日志 Analyzer 实现

## 任务目标

实现各种过滤器 Analyzer(SSL 过滤器、HTTP 过滤器、敏感信息移除器)和文件日志 Analyzer,支持表达式过滤和日志轮转。

## 前置条件

- **任务 08** 已完成(Analyzer trait 和 HTTP 解析器)

## 涉及的文件

- `collector/src/framework/analyzers/ssl_filter.rs` (新建)
- `collector/src/framework/analyzers/http_filter.rs` (新建)
- `collector/src/framework/analyzers/auth_header_remover.rs` (新建)
- `collector/src/framework/analyzers/file_logger.rs` (新建)
- `collector/src/framework/analyzers/output.rs` (新建)

## 实现步骤

### 1. 实现 SSL Filter Analyzer

#### 1.1 过滤表达式设计
- 支持字段匹配: data.type=binary
- 支持函数匹配: function=SSL_write
- 支持延迟范围: latency>1000
- 支持逻辑运算: AND/OR
- 支持转义字符

#### 1.2 表达式解析
- 解析过滤字符串为 AST
- 支持 key=value 格式
- 支持比较运算符(=, !=, >, <, >=, <=)
- 处理转义和引号

#### 1.3 过滤逻辑
- 检查事件是否为 SSL 事件
- 根据表达式匹配事件字段
- 返回匹配的事件,过滤不匹配的
- 记录过滤统计(通过/拒绝数量)

### 2. 实现 HTTP Filter Analyzer

#### 2.1 HTTP 特定过滤
- request.method: 请求方法(GET, POST 等)
- request.path: 请求路径
- response.status: 响应状态码
- headers.*: 任意头字段

#### 2.2 模式匹配
- 精确匹配
- 前缀匹配
- 正则表达式匹配(可选)
- 通配符匹配

#### 2.3 复合条件
- 支持多个条件组合
- AND/OR 逻辑
- 括号分组(可选,高级功能)

### 3. 实现 Auth Header Remover Analyzer

#### 3.1 敏感信息识别
- Authorization 头
- Cookie 头
- API Key 相关头(X-API-Key 等)
- Token 相关头

#### 3.2 移除逻辑
- 检测 HTTP 事件
- 扫描头字段
- 移除或脱敏敏感字段
- 保留其他信息

#### 3.3 配置选项
- 可配置要移除的头字段列表
- 脱敏选项(完全移除 vs 部分隐藏)
- 白名单机制

### 4. 实现 File Logger Analyzer

#### 4.1 文件写入
- 异步写入文件
- 使用缓冲 IO 提高性能
- 每个事件一行 JSON
- 确保数据持久化

#### 4.2 日志轮转
- 支持按大小轮转
- 支持按时间轮转
- 自动创建新文件
- 压缩旧日志(可选)

#### 4.3 错误处理
- 磁盘空间不足
- 权限错误
- IO 错误恢复

#### 4.4 性能优化
- 批量写入减少系统调用
- 异步 flush
- 缓冲区大小可配置

### 5. 实现 Output Analyzer

#### 5.1 标准输出
- 格式化事件为 JSON
- 输出到 stdout
- 支持彩色输出(可选)
- 支持不同详细级别

#### 5.2 输出格式
- JSON Lines 格式(一行一个 JSON)
- 紧凑格式 vs 美化格式
- 时间戳格式化

#### 5.3 性能考虑
- 异步输出避免阻塞
- 缓冲输出
- 批量 flush

### 6. 实现全局指标

#### 6.1 统计计数器
- 使用 AtomicU64 实现线程安全计数
- 过滤器通过/拒绝数量
- 处理的事件总数
- 错误数量

#### 6.2 指标收集
- 每个 Analyzer 维护自己的指标
- 定期报告指标
- 提供查询 API

### 7. 测试实现

#### 7.1 单元测试
- 测试表达式解析
- 测试过滤逻辑
- 测试敏感信息移除
- 测试文件写入

#### 7.2 集成测试
- 测试完整的过滤流程
- 测试日志轮转
- 测试多个 Analyzer 组合

## 验收标准

### 功能验收
1. SSL 过滤器正确过滤事件
2. HTTP 过滤器支持多种条件
3. 敏感信息正确移除
4. 文件日志正常写入
5. 日志轮转工作正常
6. 输出格式正确

### 性能验收
1. 过滤不影响整体性能
2. 文件写入不阻塞处理
3. 内存使用合理

## 测试方法

### 单元测试

#### 测试 1: 测试 SSL 过滤表达式
```bash
cd collector
cargo test ssl_filter::test_expression
```
**预期结果**: 表达式正确解析和匹配

#### 测试 2: 测试 HTTP 过滤
```bash
cd collector
cargo test http_filter::test_filter
```
**预期结果**: 各种 HTTP 条件正确过滤

#### 测试 3: 测试敏感信息移除
```bash
cd collector
cargo test auth_remover::test_remove
```
**预期结果**: Authorization 等头被移除

#### 测试 4: 测试文件日志
```bash
cd collector
cargo test file_logger::test_write
```
**预期结果**: 事件正确写入文件

#### 测试 5: 测试日志轮转
```bash
cd collector
cargo test file_logger::test_rotation
```
**预期结果**: 文件大小达到限制时自动轮转

### 集成测试

#### 测试 6: 完整 Analyzer 链
```bash
cd collector
cargo test test_analyzer_chain -- --nocapture
```
**预期结果**:
- 事件经过多个 Analyzer
- 过滤和日志正常
- 最终输出正确

### 手动测试

#### 测试 7: 实际过滤
运行带过滤的监控:
```bash
cd collector
sudo cargo run -- ssl --ssl-filter "function=SSL_write" > filtered.log
```
发起 HTTPS 请求:
```bash
curl https://www.example.com
```
**预期结果**: 只记录 SSL_write 事件

#### 测试 8: HTTP 过滤
```bash
cd collector
sudo cargo run -- ssl --http-filter "request.method=POST" > posts.log
```
**预期结果**: 只记录 POST 请求

#### 测试 9: 日志轮转测试
```bash
cd collector
sudo cargo run -- ssl --log-file test.log --log-max-size 1048576
# 生成大量流量
for i in {1..1000}; do curl -s https://www.example.com > /dev/null; done
ls -lh test.log*
```
**预期结果**: 生成多个日志文件(test.log, test.log.1, test.log.2 等)

## 常见问题和排查

### 问题 1: 过滤表达式不生效
**现象**: 所有事件都通过
**排查**:
- 检查表达式语法
- 验证字段名称
- 添加调试日志

### 问题 2: 文件写入失败
**现象**: IO 错误
**排查**:
- 检查磁盘空间
- 检查文件权限
- 检查路径是否存在

### 问题 3: 日志轮转不工作
**现象**: 文件超过大小限制
**排查**:
- 检查配置参数
- 验证轮转逻辑
- 查看错误日志

### 问题 4: 性能下降
**现象**: 添加 Analyzer 后变慢
**排查**:
- 检查是否有阻塞操作
- 增大缓冲区
- 优化过滤表达式

## 注意事项

1. **异步 IO**: 文件写入使用异步避免阻塞
2. **错误恢复**: IO 错误不应导致程序崩溃
3. **资源限制**: 限制日志文件大小和数量
4. **安全**: 敏感信息必须完全移除
5. **性能**: 过滤器是热点路径

## 下一步

完成过滤和日志后,可以继续:
- **任务 10**: 实现二进制提取和嵌入系统
