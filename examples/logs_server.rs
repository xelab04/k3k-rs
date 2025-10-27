use kube::Client;
use k3k_rs::logs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let client = Client::try_default().await.unwrap();

    let all_server_pod_logs = logs::server(&client, "meow", "k3k-meow").await;

    for (i, log) in all_server_pod_logs.into_iter().enumerate() {
        println!("{}", i);
        println!("{}", log);
    }

    Ok(())
}
