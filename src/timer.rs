

#[macro_export]
macro_rules! measure_time {
    ($func:expr) => {{
        let start = Instant::now();
        let result = $func();
        let duration = start.elapsed();
        println!("Execution Time : {:?}", duration);
        let _ = result;
    }};
}