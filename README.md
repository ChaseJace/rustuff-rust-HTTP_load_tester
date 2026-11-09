# ⚡ Rust Asynchronous HTTP Load Tester

A high-performance, concurrent HTTP load testing utility built from scratch in **Rust**. Designed to benchmark endpoint latency, track success/failure rates, and analyze performance metrics under concurrent network load.

## 🚀 Features
- **Asynchronous Concurrency:** Powered by `tokio` to fire and manage concurrent network requests efficiently without blocking CPU threads.
- **Dynamic CLI Parsing:** Built using `clap` to easily configure target URLs and request concurrency via terminal flags.
- **Thread-Safe Metrics:** Uses atomic counters (`AtomicUsize`) and smart pointers (`Arc`) for lock-free success/failure tracking.
- **Statistical Breakdown:** Safely aggregates individual request durations via a shared `Mutex` vector to compute **Fastest**, **Slowest**, and **Average** latencies.

## 🛠️ Tech Stack
- **Rust** (Edition 2021)
- **Tokio** (Async Runtime)
- **Reqwest** (HTTP Client)
- **Clap** (Command-Line Argument Parser)

---

## 📦 Installation & Usage

1. **Clone and navigate to the project directory:**
   ```bash
   git clone [https://github.com/your-username/rust-load-tester.git](https://github.com/your-username/rust-load-tester.git)
   cd rust-load-tester

2. Run with default arguments (targets https://httpbin.org/get with 10 requests):
```bash
  cargo run
```

3. Customize target URL and concurrent requests using CLI flags:
```bash
  cargo run -- --url [https://httpbin.org/anything](https://httpbin.org/anything) --requests 50
```

---

# 🛡️ Best Practices & Ethical Considerations
- **Target Authorization:** Always test against authorized endpoints, internal staging environments, or local servers (e.g., http://127.0.0.1:8080).
- **Rate Limiting & WAFs:** Public production sites utilize automated bot mitigation and rate-limiting firewalls. High concurrency against public commercial domains will quickly trigger 403 Forbidden or 429 Too Many Requests responses.

# 🔮 Future Recommendations
- **Percentile Metrics:** Implement p95 and p99 latency calculations for deeper production performance analysis.
- **Rate Limiting Controls:** Add a requests-per-second (RPS) throttle to simulate steady-state traffic rather than immediate bursts.
- **Custom Payload Support:** Extend CLI arguments to support custom HTTP headers and JSON bodies for POST requests.
