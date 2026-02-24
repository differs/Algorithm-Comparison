use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::time::Instant;

const ARRAY_SIZE: usize = 100_000;
const FIB_N: u64 = 45;
const PRIME_LIMIT: u64 = 100_000;
const BINARY_SEARCH_SIZE: usize = 1_000_000;

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║     Rust 算法基准测试 (优化版)         ║");
    println!("╚════════════════════════════════════════╝\n");

    let mut results = Vec::new();

    // 快速排序 (尾递归优化)
    results.push(benchmark("快速排序 (优化)", || {
        let mut arr = generate_random_array(ARRAY_SIZE);
        quick_sort_optimized(&mut arr);
        arr
    }));

    // 归并排序 (单缓冲区)
    results.push(benchmark("归并排序 (优化)", || {
        let mut arr = generate_random_array(ARRAY_SIZE);
        let mut temp = arr.clone();
        merge_sort_optimized(&mut arr, &mut temp);
        arr
    }));

    // 堆排序 (迭代 heapify)
    results.push(benchmark("堆排序 (优化)", || {
        let mut arr = generate_random_array(ARRAY_SIZE);
        heap_sort_optimized(&mut arr);
        arr
    }));

    // 二分搜索
    results.push(benchmark("二分搜索 (10000 次)", || {
        let arr = generate_sorted_array(BINARY_SEARCH_SIZE);
        let mut count = 0;
        for i in 0..10000 {
            count += binary_search_optimized(&arr, i * 100) as usize;
        }
        count
    }));

    // 斐波那契 (递归)
    results.push(benchmark("斐波那契 (递归)", || {
        fibonacci_recursive(FIB_N)
    }));

    // 斐波那契 (迭代)
    results.push(benchmark("斐波那契 (迭代)", || {
        fibonacci_iterative(FIB_N)
    }));

    // 埃拉托斯特尼筛法 (优化)
    results.push(benchmark("埃拉托斯特尼筛法 (优化)", || {
        sieve_optimized(PRIME_LIMIT)
    }));

    // 矩阵乘法 (缓存优化)
    results.push(benchmark("矩阵乘法 (100x100)", || {
        matrix_multiply_optimized(100)
    }));

    // 字符串处理 (字节操作)
    results.push(benchmark("字符串反转 (100000 次)", || {
        let s = b"Hello, World! Hello World!";
        let mut buf = [0u8; 100];
        let mut result_len = 0;
        for _ in 0..100000 {
            result_len = reverse_bytes(s, &mut buf);
        }
        result_len
    }));

    // 打印结果
    println!("\n┌─────────────────────────────────────┬──────────────┐");
    println!("│ 算法                                │ 耗时         │");
    println!("├─────────────────────────────────────┼──────────────┤");
    for (name, duration) in &results {
        println!(
            "│ {:<35} │ {:>10.3} ms │",
            name,
            duration.as_secs_f64() * 1000.0
        );
    }
    println!("└─────────────────────────────────────┴──────────────┘");
}

fn benchmark<F, T>(name: &str, mut f: F) -> (String, std::time::Duration)
where
    F: FnMut() -> T,
{
    let start = Instant::now();
    std::hint::black_box(f());
    let duration = start.elapsed();
    (name.to_string(), duration)
}

fn generate_random_array(size: usize) -> Vec<i64> {
    let mut rng = StdRng::seed_from_u64(42);
    (0..size)
        .map(|_| rng.gen_range(-1000000..1000000))
        .collect()
}

fn generate_sorted_array(size: usize) -> Vec<i64> {
    (0..size as i64).map(|x| x * 2).collect()
}

// ========== 优化的快速排序 (尾递归 + 三数取中) ==========
fn quick_sort_optimized(arr: &mut [i64]) {
    if arr.len() <= 1 {
        return;
    }

    let mut stack = [(0usize, arr.len() - 1); 64];
    let mut top = 0;

    stack[top] = (0, arr.len() - 1);

    while top < 64 {
        let (low, high) = stack[top];
        top -= 1;

        if low >= high {
            continue;
        }

        let pi = partition_optimized(arr, low, high);

        // 先处理大的，尾递归处理小的
        if pi > 0 && pi - 1 > low {
            top += 1;
            stack[top] = (low, pi - 1);
        }
        if pi + 1 < high {
            top += 1;
            stack[top] = (pi + 1, high);
        }
    }
}

fn partition_optimized(arr: &mut [i64], low: usize, high: usize) -> usize {
    // 三数取中法选择 pivot
    let mid = low + (high - low) / 2;
    if arr[low] > arr[mid] {
        arr.swap(low, mid);
    }
    if arr[low] > arr[high] {
        arr.swap(low, high);
    }
    if arr[mid] > arr[high] {
        arr.swap(mid, high);
    }
    arr.swap(mid, high);

    let pivot = arr[high];
    let mut i = low;

    for j in low..high {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}

// ========== 优化的归并排序 (单缓冲区) ==========
fn merge_sort_optimized(arr: &mut [i64], temp: &mut [i64]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    // 小数组使用插入排序
    if len <= 32 {
        insertion_sort(arr);
        return;
    }

    let mid = len / 2;
    merge_sort_optimized(&mut temp[..mid], &mut arr[..mid]);
    merge_sort_optimized(&mut temp[mid..], &mut arr[mid..]);
    merge_optimized(arr, &temp[..mid], &temp[mid..]);
}

fn merge_optimized(arr: &mut [i64], left: &[i64], right: &[i64]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        arr[k] = if left[i] <= right[j] {
            let val = left[i];
            i += 1;
            val
        } else {
            let val = right[j];
            j += 1;
            val
        };
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn insertion_sort(arr: &mut [i64]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

// ========== 优化的堆排序 (迭代 heapify) ==========
fn heap_sort_optimized(arr: &mut [i64]) {
    let n = arr.len();

    // 建堆
    for i in (0..n / 2).rev() {
        heapify_iterative(arr, n, i);
    }

    // 排序
    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify_iterative(arr, i, 0);
    }
}

fn heapify_iterative(arr: &mut [i64], n: usize, mut i: usize) {
    loop {
        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < n && arr[left] > arr[largest] {
            largest = left;
        }
        if right < n && arr[right] > arr[largest] {
            largest = right;
        }

        if largest == i {
            break;
        }

        arr.swap(i, largest);
        i = largest;
    }
}

// ========== 优化的二分搜索 ==========
fn binary_search_optimized(arr: &[i64], target: i64) -> bool {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        let val = arr[mid];

        if val == target {
            return true;
        }

        // 分支预测优化
        let mask = (val < target) as usize;
        left = left * (1 - mask) + (mid + 1) * mask;
        right = right * mask + mid * (1 - mask);
    }
    false
}

// ========== 斐波那契 (递归) ==========
fn fibonacci_recursive(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

// ========== 斐波那契 (迭代) ==========
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

// ========== 优化的筛法 (跳过偶数) ==========
fn sieve_optimized(limit: u64) -> Vec<u64> {
    if limit < 2 {
        return vec![];
    }

    let limit = limit as usize;
    let size = (limit / 2) + 1;
    let mut is_prime = vec![true; size];

    let sqrt_limit = (limit as f64).sqrt() as usize;
    for i in 1..=((sqrt_limit / 2) + 1) {
        if is_prime[i] {
            let prime = 2 * i + 1;
            let start = (prime * prime) / 2;
            for j in start..size {
                if j < is_prime.len() {
                    is_prime[j] = false;
                }
            }
        }
    }

    let mut primes = vec![2];
    for i in 1..size {
        if is_prime[i] {
            primes.push((2 * i + 1) as u64);
        }
    }
    primes
}

// ========== 优化的矩阵乘法 (缓存友好 + 扁平数组) ==========
fn matrix_multiply_optimized(size: usize) -> Vec<i64> {
    let a = vec![1i64; size * size];
    let b = vec![2i64; size * size];
    let mut c = vec![0i64; size * size];

    // 转置 B 以提高缓存命中率
    let mut b_t = vec![0i64; size * size];
    for i in 0..size {
        for j in 0..size {
            b_t[j * size + i] = b[i * size + j];
        }
    }

    // 分块优化 (block size = 32)
    const BLOCK_SIZE: usize = 32;

    for ii in (0..size).step_by(BLOCK_SIZE) {
        for jj in (0..size).step_by(BLOCK_SIZE) {
            for kk in (0..size).step_by(BLOCK_SIZE) {
                let i_end = (ii + BLOCK_SIZE).min(size);
                let j_end = (jj + BLOCK_SIZE).min(size);
                let k_end = (kk + BLOCK_SIZE).min(size);

                for i in ii..i_end {
                    for j in jj..j_end {
                        let mut sum = 0i64;
                        for k in kk..k_end {
                            sum += a[i * size + k] * b_t[j * size + k];
                        }
                        c[i * size + j] += sum;
                    }
                }
            }
        }
    }

    c
}

// ========== 优化的字符串反转 (字节操作) ==========
fn reverse_bytes(s: &[u8], buf: &mut [u8]) -> usize {
    let len = s.len().min(buf.len());
    let slice = &s[..len];

    for i in 0..len {
        buf[i] = slice[len - 1 - i];
    }

    len
}
