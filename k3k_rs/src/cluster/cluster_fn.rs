use kube::{Api, Client, ResourceExt};
use crate::cluster::Cluster;

pub async fn list(client: &Client, namespace: &str) -> anyhow::Result<Vec<Cluster>> {
    let api: Api<Cluster> = Api::namespaced(client.clone(), namespace);
    let list = api.list(&Default::default()).await?;
    Ok(list.items)
}

pub async fn get(client: &Client, namespace: &str, name: &str) -> anyhow::Result<Cluster> {
    let api: Api<Cluster> = Api::namespaced(client.clone(), namespace);
    let obj = api.get(name).await?;
    Ok(obj)
}
