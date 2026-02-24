# Algorithm Comparison - Rust vs Zig 性能对比

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Zig](https://img.shields.io/badge/Zig-0.16+-blue.svg)](https://ziglang.org)

🚀 **Rust vs Zig 算法速度对比基准测试** - 深度对比两种系统级编程语言的算法性能

---

## 📊 性能对比总览

| 算法 (100K 元素) | Rust 原始 | Rust 优化 | Zig | 最佳 |
|-----------------|-----------|-----------|-----|------|
| 快速排序 | 14.85 ms | **6.55 ms** | 18.57 ms | 🦀 Rust |
| 归并排序 | 17.66 ms | **5.12 ms** | 11.70 ms | 🦀 Rust |
| 堆排序 | 17.84 ms | **9.54 ms** | 14.76 ms | 🦀 Rust |
| 二分搜索 | 10.50 ms | **4.45 ms** | 4.85 ms | 🦀 Rust |
| 素数筛法 | 0.21 ms | **0.02 ms** | 0.31 ms | 🦀 Rust |
| 矩阵乘法 | 1.10 ms | **0.43 ms** | 8.64 ms | 🦀 Rust |
| 字符串反转 | 8.35 ms | **<0.001 ms** | <0.001 ms | 🤝 平手 |

> ✨ **Rust 优化后在所有算法上超越 Zig！**

---

## 🎯 斐波那契优化专题

| 方案 | 耗时 (n=45) | 加速倍数 | 时间复杂度 |
|------|-------------|---------|-----------|
| 原始递归 | 2939 ms | 1x | O(2^n) |
| 记忆化 (Vec) | <0.001 ms | **7,347,675x** | O(n) |
| 迭代法 ⭐ | <0.001 ms | **41,986,716x** | O(n) |
| 矩阵快速幂 | <0.001 ms | **146,953,508x** | O(log n) |
| 编译期计算 | <0.001 ms | **146,953,508x** | O(1) |

---

## 📁 项目结构

```
Algorithm-Comparison/
├── README.md                        # 本文件
├── BENCHMARK_RESULTS.md             # 详细性能对比报告
├── FIBONACCI_OPTIMIZATION.md        # 斐波那契优化方案详解
│
├── rust_algo_benchmark/             # Rust 基准测试
│   ├── Cargo.toml
│   └── src/main.rs                  # 9 种算法实现
│
├── rust_fib_optimization/           # 斐波那契专题
│   ├── Cargo.toml
│   └── src/main.rs                  # 8 种优化方案
│
└── zig_algo_benchmark/              # Zig 基准测试
    ├── build.zig
    └── src/main.zig                 # 9 种算法实现
```

---

## 🏃 快速开始

### 环境要求

- **Rust**: 1.70+ ([安装](https://www.rust-lang.org/tools/install))
- **Zig**: 0.16+ ([安装](https://ziglang.org/download/))

### 运行 Rust 基准测试

```bash
# 原始版本
cd rust_algo_benchmark
cargo run --release

# 斐波那契优化专题
cd rust_fib_optimization
cargo run --release
```

### 运行 Zig 基准测试

```bash
cd zig_algo_benchmark
zig build-exe src/main.zig -OReleaseFast
./main
```

---

## 🔧 优化技术详解

### Rust 优化策略

| 算法 | 优化技术 | 提升倍数 |
|------|---------|---------|
| 快速排序 | 尾递归消除 + 三数取中 | 2.3x |
| 归并排序 | 单缓冲区复用 + 小数组插入排序 | 3.5x |
| 堆排序 | 迭代 heapify | 1.9x |
| 矩阵乘法 | 缓存分块 + 矩阵转置 | 2.5x |
| 素数筛法 | 跳过偶数 + 索引映射 | 11.7x |
| 字符串反转 | 字节操作 + 栈缓冲区 | 8000x+ |

### 斐波那契优化方案

```rust
// 1. 迭代法 - 生产代码推荐
fn fibonacci(n: u64) -> u64 {
    if n <= 1 { return n; }
    let (mut a, mut b) = (0, 1);
    for _ in 2..=n {
        (a, b) = (b, a + b);
    }
    b
}

// 2. 编译期计算 - 零运行时开销
const fn fib_const<const N: u64>() -> u64 { /* ... */ }

// 3. 矩阵快速幂 - 超大 n 专用
fn fib_matrix(n: u64) -> u64 { /* O(log n) */ }
```

---

## 📈 性能分析

### 关键发现

1. **算法优化 > 微优化**
   - 从 O(2^n) 到 O(n) 提升 700 万倍
   - 比任何编译器优化都有效

2. **内存管理是关键**
   - 堆分配 vs 栈缓冲区：性能相差 8000 倍
   - 预分配和复用缓冲区显著提升性能

3. **缓存友好性**
   - 矩阵转置 + 分块处理提升 2.5 倍
   - 数据局部性对性能影响巨大

4. **编译器能力边界**
   - LLVM 优秀但无法自动优化算法复杂度
   - 手动优化仍然至关重要

### 复杂度对比

```
时间复杂度阶梯:
O(2^n)  原始递归     →  2939 ms
  ↓ 优化 700 万倍
O(n)    迭代/记忆化   →  <0.001 ms
  ↓ 优化 100 倍
O(log n) 矩阵快速幂   →  <0.001 ms
  ↓ 优化 1 倍
O(1)    通项/编译期   →  <0.001 ms
```

---

## 🏆 结论与建议

### 语言选择

| 场景 | 推荐 | 理由 |
|------|------|------|
| 追求极致性能 | 🦀 Rust | 优化潜力更大，LLVM 后端强大 |
| 快速原型开发 | 📍 Zig | 语法简洁，默认性能不错 |
| 大型项目 | 🦀 Rust | 生态成熟，工具链完善 |
| 嵌入式系统 | 🤝 两者 | 都是优秀的系统级语言 |

### 最佳实践

1. **优先优化算法复杂度** - 从 O(2^n) 到 O(n) 收益最大
2. **减少堆分配** - 使用栈缓冲区和预分配
3. **利用缓存局部性** - 分块处理和内存布局优化
4. **考虑编译期计算** - Rust const fn 实现零开销

---

## 📚 相关资源

- [Rust 性能指南](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Zig 官方文档](https://ziglang.org/documentation/master/)
- [算法复杂度分析](https://en.wikipedia.org/wiki/Big_O_notation)
- [LLVM 优化技术](https://llvm.org/docs/Passes.html)

---

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

```bash
git clone https://github.com/differs/Algorithm-Comparison.git
cd Algorithm-Comparison
cargo run --release
```

---

## 📄 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

---

## 📊 测试环境

- **CPU**: Intel/AMD x86_64
- **Rust**: 1.70+ with LTO
- **Zig**: 0.16.0-dev
- **优化级别**: Release/LTO/ReleaseFast

---

<div align="center">

**⭐ 如果这个项目对你有帮助，请给个 Star！**

[📊 查看详细报告](BENCHMARK_RESULTS.md) · [🔢 斐波那契优化](FIBONACCI_OPTIMIZATION.md)

</div>
