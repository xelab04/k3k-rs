use anyhow::{anyhow, Result};
// use base64::{engine::general_purpose, Engine as _};
use k8s_openapi::api::core::v1::Secret;
use kube::{Api, Client};

pub async fn get(
    client: &Client,
    cluster_name: &str,
    namespace: Option<&str>,
) -> Result<String> {

    let ns = namespace
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("k3k-{}", cluster_name));

    let secret_name = format!("k3k-{}-kubeconfig", cluster_name);

    let api: Api<Secret> = Api::namespaced(client.clone(), &ns);
    let secret = api.get(&secret_name).await?;

    let data = secret
        .data
        .as_ref()
        .ok_or_else(|| anyhow!("Malformed secret: missing data field"))?;

    let key = "kubeconfig.yaml";
    let value = data
        .get(key)
        .ok_or_else(|| anyhow!("Secret field `{key}` not found"))?;

    // Note: value.0 already contains decoded bytes (not base64)
    let kubeconfig = String::from_utf8(value.0.clone())?;

    Ok(kubeconfig)
}
