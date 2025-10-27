use k8s_openapi::api::core::v1::Pod;
use kube::api::{DeleteParams, PostParams, ListParams, LogParams};
use kube::{Api, Client, Error as KubeError};
use kube::ResourceExt;


pub async fn server(client: &Client, cluster_name: &str, namespace: &str) -> anyhow::Result<Vec<String>> {
    let labels = format!("cluster={},role=server", cluster_name);
    return logs(client, cluster_name, namespace, &labels).await;
}

pub async fn agent(client: &Client, cluster_name: &str, namespace: &str) -> anyhow::Result<Vec<String>> {
    let labels = format!("cluster={},type=agent", cluster_name);
    return logs(client, cluster_name, namespace, &labels).await;
}

async fn logs(
    client: &Client,
    cluster_name: &str,
    namespace: &str,
    labels: &str,
) -> anyhow::Result<Vec<String>> {

    let default_ns = format!("k3k-{}", cluster_name);

    let api: Api<Pod> = Api::namespaced(client.clone(), namespace);

    let lp = ListParams::default().labels(labels);
    let lgp = LogParams{
        tail_lines: Some(10),
        ..Default::default()
    };

    let mut logs_list = Vec::new();
    for p in api.list(&lp).await.unwrap() {
        // _or(String::new
        let logs = api.logs(p.name_any().as_str(), &lgp).await.unwrap();
        logs_list.push(logs.clone());
    }

    Ok(logs_list)

}
