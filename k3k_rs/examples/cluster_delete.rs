use kube::{Client};
use k3k_rs::cluster;

use k3k_rs::namespace;


#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let client = Client::try_default().await?;
    cluster::delete(&client, "k3k-namespace", "k3k-test").await?;

    // optional cleanup
    // namespace::delete(&client, "k3k-namespace").await?;

    Ok(())
}
