use serde_yaml;
use kube::{Client};
use k3k_rs;
use k3k_rs::cluster::{
    Cluster, ClusterSpec, ExposeSpec, ExposeLoadBalancer, ExposeNodePort, ExposeIngress,
    PersistenceSpec, SyncSpec, SyncResourceSpec
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::try_default().await?;

    // Build Cluster object in memory
    let cluster_schema = Cluster {
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
                storageClassName: None,
                storageRequestSize: Some("1G".to_string()),
            }),
            expose: Some(ExposeSpec {
                LoadBalancer: Some(ExposeLoadBalancer {
                    etcd_port: Some(2379),
                    server_port: Some(443),
                }),
                NodePort: None,
                Ingress: None,
            }),
            sync: Some(SyncSpec{
                ingresses: Some(SyncResourceSpec {
                    enabled: true,
                    selector: None,
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
        status: None,
    };

    // optional: you can create the namespace manually
    // namespace::create_easy(&client, "k3k-default").await?;

    // or this will create the ns automatically if it doesn't exist anyways
    let response = k3k_rs::cluster::create(&client, "k3k-default", &cluster_schema).await;

    let result;
    match response {
        Err(e) => {println!("Error creating cluster {}: {}", cluster_schema.metadata.name.unwrap(), e); return Ok(());}

        Ok(response) => {
            println!("Cluster created successfully");
            result = response;
        }
    }

    let yaml = serde_yaml::to_string(&result)?;
    println!("Cluster YAML:\n{}", yaml);
    return Ok(());
}
