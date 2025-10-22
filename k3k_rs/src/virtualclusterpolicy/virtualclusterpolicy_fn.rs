use crate::virtualclusterpolicy;
use crate::virtualclusterpolicy::VirtualClusterPolicy;
use crate::namespace;
use k8s_openapi::api::core::v1::Namespace;
use kube::api::{DeleteParams, PostParams};
use kube::{Api, Client, Error as KubeError};


pub mod list {
    use super::*;

    pub async fn namespaced(client: &Client, namespace: &str) -> anyhow::Result<Vec<VirtualClusterPolicy>> {
        super::vcp_list(client, Some(namespace)).await
    }
    pub async fn all(client: &Client) -> anyhow::Result<Vec<VirtualClusterPolicy>> {
        super::vcp_list(client, None).await
    }
}

pub async fn vcp_list(
    client: &Client,
    namespace: Option<&str>,
) -> anyhow::Result<Vec<VirtualClusterPolicy>> {

    let api: Api<VirtualClusterPolicy>;
    if let Some(namespace) = namespace {
        api = Api::namespaced(client.clone(), namespace);
    } else {
        api = Api::all(client.clone());
    }

    let list = api.list(&Default::default()).await?;
    Ok(list.items)
}

pub async fn get(client: &Client, namespace: &str, name: &str) -> anyhow::Result<VirtualClusterPolicy> {
    let api: Api<VirtualClusterPolicy> = Api::namespaced(client.clone(), namespace);
    let obj = api.get(name).await?;
    Ok(obj)
}

pub async fn create(
    client: &Client,
    namespace: &str,
    vcp: &VirtualClusterPolicy,
) -> anyhow::Result<VirtualClusterPolicy> {

    let ns_api: Api<Namespace> = Api::all(client.clone());
    match ns_api.get(namespace).await {
        Ok(ns) => {
            println!("Namespace found: {}", ns.metadata.name.unwrap());
        }
        Err(KubeError::Api(error_response)) if error_response.code == 404 => {
            println!("Namespace not found: {}", error_response.message);
            namespace::create_easy(&client, &namespace).await?;
        }
        Err(err) => {
            println!("Unexpected error: {}", err);
        }
    };

    let api: Api<VirtualClusterPolicy> = Api::namespaced(client.clone(), namespace);
    let mut pp = PostParams::default();
    pp.dry_run = true;
    let obj = api.create(&pp, vcp).await?;
    Ok(obj)
}

pub async fn delete(client: &Client, namespace: &str, cluster_name: &str) -> anyhow::Result<()> {
    let api: Api<VirtualClusterPolicy> = Api::namespaced(client.clone(), namespace);
    let dp = DeleteParams::default();
    let obj = api.delete(cluster_name, &dp).await?;
    Ok(())
}
