use clap::Parser;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

#[derive(Parser, Debug)]
#[command(
    name = "Rust Load Tester",
    about = "Fires concurrent HTTP requests to test performance."
)]
struct Args {
    /// The target URL to test
    #[arg(short, long, default_value = "https://httpbin.org/get")]
    url: String,

    /// Total number of concurrent requests to fire
    #[arg(short, long, default_value_t = 10)]
    requests: usize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("Targeting: {}", args.url);
    println!("Launching {} concurrent requests...\n", args.requests);

    let global_start = Instant::now();

    // Atomic counters for success/fail counts
    let success_count = Arc::new(AtomicUsize::new(0));
    let fail_count = Arc::new(AtomicUsize::new(0));

    // Shared thread-safe vector to collect individual request latencies (durations)
    // Mutex ensures only one background task can push to the vector at a time safely.
    let latencies: Arc<Mutex<Vec<Duration>>> = Arc::new(Mutex::new(Vec::new()));

    let mut tasks = vec![];

    for _i in 1..=args.requests {
        let url = args.url.clone();

        let success_clone = Arc::clone(&success_count);
        let fail_clone = Arc::clone(&fail_count);
        let latencies_clone = Arc::clone(&latencies);

        let task = tokio::spawn(async move {
            let req_start = Instant::now();

            match reqwest::get(&url).await {
                Ok(response) => {
                    let duration = req_start.elapsed();

                    if response.status().is_success() {
                        success_clone.fetch_add(1, Ordering::SeqCst);
                        // Safely lock the mutex and record the request duration
                        let mut lats = latencies_clone.lock().await;
                        lats.push(duration);
                    } else {
                        fail_clone.fetch_add(1, Ordering::SeqCst);
                    }
                }
                Err(err) => {
                    fail_clone.fetch_add(1, Ordering::SeqCst);
                    eprintln!("Request Failed: {}", err);
                }
            }
        });

        tasks.push(task);
    }

    // Await all tasks to finish
    for task in tasks {
        let _ = task.await;
    }

    let total_duration = global_start.elapsed();

    // Load final counts
    let total_successes = success_count.load(Ordering::SeqCst);
    let total_failures = fail_count.load(Ordering::SeqCst);

    // Retrieve latencies from the mutex lock
    let lats = latencies.lock().await;

    println!("\n==============================");
    println!("       LOAD TEST SUMMARY      ");
    println!("==============================");
    println!("Total Duration:    {:?}", total_duration);
    println!("Successful:        {}", total_successes);
    println!("Failed/Errors:     {}", total_failures);

    if !lats.is_empty() {
        // Calculate Min, Max, and Average latencies
        let min_lat = lats.iter().min().unwrap();
        let max_lat = lats.iter().max().unwrap();

        let sum_lat: Duration = lats.iter().sum();
        let avg_lat = sum_lat / lats.len() as u32;

        println!("------------------------------");
        println!("Fastest Request:   {:?}", min_lat);
        println!("Slowest Request:   {:?}", max_lat);
        println!("Average Latency:   {:?}", avg_lat);
    } else {
        println!("------------------------------");
        println!("No successful requests recorded to compute latency.");
    }
    println!("==============================");
}
