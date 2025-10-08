// Demonstration of the System Runner
// This example shows how to use the SystemRunner without requiring eBPF binaries

use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 AgentSight System Runner Demo");
    println!("{}", "=".repeat(60));
    println!();

    let current_pid = std::process::id();
    println!("Monitoring current process (PID: {})", current_pid);
    println!("This demo shows system monitoring capabilities:");
    println!("- CPU usage tracking");
    println!("- Memory consumption (RSS, VSZ)");
    println!("- Thread count");
    println!("- Child process aggregation");
    println!();
    println!("Running for 10 seconds with 2-second intervals...");
    println!("{}", "=".repeat(60));
    println!();

    // Simulate the system runner's core functionality
    let mut iteration = 0;
    let mut interval = time::interval(Duration::from_secs(2));

    for _ in 0..5 {
        interval.tick().await;
        iteration += 1;

        // Read current process stats
        let stat_path = format!("/proc/{}/stat", current_pid);
        let statm_path = format!("/proc/{}/statm", current_pid);
        let comm_path = format!("/proc/{}/comm", current_pid);
        let task_dir = format!("/proc/{}/task", current_pid);

        if let Ok(comm) = std::fs::read_to_string(&comm_path) {
            if let Ok(statm) = std::fs::read_to_string(&statm_path) {
                let fields: Vec<&str> = statm.split_whitespace().collect();
                if fields.len() >= 2 {
                    let vsz_pages: u64 = fields[0].parse().unwrap_or(0);
                    let rss_pages: u64 = fields[1].parse().unwrap_or(0);
                    let page_size = 4u64;
                    let vsz_kb = vsz_pages * page_size;
                    let rss_kb = rss_pages * page_size;
                    let rss_mb = rss_kb / 1024;

                    let thread_count = std::fs::read_dir(&task_dir)
                        .map(|entries| entries.count())
                        .unwrap_or(1);

                    println!("[{}] {} (PID: {})", iteration, comm.trim(), current_pid);
                    println!("    Memory: RSS={}MB, VSZ={}MB", rss_mb, vsz_kb / 1024);
                    println!("    Threads: {}", thread_count);

                    // Simulate JSON event output
                    let json_event = serde_json::json!({
                        "type": "system_metrics",
                        "pid": current_pid,
                        "comm": comm.trim(),
                        "timestamp": std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_nanos(),
                        "memory": {
                            "rss_kb": rss_kb,
                            "rss_mb": rss_mb,
                            "vsz_kb": vsz_kb,
                            "vsz_mb": vsz_kb / 1024,
                        },
                        "process": {
                            "threads": thread_count,
                            "children": 0,
                        },
                    });

                    println!("    Event: {}", serde_json::to_string_pretty(&json_event)?);
                    println!();
                }
            }
        }
    }

    println!("{}", "=".repeat(60));
    println!("✅ Demo completed successfully!");
    println!();
    println!("The actual SystemRunner provides:");
    println!("- Real-time CPU percentage calculation");
    println!("- Child process discovery and aggregation");
    println!("- Threshold-based alerting");
    println!("- Integration with analyzer pipeline");
    println!("- Web server support for visualization");
    println!("- Log file output with rotation");
    println!();
    println!("Usage example:");
    println!("  cargo run system --pid {} --interval 2", current_pid);
    println!("  cargo run system --comm rust --cpu-threshold 50");
    println!("  cargo run system --server --server-port 8080");

    Ok(())
}
