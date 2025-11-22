# 任务 01: 环境搭建和项目初始化

## 任务目标

搭建完整的开发环境,安装所有必需的依赖,初始化项目结构,确保可以编译和运行基本的 eBPF 程序和 Rust 代码。

## 前置条件

- Linux 操作系统(推荐 Ubuntu 20.04+ 或 Debian 11+)
- Root 权限或 sudo 访问
- 网络连接用于下载依赖

## 涉及的文件

- `Makefile` (eBPF 构建配置)
- `collector/Cargo.toml` (Rust 依赖配置)
- `frontend/package.json` (Node.js 依赖配置)
- `.gitmodules` (Git 子模块配置)

## 实现步骤

### 1. 系统依赖安装

#### 1.1 eBPF 开发工具链
- 安装 clang 和 llvm(版本 10 或更高)
- 安装 libelf 开发库
- 安装 zlib 开发库
- 安装 make 和 gcc 基础工具

#### 1.2 内核开发支持
- 检查内核版本是否支持 eBPF(4.1+)
- 安装 linux-headers 包
- 确认内核已启用 CONFIG_DEBUG_INFO_BTF 选项

#### 1.3 Rust 工具链
- 使用 rustup 安装 Rust 1.82.0 或更高版本
- 设置默认工具链为 stable
- 安装 cargo 构建工具

#### 1.4 Node.js 环境
- 安装 Node.js 18+ 和 npm
- 可以使用 nvm 管理 Node.js 版本

### 2. 项目克隆和子模块初始化

#### 2.1 克隆仓库
- 使用 git clone 获取项目代码
- 进入项目目录

#### 2.2 初始化 Git 子模块
- libbpf 子模块需要初始化和更新
- bpftool 子模块需要初始化和更新
- 确保子模块正确拉取到本地

### 3. 依赖构建

#### 3.1 编译 libbpf
- 进入 libbpf/src 目录
- 执行编译命令构建静态库和共享库
- 安装库文件到系统路径或项目本地路径

#### 3.2 编译 bpftool(可选)
- bpftool 用于调试 eBPF 程序
- 编译并安装到系统路径

### 4. Rust 依赖下载

#### 4.1 下载 Rust crates
- 进入 collector 目录
- 执行 cargo build 下载所有依赖
- 检查依赖是否正确解析

#### 4.2 验证编译
- 尝试编译 collector 项目
- 解决任何版本冲突或缺失依赖

### 5. Node.js 依赖安装

#### 5.1 安装前端依赖
- 进入 frontend 目录
- 执行 npm install 或 npm ci
- 确认依赖安装完成

#### 5.2 验证前端构建
- 运行 npm run build 测试构建
- 确保 TypeScript 编译无错误

### 6. 目录结构验证

#### 6.1 确认关键目录
- bpf/ 目录包含 eBPF 源码
- collector/ 目录包含 Rust 代码
- frontend/ 目录包含 Next.js 代码
- vmlinux/ 目录包含内核头文件
- libbpf/ 和 bpftool/ 子模块已初始化

#### 6.2 权限检查
- 确保当前用户可以执行 sudo 命令
- 检查 /tmp 目录的执行权限(用于临时 eBPF 二进制)

## 验收标准

### 功能验收
1. 所有系统依赖已安装,版本符合要求
2. Git 子模块已正确初始化和更新
3. libbpf 库已编译并可用
4. Rust 项目可以成功编译
5. 前端项目可以成功构建
6. 可以在项目根目录运行 `make --version`、`clang --version`、`cargo --version`、`node --version` 查看版本信息

### 环境验证
1. 内核版本支持 eBPF
2. BTF 信息可用(检查 /sys/kernel/btf/vmlinux 文件存在)
3. 用户具有 root 权限或 CAP_BPF 能力

## 测试方法

### 手动测试

#### 测试 1: 验证 clang 和 LLVM
```bash
clang --version
llc --version
```
**预期结果**: 显示版本 10 或更高,且包含 BPF 目标支持

#### 测试 2: 验证内核 BTF 支持
```bash
ls -l /sys/kernel/btf/vmlinux
```
**预期结果**: 文件存在且可读

#### 测试 3: 验证 libbpf 编译
```bash
ls libbpf/src/libbpf.a
ls libbpf/src/libbpf.so
```
**预期结果**: 库文件存在

#### 测试 4: 验证 Rust 工具链
```bash
cargo --version
rustc --version
```
**预期结果**: 版本 1.82.0 或更高

#### 测试 5: 编译 Rust 项目
```bash
cd collector
cargo build
```
**预期结果**: 编译成功,无错误输出

#### 测试 6: 构建前端
```bash
cd frontend
npm run build
```
**预期结果**: 构建成功,生成 .next 目录

#### 测试 7: 验证 eBPF 编译能力
```bash
cd bpf
make clean
make vmlinux.h  # 如果需要生成 vmlinux.h
```
**预期结果**: 可以生成或使用现有的 vmlinux.h

## 常见问题和排查

### 问题 1: clang 版本过低
**现象**: clang 版本低于 10
**解决**: 从官方源或 LLVM 官网安装最新版本 clang

### 问题 2: BTF 不可用
**现象**: /sys/kernel/btf/vmlinux 不存在
**解决**:
- 升级内核到支持 BTF 的版本(5.2+)
- 或使用 bpftool 生成 vmlinux.h 作为备选方案

### 问题 3: Rust 版本过低
**现象**: cargo build 报告需要更高版本的 Rust
**解决**: 使用 `rustup update` 更新到最新 stable 版本

### 问题 4: libbpf 编译失败
**现象**: 缺少 libelf 或 zlib
**解决**: 安装 libelf-dev 和 zlib1g-dev 包

### 问题 5: 权限不足
**现象**: 无法加载 eBPF 程序
**解决**:
- 使用 sudo 运行
- 或配置 CAP_BPF 和 CAP_SYS_ADMIN 能力

### 问题 6: Git 子模块未初始化
**现象**: libbpf 或 bpftool 目录为空
**解决**: 运行 `git submodule update --init --recursive`

### 问题 7: npm 依赖安装失败
**现象**: 网络问题或版本冲突
**解决**:
- 配置 npm 镜像源
- 删除 node_modules 和 package-lock.json 后重新安装

## 注意事项

1. **内核版本**: eBPF 功能在不同内核版本中差异较大,建议使用 5.4+ 版本
2. **权限管理**: 开发过程中需要频繁使用 sudo,注意安全性
3. **磁盘空间**: 编译依赖和中间文件需要至少 5GB 可用空间
4. **网络连接**: 首次构建会下载大量依赖,确保网络畅通
5. **架构支持**: 确认当前系统架构(x86_64/arm64/riscv64)在 vmlinux/ 目录中有对应的头文件

## 下一步

完成环境搭建后,可以继续进行:
- **任务 02**: eBPF 基础和简单程序开发
- 开始学习 eBPF 编程模型和 CO-RE 技术
