use kube::Client;
use k3k_rs::kubeconfig;
use serde_yaml;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let client = Client::try_default().await?;

    // don't specify the namespace, and it will default to k3k-cluster-name
    let kconf = kubeconfig::get(&client, "meow", None).await?;

    println!("{}", serde_yaml::to_string(&kconf)?);

    Ok(())
}
