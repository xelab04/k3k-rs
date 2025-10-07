use kube::{Api, Client, ResourceExt};
use k3k_rs::cluster::{
    Cluster, ClusterSpec, ExposeSpec, ExposeLoadBalancer, ExposeNodePort, ExposeIngress,
    PersistenceSpec,
};
use kube::api::{PostParams};
use serde_yaml;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to cluster (KUBECONFIG or in-cluster)
    let client = Client::try_default().await?;

    // Api for namespaced CRDs
    // let clusters: Api<Cluster> = Api::namespaced(client, "k3k-default"); // adjust namespace
    let clusters: Api<Cluster> = Api::all(client); // without namespace so i can deploy anywhere i guess


    // Build Cluster object in memory
    let cluster = Cluster {
        metadata: kube::core::ObjectMeta {
            name: Some("test-cluster".to_string()),
            namespace: Some("k3k-default".to_string()),
            ..Default::default()
        },
        spec: ClusterSpec {
            servers: 1,
            agents: 0,
            mode: "shared".to_string(),
            persistence: Some(PersistenceSpec {
                r#type: Some("dynamic".to_string()),
                storage_class_name: None,
                storage_request_size: Some("1G".to_string()),
            }),
            expose: Some(ExposeSpec {
                LoadBalancer: Some(ExposeLoadBalancer {
                    etcd_port: Some(2379),
                    server_port: Some(443),
                }),
                NodePort: None,
                Ingress: None,
            }),
            ..Default::default()
        },
        status: None,
    };

    // serialize to YAML for inspection
    let yaml = serde_yaml::to_string(&cluster)?;
    println!("Cluster YAML:\n{}", yaml);

    // create the Cluster in Kubernetes
    // let pp = PostParams::default();
    // let created = clusters.create(&pp, &cluster).await?;
    // println!("Created cluster: {}", created.name_any());

    Ok(())
}
