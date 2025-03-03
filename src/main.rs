use std::time::SystemTime;

use ip_scan_async::ping;
use tokio::{spawn, task::JoinSet};

const DISPLAY_ERROR: bool = false;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start_time = SystemTime::now();

    let tasks: JoinSet<_> = (1..255)
        .map(async |v| {
            let ip_address = format!("192.168.86.{v}");
            spawn(ping(ip_address, Some("192.168.86.200".to_string()))).await
        })
        .collect();

    let mut ip_cnt = 0;
    for v in tasks.join_all().await {
        match v? {
            Ok(t) => {
                ip_cnt = ip_cnt + 1;
                println!("[{}] ({})", t.0, t.1)
            }
            Err(e) => {
                if DISPLAY_ERROR {
                    println!("Error {:?}", e)
                }
            }
        }
    }
    let elapsed = start_time.elapsed().unwrap();
    println!("Found ips {ip_cnt} in {} Seconds", elapsed.as_secs_f64());
    Ok(())
}
