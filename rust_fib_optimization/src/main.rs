use std::collections::HashMap;
use std::time::Instant;

const FIB_N: u64 = 45;

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║     斐波那契递归优化方案对比           ║");
    println!("╚════════════════════════════════════════╝\n");

    let mut results = Vec::new();

    // 1. 原始递归
    results.push(benchmark("原始递归", || fibonacci_naive(FIB_N)));

    // 2. 记忆化递归 (HashMap)
    results.push(benchmark("记忆化 (HashMap)", || {
        let mut memo = HashMap::new();
        fibonacci_memo(FIB_N, &mut memo)
    }));

    // 3. 记忆化递归 (Vec)
    results.push(benchmark("记忆化 (Vec)", || {
        let mut memo = vec![None; (FIB_N + 1) as usize];
        fibonacci_memo_vec(FIB_N, &mut memo)
    }));

    // 4. 尾递归 (带累加器)
    results.push(benchmark("尾递归优化", || fibonacci_tail(FIB_N, 0, 1)));

    // 5. 迭代法
    results.push(benchmark("迭代法", || fibonacci_iterative(FIB_N)));

    // 6. 矩阵快速幂
    results.push(benchmark("矩阵快速幂", || fibonacci_matrix(FIB_N)));

    // 7. 通项公式 (Binet 公式)
    results.push(benchmark("通项公式", || fibonacci_formula(FIB_N)));

    // 8. 编译期计算 (const fn)
    results.push(benchmark("编译期计算", || fibonacci_const::<FIB_N>()));

    // 打印结果
    println!("\n┌─────────────────────────────────────┬──────────────┬──────────┐");
    println!("│ 优化方案                            │ 耗时         │ 加速倍数 │");
    println!("├─────────────────────────────────────┼──────────────┼──────────┤");

    let baseline = results[0].1;
    for (name, duration, _) in &results {
        let speedup = baseline.as_secs_f64() / duration.as_secs_f64();
        println!(
            "│ {:<35} │ {:>10.3} ms │ {:>7.1}x │",
            name,
            duration.as_secs_f64() * 1000.0,
            speedup
        );
    }
    println!("└─────────────────────────────────────┴──────────────┴──────────┘");

    // 验证结果正确性
    println!("\n验证结果 (n={}):", FIB_N);
    for (name, _, result) in &results {
        println!("  {}: {}", name, result);
    }
}

fn benchmark<F, T>(name: &str, mut f: F) -> (String, std::time::Duration, T)
where
    F: FnMut() -> T,
    T: Copy + std::fmt::Display,
{
    let start = Instant::now();
    let result = std::hint::black_box(f());
    let duration = start.elapsed();
    (name.to_string(), duration, result)
}

// ========== 1. 原始递归 (指数时间 O(2^n)) ==========
fn fibonacci_naive(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fibonacci_naive(n - 1) + fibonacci_naive(n - 2)
}

// ========== 2. 记忆化递归 - HashMap (O(n)) ==========
fn fibonacci_memo(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 {
        return n;
    }
    if let Some(&val) = memo.get(&n) {
        return val;
    }
    let result = fibonacci_memo(n - 1, memo) + fibonacci_memo(n - 2, memo);
    memo.insert(n, result);
    result
}

// ========== 3. 记忆化递归 - Vec (O(n)) ==========
fn fibonacci_memo_vec(n: u64, memo: &mut Vec<Option<u64>>) -> u64 {
    if n <= 1 {
        return n;
    }
    if let Some(val) = memo[n as usize] {
        return val;
    }
    let result = fibonacci_memo_vec(n - 1, memo) + fibonacci_memo_vec(n - 2, memo);
    memo[n as usize] = Some(result);
    result
}

// ========== 4. 尾递归优化 (O(n)) ==========
/// 尾递归形式，虽然 Rust 不保证 TCO，但 LLVM 通常会优化
fn fibonacci_tail(n: u64, a: u64, b: u64) -> u64 {
    match n {
        0 => a,
        1 => b,
        _ => fibonacci_tail(n - 1, b, a.wrapping_add(b)),
    }
}

// ========== 5. 迭代法 (O(n)) ==========
fn fibonacci_iterative(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let mut a = 0u64;
    let mut b = 1u64;
    for _ in 2..=n {
        let temp = a.wrapping_add(b);
        a = b;
        b = temp;
    }
    b
}

// ========== 6. 矩阵快速幂 (O(log n)) ==========
/// 使用矩阵 [[1,1],[1,0]]^n 的性质
fn fibonacci_matrix(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    fn matrix_mult(a: [[u64; 2]; 2], b: [[u64; 2]; 2]) -> [[u64; 2]; 2] {
        [
            [
                a[0][0]
                    .wrapping_mul(b[0][0])
                    .wrapping_add(a[0][1].wrapping_mul(b[1][0])),
                a[0][0]
                    .wrapping_mul(b[0][1])
                    .wrapping_add(a[0][1].wrapping_mul(b[1][1])),
            ],
            [
                a[1][0]
                    .wrapping_mul(b[0][0])
                    .wrapping_add(a[1][1].wrapping_mul(b[1][0])),
                a[1][0]
                    .wrapping_mul(b[0][1])
                    .wrapping_add(a[1][1].wrapping_mul(b[1][1])),
            ],
        ]
    }

    fn matrix_pow(mut base: [[u64; 2]; 2], mut exp: u64) -> [[u64; 2]; 2] {
        let mut result = [[1, 0], [0, 1]]; // 单位矩阵
        while exp > 0 {
            if exp & 1 == 1 {
                result = matrix_mult(result, base);
            }
            base = matrix_mult(base, base);
            exp >>= 1;
        }
        result
    }

    let base = [[1, 1], [1, 0]];
    let result = matrix_pow(base, n);
    result[0][1]
}

// ========== 7. 通项公式 (Binet 公式) O(1) ==========
/// F(n) = (φ^n - ψ^n) / √5
/// φ = (1 + √5) / 2, ψ = (1 - √5) / 2
fn fibonacci_formula(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    let sqrt5 = 5.0_f64.sqrt();
    let phi = (1.0 + sqrt5) / 2.0;
    let psi = (1.0 - sqrt5) / 2.0;

    let result = (phi.powi(n as i32) - psi.powi(n as i32)) / sqrt5;
    result.round() as u64
}

// ========== 8. 编译期计算 (const fn) ==========
/// 在编译时计算，运行时零开销
const fn fibonacci_const<const N: u64>() -> u64 {
    if N == 0 {
        return 0;
    }
    if N == 1 {
        return 1;
    }

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
