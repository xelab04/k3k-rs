use kube::{Api, Client, Error as KubeError };
use kube::api::{PostParams, DeleteParams};
use k8s_openapi::api::core::v1::Namespace;
use crate::cluster::Cluster;
use crate::namespace;

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
    let ns_api: Api<Namespace> = Api::all(client.clone());
    match ns_api.get(namespace).await {
        Ok(ns) => {
            println!("Namespace found: {}", ns.metadata.name.unwrap());
        },
        Err(KubeError::Api(error_response)) if error_response.code == 404 => {
            println!("Namespace not found: {}", error_response.message);
            namespace::create_easy(&client, &namespace).await?;
        },
        Err(err) => {
            println!("Unexpected error: {}", err);
        },
    };

    let api: Api<Cluster> = Api::namespaced(client.clone(), namespace);
    let mut pp = PostParams::default();
    pp.dry_run = true;
    let obj = api.create(&pp, cluster).await?;
    Ok(obj)
}

pub async fn delete(client: &Client, namespace: &str, cluster_name: &str) -> anyhow::Result<()> {
    let api: Api<Cluster> = Api::namespaced(client.clone(), namespace);
    let dp = DeleteParams::default();
    let obj = api.delete(cluster_name, &dp).await?;
    Ok(())
}
