# 任务 05: Rust 框架核心 - 事件系统实现

## 任务目标

设计和实现 AgentSight 的核心事件系统,定义标准化的事件结构,实现时间戳处理和转换,以及 JSON 序列化和反序列化功能。

## 前置条件

- **任务 01** 已完成(环境搭建)
- 熟悉 Rust 语言基础
- 理解 serde 序列化框架
- 理解时间戳和时区概念

## 涉及的文件

- `collector/Cargo.toml` (修改 - 添加依赖)
- `collector/src/framework/core/events.rs` (新建 - 核心事件定义)
- `collector/src/framework/core/mod.rs` (新建 - 模块声明)
- `collector/src/framework/mod.rs` (新建 - 框架模块根)

## 实现步骤

### 1. 设计事件架构

#### 1.1 事件统一模型
- 所有事件使用统一的 Event 结构
- 支持多种事件来源(SSL, Process, System)
- 包含丰富的元数据和类型安全的 payload

#### 1.2 事件字段设计
- **timestamp**: 时间戳(纳秒精度,从启动开始)
- **event_type**: 事件类型标识
- **source**: 事件来源(ssl, process, system)
- **metadata**: 通用元数据(进程信息等)
- **payload**: 类型化的事件数据(JSON Value)

#### 1.3 类型安全考虑
- 使用 Rust 枚举表示事件类型
- 使用 serde_json::Value 存储动态 payload
- 提供类型安全的访问方法

### 2. 实现事件结构

#### 2.1 定义 Event 结构体
- 使用 derive 宏实现 Clone, Debug, Serialize, Deserialize
- 所有字段使用合适的类型
- 添加文档注释说明每个字段

#### 2.2 定义事件类型枚举
- SSLWrite, SSLRead(SSL 流量)
- ProcessFork, ProcessExec, ProcessExit(进程事件)
- SystemInfo(系统信息)
- HTTPRequest, HTTPResponse(HTTP 解析后的事件)

#### 2.3 定义元数据结构
- 进程 ID、名称、命令行
- 用户 ID、组 ID
- 可选字段使用 Option 类型

### 3. 实现时间戳处理

#### 3.1 获取系统启动时间
- 从 /proc/stat 读取 btime 字段
- 解析为 Unix epoch 秒数
- 缓存结果避免重复读取
- 处理读取失败的情况

#### 3.2 时间戳转换函数
- 将 boot time 纳秒转换为 epoch 纳秒
- 实现为 Event 的方法
- 使用 chrono 库处理日期时间

#### 3.3 时间格式化
- 提供 datetime() 方法返回 DateTime 对象
- 提供 formatted_datetime() 方法返回可读字符串
- 支持不同的时区和格式

### 4. 实现 JSON 序列化

#### 4.1 Serde 集成
- 使用 #[derive(Serialize, Deserialize)]
- 处理字段重命名(serde rename)
- 处理 Option 字段(skip_serializing_if)

#### 4.2 自定义序列化
- 对于特殊字段(如二进制数据)提供自定义序列化
- 使用 serde_json::to_string 序列化为 JSON 字符串
- 使用 serde_json::to_writer 直接写入流

#### 4.3 反序列化
- 从 JSON 字符串反序列化为 Event
- 处理格式错误和缺失字段
- 提供友好的错误信息

### 5. 实现辅助方法

#### 5.1 事件创建方法
- 提供 new() 构造函数
- 提供便捷方法创建特定类型事件
- 自动填充常见字段(如时间戳)

#### 5.2 事件查询方法
- 获取事件类型
- 获取 payload 中的特定字段
- 类型安全的 payload 访问

#### 5.3 事件转换方法
- 克隆事件
- 修改事件字段
- 合并事件

### 6. 添加依赖到 Cargo.toml

#### 6.1 核心依赖
- serde: 序列化框架
- serde_json: JSON 支持
- chrono: 时间处理
- thiserror: 错误处理

#### 6.2 可选依赖
- log: 日志记录
- anyhow: 错误处理辅助

### 7. 实现单元测试

#### 7.1 测试事件创建
- 测试 Event::new() 方法
- 验证字段正确设置

#### 7.2 测试序列化
- 创建事件并序列化为 JSON
- 验证 JSON 格式正确
- 验证所有字段存在

#### 7.3 测试反序列化
- 从 JSON 字符串创建事件
- 验证字段正确解析
- 测试错误情况(格式错误的 JSON)

#### 7.4 测试时间戳转换
- 模拟启动时间
- 测试转换逻辑
- 验证结果正确

## 验收标准

### 功能验收
1. Event 结构定义完整,包含所有必要字段
2. 事件类型枚举覆盖所有事件类型
3. 时间戳转换正确,精度保持
4. JSON 序列化和反序列化正常工作
5. 辅助方法功能正确

### 代码质量
1. 所有公共 API 有文档注释
2. 代码通过 clippy 检查
3. 代码格式符合 rustfmt 标准
4. 无 unsafe 代码(除非必要)

### 测试覆盖
1. 单元测试覆盖核心功能
2. 所有测试通过
3. 测试覆盖正常和异常情况

## 测试方法

### 单元测试

#### 测试 1: 运行所有测试
```bash
cd collector
cargo test framework::core::events
```
**预期结果**:
- 所有测试通过
- 无警告或错误

#### 测试 2: 测试覆盖率
```bash
cd collector
cargo tarpaulin --out Html
```
**预期结果**: 测试覆盖率 > 80%

### 手动测试

#### 测试 3: 编译检查
```bash
cd collector
cargo build
```
**预期结果**: 编译成功,无警告

#### 测试 4: 文档生成
```bash
cd collector
cargo doc --no-deps --open
```
**预期结果**:
- 文档生成成功
- 所有公共 API 有文档
- 示例代码可运行

#### 测试 5: Clippy 检查
```bash
cd collector
cargo clippy -- -D warnings
```
**预期结果**: 无警告

#### 测试 6: 格式检查
```bash
cd collector
cargo fmt -- --check
```
**预期结果**: 代码格式正确

### 集成测试

#### 测试 7: 创建和序列化事件
创建测试文件验证事件系统:
```rust
// 在 tests/event_test.rs 中
use agentsight::framework::core::events::Event;

#[test]
fn test_event_roundtrip() {
    // 创建事件
    // 序列化
    // 反序列化
    // 验证相等
}
```
**预期结果**: 测试通过

## 常见问题和排查

### 问题 1: 序列化失败
**现象**: serde 报错
**排查**:
- 检查所有字段都实现了 Serialize
- 检查是否有递归引用
- 使用 #[serde(skip)] 跳过不需要序列化的字段

### 问题 2: 时间戳不准确
**现象**: 转换后的时间不对
**排查**:
- 检查 /proc/stat 读取是否正确
- 检查纳秒和秒的单位转换
- 验证时区设置

### 问题 3: 编译错误
**现象**: "trait bounds are not satisfied"
**排查**:
- 检查依赖版本
- 确保 derive 宏正确使用
- 查看详细错误信息

### 问题 4: 测试失败
**现象**: 某些测试不通过
**排查**:
- 使用 cargo test -- --nocapture 查看详细输出
- 检查测试逻辑
- 验证测试数据

## 注意事项

1. **时间戳精度**: 保持纳秒精度,避免精度损失
2. **线程安全**: Event 应该是线程安全的(实现 Send + Sync)
3. **性能**: 序列化性能很重要,避免不必要的克隆
4. **向后兼容**: 考虑未来添加字段时的兼容性
5. **错误处理**: 所有可能失败的操作都要返回 Result

## 学习要点

1. **Serde 框架**: 理解 Rust 中的序列化机制
2. **类型设计**: 如何设计类型安全又灵活的 API
3. **时间处理**: chrono 库的使用
4. **测试驱动**: 先写测试,再写实现

## 下一步

完成事件系统后,可以继续:
- **任务 06**: 实现 Runner trait 和基础架构
- 开始构建数据收集层
