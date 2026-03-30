use broken_app::{algo, leak_buffer, normalize, sum_even};

// Тесты для sum_even
#[test]
fn sums_even_numbers() {
    let nums = [1, 2, 3, 4];
    assert_eq!(sum_even(&nums), 6);
}

#[test]
fn sums_even_numbers_regression() {
    let test_cases = vec![
        (vec![], 0, "empty"),
        (vec![1], 0, "single_odd"),
        (vec![2], 2, "single_even"),
        (vec![1, 2, 3, 4], 6, "mixed"),
        (vec![0, 1, 2, 3], 2, "with_zero"),
        (vec![-4, -3, -2, -1], -6, "negative"),
        (vec![1000, 2001, 3000], 4000, "large"),
    ];
    
    for (input, expected, desc) in test_cases {
        assert_eq!(sum_even(&input), expected, "Failed: {}", desc);
    }
}

#[test]
fn sums_even_numbers_large() {
    let data: Vec<i64> = (0..10_000).collect();
    let result = sum_even(&data);
    assert_eq!(result, 24_995_000);
}

// Тесты для leak_buffer
#[test]
fn counts_non_zero_bytes() {
    let data = [0_u8, 1, 0, 2, 3];
    assert_eq!(leak_buffer(&data), 3);
}

#[test]
fn leak_buffer_no_memory_leak() {
    // Этот тест проверяет отсутствие утечек памяти
    let data = vec![1, 2, 3, 4, 5];
    for _ in 0..100 {
        assert_eq!(leak_buffer(&data), 5);
    }
}

#[test]
fn leak_buffer_empty() {
    assert_eq!(leak_buffer(&[]), 0);
}

#[test]
fn leak_buffer_all_zeros() {
    let zeros = vec![0; 100];
    assert_eq!(leak_buffer(&zeros), 0);
}

// Остальные тесты
#[test]
fn dedup_preserves_uniques() {
    let uniq = algo::slow_dedup(&[5, 5, 1, 2, 2, 3]);
    assert_eq!(uniq, vec![1, 2, 3, 5]);
}

#[test]
fn fib_small_numbers() {
    assert_eq!(algo::slow_fib(10), 55);
}

#[test]
fn normalize_simple() {
    assert_eq!(normalize(" Hello   World "), "helloworld");
}

#[test]
fn averages_only_positive() {
    let nums = [-5, 5, 15];
    assert!((broken_app::average_positive(&nums) - 10.0).abs() < f64::EPSILON);
}

#[test]
fn test_use_after_free() {
    // Эта функция помечена как unsafe, поэтому её нужно вызывать в unsafe блоке
    let result = unsafe { broken_app::use_after_free() };
    // Miri должен поймать use-after-free здесь
    println!("Result: {}", result);
}

#[test]
fn race_increment_is_correct() {
    let total = broken_app::concurrency::race_increment(1_000, 4);
    assert_eq!(total, 4_000);
}