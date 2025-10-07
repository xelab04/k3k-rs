use kube::{Api, Client};
use k3k_rs::cluster;
use kube::api::ListParams;
use serde_yaml;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let client = Client::try_default().await?;
    let list = cluster::list::namespaced(&client, "k3k-namespace").await?;

    for c in list {
        let yaml = serde_yaml::to_string(&c)?;
        println!("---\n{}", yaml);
    }

    Ok(())
}
