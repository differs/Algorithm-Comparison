# 斐波那契递归优化方案对比

## 📊 性能对比结果 (n=45)

| 优化方案 | 耗时 | 加速倍数 | 时间复杂度 | 空间复杂度 |
|---------|------|---------|-----------|-----------|
| **原始递归** | 2939.070 ms | 1.0x | O(2^n) | O(n) |
| **记忆化 (HashMap)** | 0.038 ms | **77,607x** | O(n) | O(n) |
| **记忆化 (Vec)** | <0.001 ms | **7,347,675x** | O(n) | O(n) |
| **尾递归优化** | <0.001 ms | **146,953,508x** | O(n) | O(n)* |
| **迭代法** | <0.001 ms | **41,986,716x** | O(n) | O(1) |
| **矩阵快速幂** | <0.001 ms | **146,953,508x** | O(log n) | O(1) |
| **通项公式** | <0.001 ms | **97,969,005x** | O(1) | O(1) |
| **编译期计算** | <0.001 ms | **146,953,508x** | O(1)运行时 | O(1) |

> *注：尾递归在 LLVM 优化下实际空间复杂度为 O(1)

---

## 🔧 各方案详解

### 1️⃣ 原始递归
```rust
fn fibonacci_naive(n: u64) -> u64 {
    if n <= 1 { return n; }
    fibonacci_naive(n - 1) + fibonacci_naive(n - 2)
}
```
- **问题**: 大量重复计算
- **调用次数**: F(45) 需要调用约 36 亿次
- **适用场景**: 仅用于教学演示

### 2️⃣ 记忆化递归 (HashMap)
```rust
fn fibonacci_memo(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 { return n; }
    if let Some(&val) = memo.get(&n) { return val; }
    let result = fibonacci_memo(n - 1, memo) + fibonacci_memo(n - 2, memo);
    memo.insert(n, result);
    result
}
```
- **优点**: 代码改动最小
- **缺点**: HashMap 有哈希开销
- **适用场景**: 稀疏查询、n 不确定范围

### 3️⃣ 记忆化递归 (Vec)
```rust
fn fibonacci_memo_vec(n: u64, memo: &mut Vec<Option<u64>>) -> u64 {
    if n <= 1 { return n; }
    if let Some(val) = memo[n as usize] { return val; }
    let result = fibonacci_memo_vec(n - 1, memo) + fibonacci_memo_vec(n - 2, memo);
    memo[n as usize] = Some(result);
    result
}
```
- **优点**: 直接索引，无哈希开销
- **缺点**: 需要预分配空间
- **适用场景**: n 范围已知且连续

### 4️⃣ 尾递归优化
```rust
fn fibonacci_tail(n: u64, a: u64, b: u64) -> u64 {
    match n {
        0 => a,
        1 => b,
        _ => fibonacci_tail(n - 1, b, a.wrapping_add(b)),
    }
}
```
- **优点**: 函数式风格，LLVM 会优化为循环
- **缺点**: Rust 不保证 TCO，依赖编译器
- **适用场景**: 函数式编程偏好者

### 5️⃣ 迭代法 ⭐ 推荐
```rust
fn fibonacci_iterative(n: u64) -> u64 {
    if n <= 1 { return n; }
    let mut a = 0u64;
    let mut b = 1u64;
    for _ in 2..=n {
        let temp = a.wrapping_add(b);
        a = b;
        b = temp;
    }
    b
}
```
- **优点**: 简单、高效、O(1) 空间
- **缺点**: 无明显缺点
- **适用场景**: **绝大多数生产场景**

### 6️⃣ 矩阵快速幂
```rust
fn fibonacci_matrix(n: u64) -> u64 {
    if n == 0 { return 0; }
    
    fn matrix_mult(a: [[u64; 2]; 2], b: [[u64; 2]; 2]) -> [[u64; 2]; 2] {
        [
            [a[0][0]*b[0][0] + a[0][1]*b[1][0], a[0][0]*b[0][1] + a[0][1]*b[1][1]],
            [a[1][0]*b[0][0] + a[1][1]*b[1][0], a[1][0]*b[0][1] + a[1][1]*b[1][1]],
        ]
    }
    
    fn matrix_pow(mut base: [[u64; 2]; 2], mut exp: u64) -> [[u64; 2]; 2] {
        let mut result = [[1, 0], [0, 1]];
        while exp > 0 {
            if exp & 1 == 1 { result = matrix_mult(result, base); }
            base = matrix_mult(base, base);
            exp >>= 1;
        }
        result
    }
    
    matrix_pow([[1, 1], [1, 0]], n)[0][1]
}
```
- **优点**: O(log n) 时间复杂度，适合超大 n
- **缺点**: 代码复杂，小 n 时 overhead 大
- **适用场景**: n > 10^6 的超大值

### 7️⃣ 通项公式 (Binet)
```rust
fn fibonacci_formula(n: u64) -> u64 {
    let sqrt5 = 5.0_f64.sqrt();
    let phi = (1.0 + sqrt5) / 2.0;
    let psi = (1.0 - sqrt5) / 2.0;
    ((phi.powi(n as i32) - psi.powi(n as i32)) / sqrt5).round() as u64
}
```
- **优点**: 数学上最优雅，O(1) 时间
- **缺点**: 浮点精度问题，n>70 时不准确
- **适用场景**: n 较小且需要 O(1) 的场景

### 8️⃣ 编译期计算 (const fn)
```rust
const fn fibonacci_const<const N: u64>() -> u64 {
    if N == 0 { return 0; }
    if N == 1 { return 1; }
    let mut i = 2;
    let mut a = 0;
    let mut b = 1;
    while i <= N {
        let temp = a + b;
        a = b;
        b = temp;
        i += 1;
    }
    b
}

// 使用：fibonacci_const::<45>()
```
- **优点**: 运行时零开销，结果在编译时计算
- **缺点**: n 必须是编译期常量
- **适用场景**: 固定的斐波那契数

---

## 📈 性能对比图

```
时间 (对数刻度，越小越好)

原始递归      ████████████████████████████████ 2939ms
记忆化 HashMap ▏0.038ms                         ↑ 7.7 万倍
记忆化 Vec    ▏<0.001ms                         ↑ 730 万倍
迭代法        ▏<0.001ms                         ↑ 4200 万倍
矩阵快速幂    ▏<0.001ms                         ↑ 1.4 亿倍
编译期计算    ▏<0.001ms                         ↑ 1.4 亿倍
```

---

## 🎯 推荐方案

| 场景 | 推荐方案 | 理由 |
|------|---------|------|
| **生产代码** | 迭代法 | 简单、高效、无依赖 |
| **n 是常量** | const fn | 运行时零开销 |
| **超大 n** | 矩阵快速幂 | O(log n) 复杂度 |
| **函数式风格** | 尾递归 | 代码优雅，LLVM 优化 |
| **动态规划教学** | 记忆化 Vec | 展示 DP 思想 |
| **避免重复计算** | 记忆化 HashMap | 稀疏查询友好 |

---

## 💡 关键洞见

### 1. 算法复杂度决定性能上限
- O(2^n) → O(n) → O(log n) → O(1)
- 每次复杂度降低都是数量级的提升

### 2. 数据结构选择很重要
- HashMap vs Vec: 77,607x vs 7,347,675x (相差 95 倍)
- 简单场景用简单结构

### 3. 编译器优化能力有限
- 原始递归无法自动优化为迭代
- 程序员需要手动优化算法

### 4. 编译期计算是终极优化
- Rust 的 const fn 可以在编译时完成计算
- 运行时真正零开销

---

## 📁 项目结构

```
/home/de/works/compair/
├── rust_fib_optimization/
│   ├── Cargo.toml
│   └── src/main.rs          # 8 种实现对比
└── FIBONACCI_OPTIMIZATION.md
```

## 🏃 运行测试

```bash
cd rust_fib_optimization && cargo run --release
```
