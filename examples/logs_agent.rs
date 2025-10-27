use kube::Client;
use k3k_rs::logs;
use serde_yaml;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let client = Client::try_default().await.unwrap();

    let all_agent_pod_logs = logs::agent(&client, "meow", "k3k-meow").await;

    for (i, log) in all_agent_pod_logs.into_iter().enumerate() {
        println!("{}", i);
        println!("{}", log);
    }

    Ok(())
}
