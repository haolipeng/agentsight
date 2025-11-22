# 任务 02: eBPF 基础和简单程序开发

## 任务目标

理解 eBPF 编程模型、CO-RE(Compile Once - Run Everywhere)技术和 libbpf 库的使用方法,编写一个简单的 eBPF 程序用于追踪系统调用,掌握 eBPF 开发的基本流程。

## 前置条件

- **任务 01** 已完成(环境搭建)
- 具备 C 语言编程基础
- 了解 Linux 系统调用和内核基础概念

## 涉及的文件

- `bpf/hello.bpf.c` (新建 - eBPF 内核态程序)
- `bpf/hello.c` (新建 - 用户态加载器)
- `bpf/Makefile` (修改 - 添加新程序构建规则)
- `vmlinux/x86/vmlinux.h` (使用 - 内核类型定义)

## 实现步骤

### 1. 理解 eBPF 编程模型

#### 1.1 eBPF 架构概念
- 理解内核态和用户态的分离
- 理解 eBPF 验证器(Verifier)的作用和限制
- 理解 eBPF Maps 用于内核态和用户态数据交换
- 理解不同类型的 eBPF 程序(tracepoint, kprobe, uprobe, etc.)

#### 1.2 CO-RE 技术
- 理解 BTF(BPF Type Format)的作用
- 理解如何使用 vmlinux.h 获取内核数据结构定义
- 理解 __builtin_preserve_access_index 等 CO-RE 宏
- 理解 bpf_core_read 辅助函数

#### 1.3 libbpf 库
- 理解 libbpf 的加载流程
- 理解 skeleton 机制(自动生成的 .skel.h 文件)
- 理解 ring buffer 和 perf buffer 的区别

### 2. 创建简单的 tracepoint 程序

#### 2.1 选择追踪点
- 选择一个简单的系统调用作为追踪目标(如 execve)
- 使用 tracepoint 而不是 kprobe(更稳定,API 不变)
- 在 /sys/kernel/debug/tracing/events 下查找可用的 tracepoint

#### 2.2 编写 eBPF 内核态程序
- 创建 hello.bpf.c 文件
- 包含必要的头文件(vmlinux.h, bpf/bpf_helpers.h)
- 定义 SEC() 宏指定程序类型和挂载点
- 实现追踪函数,获取系统调用参数
- 使用 bpf_printk 输出调试信息

#### 2.3 定义数据结构
- 定义用于传递给用户态的事件结构体
- 注意结构体对齐和大小限制
- 使用 __attribute__((packed)) 如果需要

#### 2.4 使用 ring buffer 传递数据
- 定义 BPF_MAP_TYPE_RINGBUF 类型的 map
- 使用 bpf_ringbuf_reserve 预留空间
- 使用 bpf_ringbuf_submit 提交数据

### 3. 编写用户态加载器

#### 3.1 生成 skeleton 头文件
- 使用 bpftool gen skeleton 从 .bpf.o 生成 .skel.h
- 理解 skeleton 中包含的结构和函数

#### 3.2 实现加载逻辑
- 创建 hello.c 文件
- 调用 skeleton 的 open 函数
- 调用 skeleton 的 load 函数(执行验证和加载)
- 调用 skeleton 的 attach 函数(挂载到追踪点)

#### 3.3 处理 ring buffer 数据
- 创建 ring buffer manager
- 注册回调函数处理接收到的事件
- 实现事件处理函数,打印事件信息

#### 3.4 主循环和清理
- 实现主循环,持续轮询 ring buffer
- 处理 Ctrl+C 信号,优雅退出
- 调用 skeleton 的 destroy 函数释放资源

### 4. 修改构建系统

#### 4.1 添加到 Makefile
- 在 APPS 变量中添加 hello
- 确保构建规则能够正确编译 eBPF 程序
- 确保 skeleton 生成规则正确

#### 4.2 理解编译流程
- C 源码通过 clang 编译为 BPF 字节码(.o 文件)
- 使用 -target bpf 指定目标架构
- 使用 -g 生成调试信息(BTF)
- bpftool 生成 skeleton 头文件

### 5. 调试和验证

#### 5.1 编译程序
- 运行 make 编译 hello 程序
- 检查是否生成 hello.bpf.o 和 hello 可执行文件
- 检查是否生成 hello.skel.h 文件

#### 5.2 运行和测试
- 使用 sudo 运行编译的程序
- 在另一个终端执行会触发追踪点的命令
- 观察程序输出是否捕获到事件

#### 5.3 使用 bpftool 调试
- 使用 bpftool prog list 查看加载的 eBPF 程序
- 使用 bpftool map list 查看创建的 maps
- 使用 cat /sys/kernel/debug/tracing/trace_pipe 查看 bpf_printk 输出

## 验收标准

### 功能验收
1. hello.bpf.c 编译成功,通过 eBPF 验证器
2. hello 用户态程序成功加载和挂载 eBPF 程序
3. 程序能够捕获目标系统调用事件
4. 事件数据正确通过 ring buffer 传递到用户态
5. 用户态程序能够解析和打印事件信息
6. 程序可以优雅退出并清理资源

### 代码质量
1. 代码结构清晰,有适当的注释
2. 错误处理完善(检查返回值)
3. 没有内存泄漏

## 测试方法

### 手动测试

#### 测试 1: 编译 eBPF 程序
```bash
cd bpf
make hello
```
**预期结果**:
- 编译成功,无警告
- 生成 hello.bpf.o 文件
- 生成 hello.skel.h 文件
- 生成 hello 可执行文件

#### 测试 2: 运行程序
```bash
sudo ./bpf/hello
```
**预期结果**:
- 程序启动,无错误信息
- 打印"正在追踪..."或类似提示信息
- 程序持续运行,不崩溃

#### 测试 3: 触发事件
在另一个终端执行:
```bash
ls /tmp
echo "test"
```
**预期结果**:
- hello 程序输出捕获到的事件
- 事件信息包含进程 ID、命令名称等
- 输出格式清晰可读

#### 测试 4: 使用 bpftool 验证
```bash
sudo bpftool prog list | grep hello
sudo bpftool map list
```
**预期结果**:
- 可以看到加载的 hello 程序
- 可以看到创建的 ring buffer map

#### 测试 5: 查看内核日志
```bash
sudo cat /sys/kernel/debug/tracing/trace_pipe
```
**预期结果**:
- 可以看到 bpf_printk 的输出(如果使用了该函数)

#### 测试 6: 优雅退出
按 Ctrl+C 终止程序
**预期结果**:
- 程序正常退出
- 使用 bpftool prog list 确认程序已卸载

### 单元测试

不需要单元测试,因为这是一个概念验证程序。

## 常见问题和排查

### 问题 1: eBPF 验证器错误
**现象**: "invalid argument" 或 "R1 invalid mem access"
**排查**:
- 检查是否正确使用 bpf_core_read 读取内核数据
- 检查是否访问了未初始化的指针
- 检查数组访问是否有边界检查
- 使用 bpf_printk 添加调试信息

### 问题 2: skeleton 生成失败
**现象**: bpftool gen skeleton 报错
**排查**:
- 确认 bpftool 版本足够新
- 检查 .bpf.o 文件是否正确生成
- 检查是否包含 BTF 信息(使用 -g 编译)

### 问题 3: 加载时权限错误
**现象**: "Operation not permitted"
**排查**:
- 确认使用 sudo 运行
- 检查 /proc/sys/kernel/unprivileged_bpf_disabled 的值
- 确认内核支持 eBPF

### 问题 4: 找不到 tracepoint
**现象**: attach 失败,tracepoint 不存在
**排查**:
- 在 /sys/kernel/debug/tracing/events 下确认 tracepoint 存在
- 检查内核版本,某些 tracepoint 可能不可用
- 使用 bpftool perf list 查看可用的追踪点

### 问题 5: ring buffer 无数据
**现象**: 程序运行但没有输出
**排查**:
- 检查 eBPF 程序是否正确挂载
- 确认触发了追踪点
- 检查 bpf_ringbuf_reserve 是否返回 NULL
- 检查是否调用了 bpf_ringbuf_submit

### 问题 6: 编译时找不到 vmlinux.h
**现象**: "vmlinux.h: No such file or directory"
**排查**:
- 确认 vmlinux/ 目录中有对应架构的 vmlinux.h
- 检查 Makefile 中的 include 路径
- 或使用 bpftool btf dump 生成 vmlinux.h

## 学习要点

1. **eBPF 限制**: 理解 eBPF 验证器的限制(无循环、栈大小、辅助函数使用)
2. **CO-RE 原理**: 理解如何实现一次编译到处运行
3. **数据传递**: 掌握 ring buffer 的使用,理解与 perf buffer 的区别
4. **调试技巧**: 学会使用 bpftool 和 trace_pipe 调试 eBPF 程序
5. **性能考虑**: 理解 eBPF 程序对系统性能的影响

## 参考资料

- libbpf-bootstrap 项目示例
- Linux 内核文档: Documentation/bpf/
- BPF CO-RE reference guide
- bpftool 使用手册

## 下一步

完成 eBPF 基础学习后,可以继续:
- **任务 03**: 实现进程监控 eBPF 程序
- 学习更复杂的追踪场景和数据结构
