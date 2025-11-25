use kube::{Api, Client, ResourceExt, Error as KubeError };
use kube::error::ErrorResponse;

use kube::api::{PostParams, DeleteParams};
use k8s_openapi::api::core::v1::Namespace;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

use crate::namespace;
// use crate::namespace::Namespace;

pub async fn create_easy(client: &Client, name: &str) -> anyhow::Result<()> {
    let api: Api<Namespace> = Api::all(client.clone());

    let pp = PostParams::default();
    let data = Namespace {
        metadata: ObjectMeta { name: Some(name.to_string()), ..Default::default() },
        ..Default::default()
    };
    api.create(&pp, &data).await?;

    Ok(())
}

pub async fn create(client: &Client, namespace: &Namespace) -> anyhow::Result<()> {
    let api: Api<Namespace> = Api::all(client.clone());

    let pp = PostParams::default();
    api.create(&pp, &namespace).await?;

    Ok(())
}

pub async fn delete(client: &Client, name: &str) -> anyhow::Result<()> {
    let api: Api<Namespace> = Api::all(client.clone());

    let dp = DeleteParams::default();
    api.delete(name, &dp).await?;

    Ok(())
}

pub async fn exists(client: &Client, namespace: &str) -> Result<bool, anyhow::Error> {

    let ns_api: Api<Namespace> = Api::all(client.clone());
    match ns_api.get(namespace).await {
        Ok(ns) => {
            // if a namespace exists, it must have a name
            println!("Namespace found: {}", ns.metadata.name.unwrap());
            return Ok(true);
        }
        Err(KubeError::Api(error_response)) if error_response.code == 404 => {
            return Ok(false);
        }
        Err(err) => {
            return Err(err.into());
        }
    };

}
