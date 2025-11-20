use k8s_openapi::api::core::v1::Pod;
use kube::api::{DeleteParams, PostParams, ListParams, LogParams};
use kube::{Api, Client, Error as KubeError};
use kube::ResourceExt;


pub async fn server(client: &Client, cluster_name: &str, namespace: &str, num_lines: i64) -> anyhow::Result<Vec<String>> {
    let labels = format!("cluster={},role=server", cluster_name);
    return logs(client, cluster_name, namespace, &labels, num_lines).await;
}

pub async fn agent(client: &Client, cluster_name: &str, namespace: &str, num_lines: i64) -> anyhow::Result<Vec<String>> {
    let labels = format!("cluster={},type=agent", cluster_name);
    return logs(client, cluster_name, namespace, &labels, num_lines).await;
}

async fn logs(
    client: &Client,
    cluster_name: &str,
    namespace: &str,
    labels: &str,
    num_lines: i64,
) -> anyhow::Result<Vec<String>> {

    let default_ns = format!("k3k-{}", cluster_name);

    let api: Api<Pod> = Api::namespaced(client.clone(), namespace);

    let lp = ListParams::default().labels(labels);

    let lgp;
    if num_lines > 0 {
        lgp = LogParams{
            tail_lines: Some(num_lines),
            ..Default::default()
        };
    } else {
        lgp = LogParams::default();
    }

    let mut logs_list = Vec::new();
    let mut pod_list;

    match api.list(&lp).await {
        Ok(pods) => pod_list = pods,
        Err(e) => return Err(anyhow::Error::new(e)),
    }

    for p in pod_list {
        // _or(String::new
        match api.logs(p.name_any().as_str(), &lgp).await {
            Ok(logs) => logs_list.push(logs.clone()),
            Err(e) => return Err(anyhow::Error::new(e)),
        }
    }

    Ok(logs_list)

}
