use std::collections::BTreeMap;
use serde_yaml;
use kube::{Client};
use k3k_rs;
use k3k_rs::cluster::{
    Cluster, ClusterSpec, ExposeSpec, ExposeLoadBalancer, ExposeNodePort, ExposeIngress,
    PersistenceSpec, SyncSpec, SyncResourceSpec, EnvVar, TokenSecretRefSpec
};
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

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
            mode: "shared".to_string(),
            servers: 1,
            agents: 0,
            version: "v1.34.1+k3s1".to_string(),

            nodeSelector: Some(BTreeMap::from([
                ("kubernetes.io/arch".to_string(), "amd64".to_string()),
                ("nodepool".to_string(), "main".to_string()),
            ])),

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

            serverEnvs: Some(Vec::from([
                EnvVar{name: "TEST_ENV".to_string(), value: "test_value".to_string()}
            ])),

            serverArgs: Some(Vec::from(["--write-kubeconfig-mode=644".to_string()])),

            serverLimit: Some(BTreeMap::from([
                ("cpu".to_string(), IntOrString::String("2".into())),
                ("memory".to_string(), IntOrString::String("4Gi".into())),
                ("storage".to_string(), IntOrString::String("100Gi".into())),
            ])),

            // agentEnvs, agentArgs, workerLimit are the same as above

            tlsSANs: Some(Vec::from(["test.kraft.alexbissessur.dev".to_string()])),
            clusterCIDR: String::from("10.42.0.0/16"),
            clusterDNS: String::from("10.43.0.10"),
            priorityClass: Some(String::from("default")),
            serviceCIDR: String::from("10.43.0.0/16"),
            tokenSecretRef: Some(TokenSecretRefSpec {
                name: "my-secret-token".to_string(),
                namespace: Some("a-different-ns".to_string()),
            }),
            mirrorHostNodes: Some(false),

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
