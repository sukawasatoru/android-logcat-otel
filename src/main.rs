use android_logcat_otel::feature::init_otel;
use android_logcat_otel::model::LogcatLine;
use android_logcat_otel::prelude::*;
use clap::Parser;
use reqwest::Client;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::signal::ctrl_c;
use tokio::sync::oneshot::error::TryRecvError;
use tokio::task::yield_now;
use url::Url;

#[derive(Parser)]
struct Opt {
    /// OTel logs endpoint URL.
    #[clap(long)]
    logs_endpoint: Url,
}

#[tokio::main]
async fn main() -> Fallible<()> {
    let opt = Opt::parse();

    let _otel_guards = init_otel(Client::new(), opt.logs_endpoint)?;
    // tracing_subscriber::fmt::init();

    let (tx, mut rx) = tokio::sync::oneshot::channel();
    let adb_task = tokio::spawn(async move {
        let mut child = tokio::process::Command::new("adb")
            .args(["logcat", "-v", "epoch,uid"])
            .stdout(std::process::Stdio::piped())
            .spawn()
            .context("failed to execute adb command")
            .unwrap();

        let mut stdout = BufReader::new(child.stdout.take().expect("child stdout"));
        let mut buf = String::new();
        loop {
            if let Err(TryRecvError::Closed) = rx.try_recv() {
                debug!("end adb process");
                break;
            }

            buf.clear();
            stdout.read_line(&mut buf).await.expect("read line");
            if buf.is_empty() {
                match child.try_wait() {
                    Ok(Some(data)) => {
                        warn!(?data, "adb already exited");
                        debug!("end adb thread");
                        return;
                    }
                    Ok(None) => {
                        debug!("continue");
                        continue;
                    }
                    Err(e) => {
                        error!(?e, "error attempting to wait");
                        break;
                    }
                }
            }
            let line = match buf.parse::<LogcatLine>() {
                Ok(data) => data,
                Err(_) => {
                    debug!(line = buf, "failed to parse line");
                    continue;
                }
            };
            info!(
                event.name = "device.app.logcat",
                timestamp = line.timestamp,
                uid = line.uid,
                pid = line.pid,
                tid = line.tid,
                level = line.level,
                tag = line.tag,
                "{}",
                line.msg,
            );
            yield_now().await;
        }
        child.kill().await.unwrap();
        debug!("end adb thread");
    });

    tokio::select! {
        result = adb_task => {
            result?;
        },
        _ = ctrl_c() => {
            tx.send(true).ok();
        },
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verity_cli() {
        Opt::command().debug_assert();
    }
}
