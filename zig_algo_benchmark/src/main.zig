const std = @import("std");
const linux = std.os.linux;

const ARRAY_SIZE: usize = 100_000;
const FIB_N: u64 = 45;
const PRIME_LIMIT: u64 = 100_000;
const BINARY_SEARCH_SIZE: usize = 1_000_000;

fn nanoTimestamp() i128 {
    var ts: linux.timespec = undefined;
    _ = linux.clock_gettime(.MONOTONIC, &ts);
    return @as(i128, @intCast(ts.sec)) * 1_000_000_000 + @as(i128, @intCast(ts.nsec));
}

pub fn main() !void {
    std.debug.print(
        \\╔════════════════════════════════════════╗
        \\║     Zig 算法基准测试                   ║
        \\╚════════════════════════════════════════╝
        \\
        \\
    , .{});

    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var results = std.array_list.Aligned(Result, null){};
    defer results.deinit(allocator);

    // 快速排序
    try results.append(allocator, try benchmark("快速排序", quickSortBenchmark));

    // 归并排序
    try results.append(allocator, try benchmark("归并排序", mergeSortBenchmark));

    // 堆排序
    try results.append(allocator, try benchmark("堆排序", heapSortBenchmark));

    // 二分搜索
    try results.append(allocator, try benchmark("二分搜索 (10000 次)", binarySearchBenchmark));

    // 斐波那契 (递归)
    try results.append(allocator, try benchmark("斐波那契 (递归)", fibonacciRecursiveBenchmark));

    // 斐波那契 (迭代)
    try results.append(allocator, try benchmark("斐波那契 (迭代)", fibonacciIterativeBenchmark));

    // 埃拉托斯特尼筛法求素数
    try results.append(allocator, try benchmark("埃拉托斯特尼筛法", sieveBenchmark));

    // 矩阵乘法
    try results.append(allocator, try benchmark("矩阵乘法 (100x100)", matrixMultiplyBenchmark));

    // 字符串处理
    try results.append(allocator, try benchmark("字符串反转 (100000 次)", stringReverseBenchmark));

    // 打印结果
    std.debug.print(
        \\
        \\┌─────────────────────────────────────┬──────────────┐
        \\│ 算法                                │ 耗时         │
        \\├─────────────────────────────────────┼──────────────┤
    , .{});

    for (results.items) |result| {
        std.debug.print(
            \\│ {s:<35} │ {:>10.3} ms │
        , .{ result.name, @as(f64, @floatFromInt(result.duration_ns)) / 1_000_000.0 });
    }

    std.debug.print(
        \\└─────────────────────────────────────┴──────────────┘
        \\
    , .{});
}

const Result = struct {
    name: []const u8,
    duration_ns: u64,
};

fn benchmark(name: []const u8, comptime f: fn () void) !Result {
    const start = nanoTimestamp();
    f();
    const end = nanoTimestamp();
    return Result{
        .name = name,
        .duration_ns = @intCast(end - start),
    };
}

fn generateRandomArray(allocator: std.mem.Allocator, size: usize) ![]i64 {
    const arr = try allocator.alloc(i64, size);
    var rng = std.Random.DefaultPrng.init(@intCast(nanoTimestamp()));
    const random = rng.random();
    for (arr) |*val| {
        val.* = @mod(random.int(i64), 2_000_000) - 1_000_000;
    }
    return arr;
}

fn generateSortedArray(allocator: std.mem.Allocator, size: usize) ![]i64 {
    const arr = try allocator.alloc(i64, size);
    for (0..size) |i| {
        arr[i] = @as(i64, @intCast(i)) * 2;
    }
    return arr;
}

// 快速排序基准测试
fn quickSortBenchmark() void {
    const arr = generateRandomArray(std.heap.page_allocator, ARRAY_SIZE) catch return;
    defer std.heap.page_allocator.free(arr);
    quickSort(arr, 0, arr.len - 1);
}

fn quickSort(arr: []i64, low: usize, high: usize) void {
    if (low < high) {
        const pi = partition(arr, low, high);
        if (pi > 0) {
            quickSort(arr, low, pi - 1);
        }
        quickSort(arr, pi + 1, high);
    }
}

fn partition(arr: []i64, low: usize, high: usize) usize {
    const pivot = arr[high];
    var i = low;
    for (low..high) |j| {
        if (arr[j] <= pivot) {
            std.mem.swap(i64, &arr[i], &arr[j]);
            i += 1;
        }
    }
    std.mem.swap(i64, &arr[i], &arr[high]);
    return i;
}

// 归并排序基准测试
fn mergeSortBenchmark() void {
    const arr = generateRandomArray(std.heap.page_allocator, ARRAY_SIZE) catch return;
    defer std.heap.page_allocator.free(arr);
    mergeSortMain(arr);
}

fn mergeSortMain(arr: []i64) void {
    const len = arr.len;
    if (len <= 1) return;
    
    const temp = std.heap.page_allocator.alloc(i64, len) catch return;
    defer std.heap.page_allocator.free(temp);
    
    mergeSortRecursive(arr, temp, 0, len);
}

fn mergeSortRecursive(arr: []i64, temp: []i64, start: usize, end: usize) void {
    if (end - start <= 1) return;
    
    const mid = start + (end - start) / 2;
    mergeSortRecursive(arr, temp, start, mid);
    mergeSortRecursive(arr, temp, mid, end);
    mergeArrays(arr, temp, start, mid, end);
}

fn mergeArrays(arr: []i64, temp: []i64, start: usize, mid: usize, end: usize) void {
    var i = start;
    var j = mid;
    var k = start;
    
    while (i < mid and j < end) {
        if (arr[i] <= arr[j]) {
            temp[k] = arr[i];
            i += 1;
        } else {
            temp[k] = arr[j];
            j += 1;
        }
        k += 1;
    }
    
    while (i < mid) {
        temp[k] = arr[i];
        i += 1;
        k += 1;
    }
    
    while (j < end) {
        temp[k] = arr[j];
        j += 1;
        k += 1;
    }
    
    for (start..end) |idx| {
        arr[idx] = temp[idx];
    }
}

// 堆排序基准测试
fn heapSortBenchmark() void {
    const arr = generateRandomArray(std.heap.page_allocator, ARRAY_SIZE) catch return;
    defer std.heap.page_allocator.free(arr);
    heapSort(arr);
}

fn heapSort(arr: []i64) void {
    const n = arr.len;
    var i: usize = n / 2;
    while (i > 0) {
        i -= 1;
        heapify(arr, n, i);
    }
    
    var j: usize = n;
    while (j > 1) {
        j -= 1;
        std.mem.swap(i64, &arr[0], &arr[j]);
        heapify(arr, j, 0);
    }
}

fn heapify(arr: []i64, n: usize, i: usize) void {
    var largest = i;
    const left = 2 * i + 1;
    const right = 2 * i + 2;
    
    if (left < n and arr[left] > arr[largest]) {
        largest = left;
    }
    if (right < n and arr[right] > arr[largest]) {
        largest = right;
    }
    if (largest != i) {
        std.mem.swap(i64, &arr[i], &arr[largest]);
        heapify(arr, n, largest);
    }
}

// 二分搜索基准测试
fn binarySearchBenchmark() void {
    const arr = generateSortedArray(std.heap.page_allocator, BINARY_SEARCH_SIZE) catch return;
    defer std.heap.page_allocator.free(arr);
    
    for (0..10000) |i| {
        _ = binarySearch(arr, @as(i64, @intCast(i * 100)));
    }
}

fn binarySearch(arr: []i64, target: i64) bool {
    var left: usize = 0;
    var right: usize = arr.len;
    
    while (left < right) {
        const mid = left + (right - left) / 2;
        if (arr[mid] == target) {
            return true;
        } else if (arr[mid] < target) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    return false;
}

// 斐波那契 (递归) 基准测试
fn fibonacciRecursiveBenchmark() void {
    const result = fibonacciRecursive(FIB_N);
    _ = result;
}

fn fibonacciRecursive(n: u64) u64 {
    if (n <= 1) return n;
    return fibonacciRecursive(n - 1) + fibonacciRecursive(n - 2);
}

// 斐波那契 (迭代) 基准测试
fn fibonacciIterativeBenchmark() void {
    const result = fibonacciIterative(FIB_N);
    _ = result;
}

fn fibonacciIterative(n: u64) u64 {
    if (n <= 1) return n;
    var a: u64 = 0;
    var b: u64 = 1;
    for (2..n + 1) |_| {
        const temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}

// 埃拉托斯特尼筛法基准测试
fn sieveBenchmark() void {
    const primes = sieveOfEratosthenes(PRIME_LIMIT);
    defer std.heap.page_allocator.free(primes);
}

fn sieveOfEratosthenes(limit: u64) []u64 {
    const limit_usize: usize = @intCast(limit);
    const is_prime = std.heap.page_allocator.alloc(bool, limit_usize + 1) catch return &[_]u64{};
    @memset(is_prime, true);
    is_prime[0] = false;
    is_prime[1] = false;
    
    var i: usize = 2;
    while (i * i <= limit_usize) : (i += 1) {
        if (is_prime[i]) {
            var j = i * i;
            while (j <= limit_usize) : (j += i) {
                is_prime[j] = false;
            }
        }
    }
    
    var count: usize = 0;
    for (is_prime) |p| {
        if (p) count += 1;
    }
    
    const primes = std.heap.page_allocator.alloc(u64, count) catch return &[_]u64{};
    var idx: usize = 0;
    for (0..is_prime.len) |x| {
        if (is_prime[x]) {
            primes[idx] = @intCast(x);
            idx += 1;
        }
    }
    
    std.heap.page_allocator.free(is_prime);
    return primes;
}

// 矩阵乘法基准测试
fn matrixMultiplyBenchmark() void {
    const size: usize = 100;
    const result = matrixMultiply(size);
    defer {
        for (result) |row| {
            std.heap.page_allocator.free(row);
        }
        std.heap.page_allocator.free(result);
    }
}

fn matrixMultiply(size: usize) [][]i64 {
    const a = std.heap.page_allocator.alloc([]i64, size) catch return &[_][]i64{};
    const b = std.heap.page_allocator.alloc([]i64, size) catch return &[_][]i64{};
    const c = std.heap.page_allocator.alloc([]i64, size) catch return &[_][]i64{};
    
    for (0..size) |i| {
        a[i] = std.heap.page_allocator.alloc(i64, size) catch return &[_][]i64{};
        b[i] = std.heap.page_allocator.alloc(i64, size) catch return &[_][]i64{};
        c[i] = std.heap.page_allocator.alloc(i64, size) catch return &[_][]i64{};
        @memset(a[i], 1);
        @memset(b[i], 2);
        @memset(c[i], 0);
    }
    
    for (0..size) |i| {
        for (0..size) |j| {
            for (0..size) |k| {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    
    for (0..size) |i| {
        std.heap.page_allocator.free(a[i]);
        std.heap.page_allocator.free(b[i]);
    }
    std.heap.page_allocator.free(a);
    std.heap.page_allocator.free(b);
    
    return c;
}

// 字符串反转基准测试
fn stringReverseBenchmark() void {
    const s = "Hello, World! 你好世界";
    var buffer: [100]u8 = undefined;
    
    for (0..100000) |_| {
        _ = reverseStringInPlace(s, &buffer);
    }
}

fn reverseStringInPlace(s: []const u8, buffer: []u8) []u8 {
    const len = @min(s.len, buffer.len);
    var i: usize = 0;
    while (i < len) : (i += 1) {
        buffer[i] = s[len - 1 - i];
    }
    return buffer[0..len];
}
