use std::time::Instant;

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║     圆周率 π 计算方案对比              ║");
    println!("╚════════════════════════════════════════╝\n");

    let mut results = Vec::new();

    // 1. 莱布尼茨公式 (1000 万次迭代)
    results.push(benchmark("莱布尼茨公式 (10M)", || {
        leibniz_pi(10_000_000)
    }));

    // 2. 改进的莱布尼茨 (100 万次)
    results.push(benchmark("改进莱布尼茨 (1M)", || {
        improved_leibniz_pi(1_000_000)
    }));

    // 3. 马青公式 (100 次)
    results.push(benchmark("马青公式 (100)", || machin_pi(100)));

    // 4. 蒙特卡洛方法 (1000 万次)
    results.push(benchmark("蒙特卡洛 (10M)", || {
        monte_carlo_pi(10_000_000)
    }));

    // 5. Nilakantha 级数 (1000 万次)
    results.push(benchmark("Nilakantha (10M)", || nilakantha_pi(10_000_000)));

    // 6. BBP 公式 (计算第 10-15 位)
    results.push(benchmark("BBP 公式 (15 位)", || bbp_pi(15)));

    // 7. 高斯 - 勒让德算法 (5 次迭代)
    results.push(benchmark("高斯 - 勒让德 (5 次)", || {
        gauss_legendre_pi(5)
    }));

    // 8. Chudnovsky 算法 (5 次迭代)
    results.push(benchmark("Chudnovsky (5 次)", || chudnovsky_pi(5)));

    // 打印结果
    println!("\n┌─────────────────────────────────────┬──────────────┬──────────────┐");
    println!("│ 计算方案                            │ 耗时         │ π 值         │");
    println!("├─────────────────────────────────────┼──────────────┼──────────────┤");

    for (name, duration, pi_value, accuracy) in &results {
        println!(
            "│ {:<35} │ {:>10.3} ms │ {:.10} {} │",
            name,
            duration.as_secs_f64() * 1000.0,
            pi_value,
            accuracy
        );
    }
    println!("└─────────────────────────────────────┴──────────────┴──────────────┘");

    println!("\n参考值: π = 3.14159265358979323846...");
}

fn benchmark<F>(name: &str, mut f: F) -> (String, std::time::Duration, f64, &'static str)
where
    F: FnMut() -> f64,
{
    let start = Instant::now();
    let pi_value = std::hint::black_box(f());
    let duration = start.elapsed();

    let accuracy = if (pi_value - std::f64::consts::PI).abs() < 1e-10 {
        "✓✓✓"
    } else if (pi_value - std::f64::consts::PI).abs() < 1e-6 {
        "✓✓"
    } else if (pi_value - std::f64::consts::PI).abs() < 1e-3 {
        "✓"
    } else {
        "✗"
    };

    (name.to_string(), duration, pi_value, accuracy)
}

// ========== 1. 莱布尼茨公式 ==========
/// π/4 = 1 - 1/3 + 1/5 - 1/7 + 1/9 - ...
/// 收敛极慢，需要数十亿次迭代才能获得高精度
fn leibniz_pi(iterations: u64) -> f64 {
    let mut sum = 0.0;
    for i in 0..iterations {
        let term = if i % 2 == 0 { 1.0 } else { -1.0 };
        sum += term / (2.0 * i as f64 + 1.0);
    }
    sum * 4.0
}

// ========== 2. 改进的莱布尼茨公式 ==========
/// 使用欧拉变换加速收敛
/// π/4 = Σ (-1)^n / (2n+1)
fn improved_leibniz_pi(iterations: u64) -> f64 {
    let mut sum = 0.0;
    let mut term = 1.0;
    let mut denominator = 1.0;

    for i in 0..iterations {
        sum += term / denominator;
        term = -term;
        denominator += 2.0;

        // 每 100 项使用一次加速
        if i > 0 && i % 100 == 0 {
            // Richardson 外推
            let prev = sum - term / denominator;
            sum = sum + (sum - prev) / 3.0;
        }
    }
    sum * 4.0
}

// ========== 3. 马青公式 ==========
/// π/4 = 4*arctan(1/5) - arctan(1/239)
/// 收敛速度快，历史上用于计算π的位数记录
fn machin_pi(iterations: u64) -> f64 {
    fn arctan(x: f64, n: u64) -> f64 {
        let x2 = x * x;
        let mut sum = 0.0;
        let mut term = x;
        let mut divisor = 1.0;

        for _i in 0..n {
            sum += term / divisor;
            term *= -x2;
            divisor += 2.0;
        }
        sum
    }

    4.0 * (4.0 * arctan(1.0 / 5.0, iterations) - arctan(1.0 / 239.0, iterations))
}

// ========== 4. 蒙特卡洛方法 ==========
/// 在单位正方形内随机投点，计算落在 1/4 圆内的比例
/// 收敛慢，主要用于演示
fn monte_carlo_pi(samples: u64) -> f64 {
    let mut inside = 0u64;

    for i in 0..samples {
        // 使用简单的 LCG 随机数生成器
        let seed = (i * 1103515245 + 12345) as u64;
        let x = ((seed >> 16) & 0xFFFF) as f64 / 65535.0;
        let y = ((seed >> 32) & 0xFFFF) as f64 / 65535.0;

        if x * x + y * y <= 1.0 {
            inside += 1;
        }
    }

    4.0 * inside as f64 / samples as f64
}

// ========== 5. Nilakantha 级数 ==========
/// π = 3 + 4/(2×3×4) - 4/(4×5×6) + 4/(6×7×8) - ...
/// 比莱布尼茨收敛快
fn nilakantha_pi(iterations: u64) -> f64 {
    let mut pi = 3.0;
    let mut sign = 1.0;

    for i in 1..=iterations {
        let denominator = (2 * i) as f64 * (2 * i + 1) as f64 * (2 * i + 2) as f64;
        pi += sign * 4.0 / denominator;
        sign = -sign;
    }

    pi
}

// ========== 6. BBP 公式 (Bailey-Borwein-Plouffe) ==========
/// 可以计算π的任意十六进制位，无需计算前面的位
/// π = Σ (1/16^k) × (4/(8k+1) - 2/(8k+4) - 1/(8k+5) - 1/(8k+6))
fn bbp_pi(digits: u64) -> f64 {
    let mut pi = 0.0;

    for k in 0..digits {
        let k = k as f64;
        let power = 16.0_f64.powi(-k as i32);
        pi += power
            * (4.0 / (8.0 * k + 1.0)
                - 2.0 / (8.0 * k + 4.0)
                - 1.0 / (8.0 * k + 5.0)
                - 1.0 / (8.0 * k + 6.0));
    }

    pi
}

// ========== 7. 高斯 - 勒让德算法 ==========
/// 二阶收敛，每次迭代正确位数翻倍
/// 25 次迭代即可计算出超过 4500 万位的π
fn gauss_legendre_pi(iterations: u64) -> f64 {
    let mut a = 1.0;
    let mut b = 2.0_f64.sqrt().recip(); // 1/√2
    let mut t = 0.25;
    let mut p = 1.0;

    for _ in 0..iterations {
        let a_next = (a + b) / 2.0;
        b = (a * b).sqrt();
        let a_diff = a - a_next;
        t -= p * a_diff * a_diff;
        a = a_next;
        p *= 2.0;
    }

    (a + b).powi(2) / (4.0 * t)
}

// ========== 8. Chudnovsky 算法 ==========
/// 目前最快的π计算算法之一
/// 每次迭代增加约 14 位有效数字
/// 用于打破π计算位数记录
fn chudnovsky_pi(iterations: u64) -> f64 {
    let mut sum = 0.0;

    for k in 0..iterations {
        let k = k as f64;

        // 分子：(6k)! × (545140134k + 13591409)
        let numerator = factorial(6 * k as usize) as f64 * (545140134.0 * k + 13591409.0);

        // 分母：(3k)! × (k!)^3 × 640320^(3k + 3/2)
        // 注意：使用正数，符号单独处理
        let denom1 = factorial(3 * k as usize) as f64;
        let denom2 = factorial(k as usize) as f64;
        let denom3: f64 = 640320.0_f64.powf(3.0 * k + 1.5);

        // (-1)^k 符号
        let sign = if k as i64 % 2 == 0 { 1.0 } else { -1.0 };

        let term = sign * numerator / (denom1 * denom2.powi(3) * denom3);
        sum += term;
    }

    // π = 1 / (12 × sum)
    1.0 / (12.0 * sum)
}

fn factorial(n: usize) -> u128 {
    (1..=n).map(|x| x as u128).product()
}
