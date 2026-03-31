use broken_app::{algo, leak_buffer, normalize, sum_even};

fn main() {
    println!("=== Extreme Demo for Profiling ===\n");

    // Увеличиваем нагрузку
    let iterations = 1_000;

    for iteration in 1..=iterations {
        // sum_even на больших данных
        let nums: Vec<i64> = (0..1_000).collect();
        let _ = sum_even(&nums);

        // leak_buffer
        let data: Vec<u8> = (0..250).map(|x| x % 3).collect();
        let _ = leak_buffer(&data);

        // normalize
        let text = " Hello   World   Test   String   With   Many   Words ";
        let _ = normalize(text);

        // Fibonacci (с мемоизацией, но всё равно нагружает)
        for n in [20, 25, 30, 35] {
            let _ = algo::fast_fib(n);
        }

        // dedup на больших данных
        let dedup_data: Vec<u64> = (0..5000).flat_map(|n| [n, n, n]).collect();
        let _ = algo::fast_dedup(&dedup_data);

        if iteration % 100 == 0 {
            println!("Completed {} / {} iterations", iteration, iterations);
        }
    }

    println!("\n=== Extreme Demo Complete ===");
}
