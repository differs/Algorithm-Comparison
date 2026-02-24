const std = @import("std");
const math = std.math;

pub fn main() !void {
    std.debug.print(
        \\╔════════════════════════════════════════╗
        \\║     圆周率 π 计算方案对比 (Zig)        ║
        \\╚════════════════════════════════════════╝
        \\
        \\
    , .{});

    var results = std.array_list.Aligned(Result, null){};
    defer results.deinit(std.heap.page_allocator);

    // 1. 莱布尼茨公式 (1000 万次迭代)
    try results.append(std.heap.page_allocator, try benchmark("莱布尼茨公式 (10M)", leibnizPi, .{10_000_000}));

    // 2. 改进的莱布尼茨 (100 万次)
    try results.append(std.heap.page_allocator, try benchmark("改进莱布尼茨 (1M)", improvedLeibnizPi, .{1_000_000}));

    // 3. 马青公式 (100 次)
    try results.append(std.heap.page_allocator, try benchmark("马青公式 (100)", machinPi, .{100}));

    // 4. 蒙特卡洛方法 (1000 万次)
    try results.append(std.heap.page_allocator, try benchmark("蒙特卡洛 (10M)", monteCarloPi, .{10_000_000}));

    // 5. Nilakantha 级数 (1000 万次)
    try results.append(std.heap.page_allocator, try benchmark("Nilakantha (10M)", nilakanthaPi, .{10_000_000}));

    // 6. BBP 公式 (计算 15 位)
    try results.append(std.heap.page_allocator, try benchmark("BBP 公式 (15 位)", bbpPi, .{15}));

    // 7. 高斯 - 勒让德算法 (5 次迭代)
    try results.append(std.heap.page_allocator, try benchmark("高斯 - 勒让德 (5 次)", gaussLegendrePi, .{5}));

    // 8. Chudnovsky 算法 (5 次迭代)
    try results.append(std.heap.page_allocator, try benchmark("Chudnovsky (5 次)", chudnovskyPi, .{5}));

    // 打印结果
    std.debug.print(
        \\
        \\┌─────────────────────────────────────┬──────────────┬──────────────┐
        \\│ 计算方案                            │ 耗时         │ π 值         │
        \\├─────────────────────────────────────┼──────────────┼──────────────┤
    , .{});

    const PI_REF = 3.14159265358979323846;
    for (results.items) |result| {
        const error_val = @abs(result.pi_value - PI_REF);
        const accuracy = if (error_val < 1e-10) "✓✓✓" else if (error_val < 1e-6) "✓✓" else if (error_val < 1e-3) "✓" else "✗";
        
        std.debug.print(
            \\│ {s:<35} │ {:>10.3} ms │ {d:.10} {s} │
        , .{ result.name, @as(f64, @floatFromInt(result.duration_ns)) / 1_000_000.0, result.pi_value, accuracy });
    }

    std.debug.print(
        \\└─────────────────────────────────────┴──────────────┴──────────────┘
        \\
        \\参考值：π = 3.14159265358979323846...
        \\
    , .{});
}

const Result = struct {
    name: []const u8,
    duration_ns: u64,
    pi_value: f64,
};

fn nanoTimestamp() i128 {
    var ts: std.os.linux.timespec = undefined;
    _ = std.os.linux.clock_gettime(.MONOTONIC, &ts);
    return @as(i128, @intCast(ts.sec)) * 1_000_000_000 + @as(i128, @intCast(ts.nsec));
}

fn benchmark(name: []const u8, comptime f: fn (u64) f64, args: anytype) !Result {
    const start = nanoTimestamp();
    const pi_value = f(args[0]);
    const end = nanoTimestamp();
    return Result{
        .name = name,
        .duration_ns = @intCast(end - start),
        .pi_value = pi_value,
    };
}

// ========== 1. 莱布尼茨公式 ==========
fn leibnizPi(iterations: u64) f64 {
    var sum: f64 = 0.0;
    var i: u64 = 0;
    while (i < iterations) : (i += 1) {
        const term: f64 = if (i % 2 == 0) 1.0 else -1.0;
        sum += term / (2.0 * @as(f64, @floatFromInt(i)) + 1.0);
    }
    return sum * 4.0;
}

// ========== 2. 改进的莱布尼茨公式 ==========
fn improvedLeibnizPi(iterations: u64) f64 {
    var sum: f64 = 0.0;
    var term: f64 = 1.0;
    var denominator: f64 = 1.0;
    
    var i: u64 = 0;
    while (i < iterations) : (i += 1) {
        sum += term / denominator;
        term = -term;
        denominator += 2.0;
        
        if (i > 0 and i % 100 == 0) {
            const prev = sum - term / denominator;
            sum = sum + (sum - prev) / 3.0;
        }
    }
    return sum * 4.0;
}

// ========== 3. 马青公式 ==========
fn arctan(x: f64, n: u64) f64 {
    const x2 = x * x;
    var sum: f64 = 0.0;
    var term: f64 = x;
    var divisor: f64 = 1.0;
    
    var i: u64 = 0;
    while (i < n) : (i += 1) {
        sum += term / divisor;
        term *= -x2;
        divisor += 2.0;
    }
    return sum;
}

fn machinPi(iterations: u64) f64 {
    return 4.0 * (4.0 * arctan(1.0 / 5.0, iterations) - arctan(1.0 / 239.0, iterations));
}

// ========== 4. 蒙特卡洛方法 ==========
fn monteCarloPi(samples: u64) f64 {
    var inside: u64 = 0;
    
    var i: u64 = 0;
    while (i < samples) : (i += 1) {
        const seed = (@as(u64, i) * 1103515245 + 12345);
        const x = @as(f64, @floatFromInt((seed >> 16) & 0xFFFF)) / 65535.0;
        const y = @as(f64, @floatFromInt((seed >> 32) & 0xFFFF)) / 65535.0;
        
        if (x * x + y * y <= 1.0) {
            inside += 1;
        }
    }
    
    return 4.0 * @as(f64, @floatFromInt(inside)) / @as(f64, @floatFromInt(samples));
}

// ========== 5. Nilakantha 级数 ==========
fn nilakanthaPi(iterations: u64) f64 {
    var pi: f64 = 3.0;
    var sign: f64 = 1.0;
    
    var i: u64 = 1;
    while (i <= iterations) : (i += 1) {
        const denom = @as(f64, @floatFromInt(2 * i)) * 
                      @as(f64, @floatFromInt(2 * i + 1)) * 
                      @as(f64, @floatFromInt(2 * i + 2));
        pi += sign * 4.0 / denom;
        sign = -sign;
    }
    
    return pi;
}

// ========== 6. BBP 公式 ==========
fn bbpPi(digits: u64) f64 {
    var pi: f64 = 0.0;
    
    var k: u64 = 0;
    while (k < digits) : (k += 1) {
        const kf = @as(f64, @floatFromInt(k));
        const power = math.pow(f64, 16.0, -kf);
        pi += power * (
            4.0 / (8.0 * kf + 1.0)
            - 2.0 / (8.0 * kf + 4.0)
            - 1.0 / (8.0 * kf + 5.0)
            - 1.0 / (8.0 * kf + 6.0)
        );
    }
    
    return pi;
}

// ========== 7. 高斯 - 勒让德算法 ==========
fn gaussLegendrePi(iterations: u64) f64 {
    var a: f64 = 1.0;
    var b: f64 = @sqrt(2.0);
    b = 1.0 / b;
    var t: f64 = 0.25;
    var p: f64 = 1.0;
    
    var i: u64 = 0;
    while (i < iterations) : (i += 1) {
        const a_next = (a + b) / 2.0;
        b = @sqrt(a * b);
        const a_diff = a - a_next;
        t -= p * a_diff * a_diff;
        a = a_next;
        p *= 2.0;
    }
    
    return math.pow(f64, a + b, 2.0) / (4.0 * t);
}

// ========== 8. Chudnovsky 算法 ==========
fn chudnovskyPi(iterations: u64) f64 {
    var sum: f64 = 0.0;
    
    var k: u64 = 0;
    while (k < iterations) : (k += 1) {
        const kf = @as(f64, @floatFromInt(k));
        
        const numerator = factorial(6 * @as(usize, @intCast(k))) * 
                         (545140134.0 * kf + 13591409.0);
        
        const denom1 = factorial(3 * @as(usize, @intCast(k)));
        const denom2 = factorial(@as(usize, @intCast(k)));
        const denom3 = math.pow(f64, 640320.0, 3.0 * kf + 1.5);
        
        const sign: f64 = if (k % 2 == 0) 1.0 else -1.0;
        
        const term = sign * numerator / (denom1 * math.pow(f64, denom2, 3.0) * denom3);
        sum += term;
    }
    
    return 1.0 / (12.0 * sum);
}

fn factorial(n: usize) f64 {
    var result: f64 = 1.0;
    var i: usize = 1;
    while (i <= n) : (i += 1) {
        result *= @as(f64, @floatFromInt(i));
    }
    return result;
}
