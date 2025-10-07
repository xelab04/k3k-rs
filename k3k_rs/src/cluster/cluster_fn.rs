use kube::{Api, Client, ResourceExt};
use kube::api::{PostParams};
use crate::cluster::Cluster;


pub mod list {
    use super::*;

    pub async fn namespaced(client: &Client, namespace: &str) -> anyhow::Result<Vec<Cluster>> {
        super::cluster_list(client, Some(namespace)).await
    }
    pub async fn all(client: &Client) -> anyhow::Result<Vec<Cluster>> {
        super::cluster_list(client, None).await
    }
}

pub async fn cluster_list(client: &Client, namespace: Option<&str>) -> anyhow::Result<Vec<Cluster>> {

    let api: Api<Cluster>;
    if let Some(namespace) = namespace {
        api = Api::namespaced(client.clone(), namespace);
    }
    else {
         api = Api::all(client.clone());
    }

    let list = api.list(&Default::default()).await?;
    Ok(list.items)
}


pub async fn get(client: &Client, namespace: &str, name: &str) -> anyhow::Result<Cluster> {
    let api: Api<Cluster> = Api::namespaced(client.clone(), namespace);
    let obj = api.get(name).await?;
    Ok(obj)
}

pub async fn create(client: &Client, namespace: &str, cluster: &Cluster) -> anyhow::Result<Cluster> {
    let api: Api<Cluster> = Api::namespaced(client.clone(), namespace);
    let pp = PostParams::default();
    let obj = api.create(&pp, cluster).await?;
    Ok(obj)
}
