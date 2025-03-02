use std::time::SystemTime;

use ip_scan_async::ping;
use tokio::spawn;

#[tokio::main()]
async fn main() -> anyhow::Result<()> {
    let start_time = SystemTime::now();
    let mut handles = vec![];
    for i in 1..255 {
        let ip_address = format!("192.168.86.{i}");
        let p = spawn(ping(ip_address, Some("192.168.86.200".to_string())));
        handles.push(p);
    }

    let mut ip_cnt = 0;
    for v in handles {
        match v.await? {
            Ok(t) => {
                ip_cnt = ip_cnt + 1;
                println!("Address {}", t.0)
            }
            Err(_e) => {}
        }
    }

    let elapsed = start_time.elapsed().unwrap();
    println!("Found ips {ip_cnt} in {} Seconds", elapsed.as_secs_f64());
    Ok(())
}
