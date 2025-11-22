# 任务 10: 二进制提取器和嵌入系统实现

## 任务目标

实现 BinaryExtractor 用于管理嵌入的 eBPF 二进制文件,支持在运行时提取到临时目录并自动清理,支持编译时嵌入二进制到 Rust 可执行文件中。

## 前置条件

- **任务 03** 已完成(process eBPF 程序)
- **任务 04** 已完成(sslsniff eBPF 程序)
- 理解 Rust 的 include_bytes! 宏
- 理解临时文件管理

## 涉及的文件

- `collector/src/framework/binary_extractor.rs` (新建)
- `collector/Cargo.toml` (修改 - 添加 rust-embed 依赖)
- `collector/build.rs` (新建 - 构建脚本)

## 实现步骤

### 1. 设计二进制嵌入策略

#### 1.1 开发模式 vs 生产模式
- **开发模式**: 直接使用编译目录中的二进制文件
- **生产模式**: 使用嵌入的二进制文件
- 通过 Cargo feature 切换模式

#### 1.2 嵌入方式选择
- 方案 A: 使用 include_bytes! 宏(简单)
- 方案 B: 使用 rust-embed crate(更强大)
- 推荐方案 B,支持多文件和压缩

#### 1.3 文件组织
```
collector/
  assets/
    ebpf/
      process
      sslsniff
```

### 2. 实现 BinaryExtractor

#### 2.1 结构设计
```rust
pub struct BinaryExtractor {
    temp_dir: PathBuf,
    extracted_files: Vec<PathBuf>,
}
```

#### 2.2 提取逻辑
- 创建临时目录(使用 tempfile crate)
- 从嵌入资源读取二进制数据
- 写入临时文件
- 设置执行权限(chmod +x)
- 记录文件路径用于清理

#### 2.3 路径管理
- extract(name: &str) -> Result<PathBuf>
- 返回提取后的文件绝对路径
- 检查文件是否已提取,避免重复
- 缓存路径

#### 2.4 清理机制
- 实现 Drop trait
- 在 drop 时删除所有提取的文件
- 删除临时目录
- 处理清理失败情况(记录警告)

### 3. 使用 rust-embed 嵌入资源

#### 3.1 添加依赖
```toml
[dependencies]
rust-embed = "8.0"

[features]
embed-ebpf = []
```

#### 3.2 定义嵌入结构
```rust
#[derive(RustEmbed)]
#[folder = "assets/ebpf"]
struct EbpfBinaries;
```

#### 3.3 条件编译
- 使用 #[cfg(feature = "embed-ebpf")]
- 开发时从文件系统读取
- 生产时从嵌入资源读取

### 4. 实现构建脚本

#### 4.1 build.rs 功能
- 编译 eBPF 程序(调用 make)
- 复制二进制到 assets 目录
- 设置重新构建条件

#### 4.2 eBPF 编译
- 检查 bpf 目录是否存在
- 执行 `make -C bpf build`
- 处理编译失败

#### 4.3 文件复制
- 复制 bpf/process 到 assets/ebpf/
- 复制 bpf/sslsniff 到 assets/ebpf/
- 创建目录如果不存在

#### 4.4 依赖追踪
- 使用 println!("cargo:rerun-if-changed=bpf/")
- 确保 eBPF 源码修改时重新构建

### 5. 集成到 Runner

#### 5.1 修改 Runner 使用 BinaryExtractor
- 创建 BinaryExtractor 实例
- 提取所需的二进制
- 使用提取的路径启动进程
- 自动清理(通过 Drop)

#### 5.2 配置选项
- 允许用户指定自定义二进制路径
- 如果指定,使用自定义路径而不是嵌入的
- 用于开发和调试

### 6. 安全考虑

#### 6.1 权限设置
- 临时文件仅对当前用户可读可执行
- 使用 mode 0o700
- 防止其他用户访问

#### 6.2 路径验证
- 验证提取路径合法
- 避免路径遍历攻击
- 检查文件完整性(可选,使用校验和)

#### 6.3 清理保证
- 使用 RAII 确保清理
- 即使程序崩溃也尽可能清理
- 可选:注册信号处理器

### 7. 测试实现

#### 7.1 单元测试
- 测试提取功能
- 测试权限设置
- 测试清理机制
- 测试重复提取

#### 7.2 集成测试
- 测试嵌入和提取流程
- 测试提取的二进制可执行
- 测试多线程环境

## 验收标准

### 功能验收
1. 二进制正确嵌入到可执行文件
2. 运行时可以提取二进制
3. 提取的文件有执行权限
4. 程序退出时自动清理
5. 开发模式可以使用本地文件

### 安全验收
1. 临时文件权限正确
2. 路径安全,无遍历风险
3. 清理机制可靠

## 测试方法

### 单元测试

#### 测试 1: 测试提取功能
```bash
cd collector
cargo test binary_extractor::test_extract
```
**预期结果**: 文件成功提取到临时目录

#### 测试 2: 测试清理
```bash
cd collector
cargo test binary_extractor::test_cleanup
```
**预期结果**: Drop 后临时文件被删除

#### 测试 3: 测试权限
```bash
cd collector
cargo test binary_extractor::test_permissions
```
**预期结果**: 提取的文件可执行

### 集成测试

#### 测试 4: 测试嵌入构建
```bash
cd collector
cargo build --release --features embed-ebpf
```
**预期结果**:
- 编译成功
- 二进制文件包含嵌入资源
- 文件大小增加(包含 eBPF 程序)

#### 测试 5: 测试嵌入模式运行
```bash
cd collector
./target/release/agentsight --help
```
**预期结果**: 程序正常运行,不依赖外部文件

### 手动测试

#### 测试 6: 验证嵌入资源
```bash
cd collector
strings target/release/agentsight | grep "ELF"
```
**预期结果**: 可以看到嵌入的 ELF 文件头

#### 测试 7: 测试清理
```bash
cd collector
ls -la /tmp/agentsight*
cargo run ssl -- -c curl &
PID=$!
ls -la /tmp/agentsight*  # 应该有临时目录
kill $PID
sleep 1
ls -la /tmp/agentsight*  # 应该被清理
```
**预期结果**: 程序退出后临时文件被删除

#### 测试 8: 测试自定义路径
```bash
cd collector
cargo run ssl -- --binary-path /path/to/custom/sslsniff
```
**预期结果**: 使用指定的二进制而不是嵌入的

## 常见问题和排查

### 问题 1: 提取失败
**现象**: "failed to extract binary"
**排查**:
- 检查 /tmp 权限
- 检查磁盘空间
- 查看详细错误信息

### 问题 2: 权限错误
**现象**: "permission denied" 执行提取的文件
**排查**:
- 检查 chmod 调用
- 检查 /tmp 挂载选项(不应有 noexec)
- 手动设置权限测试

### 问题 3: 清理失败
**现象**: 临时文件残留
**排查**:
- 检查 Drop 是否被调用
- 查看是否有文件被占用
- 手动清理测试

### 问题 4: 构建脚本失败
**现象**: cargo build 失败
**排查**:
- 检查 make 命令可用
- 检查 bpf 目录存在
- 查看 build.rs 输出

### 问题 5: 嵌入后文件过大
**现象**: 可执行文件太大
**排查**:
- 考虑压缩嵌入资源
- 或使用其他分发方式

## 注意事项

1. **跨平台**: 考虑不同系统的临时目录路径
2. **并发**: 确保多个实例可以同时运行
3. **清理**: 即使崩溃也要尽可能清理
4. **大小**: 嵌入会增加可执行文件大小
5. **更新**: 更新 eBPF 程序需要重新编译 collector

## 学习要点

1. **Rust 宏**: 理解 include_bytes! 等编译时宏
2. **资源嵌入**: 理解如何在可执行文件中嵌入资源
3. **RAII**: 使用 Drop trait 实现资源管理
4. **构建脚本**: 理解 build.rs 的作用

## 下一步

完成二进制嵌入后,可以继续:
- **任务 11**: 实现 CLI 命令行接口
- 整合所有组件
