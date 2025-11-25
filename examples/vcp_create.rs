use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use std::collections::BTreeMap;
use serde_yaml;
use kube::{Client};
use k3k_rs;
use k3k_rs::virtualclusterpolicy::{
    VirtualClusterPolicy, VirtualClusterPolicySpec, SyncSpec,
    SyncResourceSpec, LimitSpec, LimitsSpec, QuotaSpec, ScopeSelectorSpec
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::try_default().await?;

    // Build virtual cluster policy
    let vcp_schema = VirtualClusterPolicy {
        metadata: kube::core::ObjectMeta {
            name: Some("test-vpc".to_string()),
            namespace: Some("k3k-default".to_string()),
            ..Default::default()
        },
        spec: VirtualClusterPolicySpec {
            allowedMode: "shared".to_string(),
            // defaultNodeSelector: Some("node-role.kubernetes.io/master".to_string()),


            defaultNodeSelector: Some(BTreeMap::from([
                ("kubernetes.io/arch".to_string(), "amd64".to_string()),
                ("nodepool".to_string(), "main".to_string()),
            ])),

            defaultPriorityClass: Some(String::from("default")),

            disableNetworkPolicy: Some(false),

            // for limits, please refer to the CRD implementation, there are many options
            limit: Some(LimitSpec {
                limits: vec![
                    LimitsSpec {
                        default: Some(BTreeMap::from([
                            ("cpu".to_string(), IntOrString::String("30m".into())),
                            ("memory".to_string(), IntOrString::String("256m".into())),
                            ("storage".to_string(), IntOrString::String("100Gi".into())),
                        ])),
                        ..Default::default()
                    }
                ],
            }),

            podSecurityAdmissionLevel: Some(String::from("restricted")),

            // also very very long with many options, check the CRD
            // quota:

            sync: Some(SyncSpec{
                ingresses: Some(SyncResourceSpec {
                    enabled: true,
                    selector: None,
                }),
                ..Default::default()
            }),

            ..Default::default()
        },
    };

    // optional: you can create the namespace manually
    // namespace::create_easy(&client, "k3k-default").await?;

    // or this will create the ns automatically if it doesn't exist anyways
    let response = k3k_rs::virtualclusterpolicy::create(&client, &vcp_schema, "k3k-default").await;

    let result;
    match response {
        Err(e) => {println!("Error creating cluster {}: {}", vcp_schema.metadata.name.unwrap(), e); return Ok(());}

        Ok(response) => {
            println!("Cluster created successfully");
            result = response;
        }
    }

    let yaml = serde_yaml::to_string(&result)?;
    println!("Cluster YAML:\n{}", yaml);
    return Ok(());
}
