use std::{
    io::{BufRead, Read},
    process::Stdio,
};

use anyhow::Ok;
use tokio::{
    io::{AsyncReadExt, BufReader},
    process::Command,
};

#[derive(Debug, Clone)]
pub struct IpAddressLookupError {
    ip_address: String,
    cause: String,
}

impl IpAddressLookupError {
    fn new(ip_address: &str, cause: &str) -> Self {
        Self {
            ip_address: ip_address.to_string(),
            cause: cause.to_string(),
        }
    }
}

pub async fn launch_process(cmd: &str, args: &[&str]) -> anyhow::Result<String> {
    let mut process = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()?;
    let mut process_out = String::new();
    let std_out = process.stdout.take();
    std_out.unwrap().read_to_string(&mut process_out).await?;
    Ok(process_out)
}

pub async fn ping(
    ip_address: String,
    dns_ip: Option<String>,
) -> Result<(String, String), IpAddressLookupError> {
    let ping_cmd = Command::new("ping")
        .args(&["-c1", "-q", "-4", &ip_address])
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|_| IpAddressLookupError::new(&ip_address, "failed to spawn ping command"))?;

    let ping_op = ping_cmd
        .wait_with_output()
        .await
        .map_err(|x| IpAddressLookupError::new(&ip_address, &x.to_string()))?;

    if ping_op.status.success() {
        let nslookup_cmd = Command::new("nslookup")
            .args(&[&ip_address, &dns_ip.unwrap_or_default()])
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|_| {
                IpAddressLookupError::new(&ip_address, "failed to run nslookup command")
            })?;

        let nslookup_op = nslookup_cmd
            .wait_with_output()
            .await
            .map_err(|x| IpAddressLookupError::new(&ip_address, &x.to_string()))?;

        if nslookup_op.status.success() {
            let p = nslookup_op
                .stdout
                .lines()
                .next()
                .expect("no output was produced from nslookup command")
                .map_err(|x| IpAddressLookupError::new(&ip_address, &x.to_string()))?;
            return Result::Ok((p, ip_address.to_owned()));
        } else {
            return Result::Ok((ip_address.to_owned(), ip_address.to_owned()));
        }
    } else {
        return Err(IpAddressLookupError::new(
            &ip_address,
            "unknow error executing ping",
        ));
    }
}
