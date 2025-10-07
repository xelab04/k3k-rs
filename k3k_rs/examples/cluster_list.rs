use kube::{Api, Client};
use k3k_rs::cluster::Cluster;
use kube::api::ListParams;
use serde_yaml;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to cluster (KUBECONFIG or in-cluster)
    let client = Client::try_default().await.unwrap();

    // Api for namespaced CRDs
    let clusters: Api<Cluster> = Api::namespaced(client, "k3k-meow"); // adjust namespace

    // List all Cluster objects
    let lp = ListParams::default();
    let list = clusters.list(&lp).await.unwrap();

    println!("Found {} clusters:", list.items.len());
    // for c in list.items {
    //     let name = c.metadata.name.unwrap_or_default();
    //     let servers = c.spec.servers;
    //     println!(" - {} ({} server(s))", name, servers);
    // }

    for c in list.items {
        let yaml = serde_yaml::to_string(&c)?;

        println!("---\n{}", yaml);
    }

    Ok(())
}
