use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

mod defaults {
    pub fn cluster_type() -> String { String::from("shared")}
    pub const fn zero() -> i32 { 0 }
    pub fn one() -> i32 { 1 }
    pub fn empty_vec<T>() -> Vec<T> { Vec::new() }
    pub fn empty_obj() -> String { String::from("{}") }
    pub fn empty_str() -> String { String::new() }
}

/// Represents a k3k cluster
#[derive(CustomResource, Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[kube(
    group = "k3k.io",
    version = "v1alpha1",
    kind = "Cluster",
    plural = "clusters",
    namespaced
)]

#[kube(status = "ClusterStatus")]
pub struct ClusterSpec {
    // spec.mode => shared | virtual
    #[serde(default = "defaults::cluster_type")]
    pub mode: String,

    // spec.servers => >= 1
    #[serde(default = "defaults::one")]
    pub servers: i32,

    // spec.agents => >= 0 (not needed in shared)
    #[serde(default = "defaults::zero")]
    pub agents: i32,

    #[serde(default = "defaults::empty_str")]
    pub version: String,

    #[serde(default)]
    pub nodeSelector: Option<BTreeMap<String, String>>,
    #[serde(default)]
    pub persistence: Option<PersistenceSpec>,
    #[serde(default)]
    pub expose: Option<ExposeSpec>,
    #[serde(default)]
    pub serverEnvs: Option<Vec<EnvVar>>,
    #[serde(default)]
    pub serverArgs: Option<Vec<String>>,
    #[serde(default)]
    pub serverLimit: Option<std::collections::BTreeMap<String, IntOrString>>,
    #[serde(default)]
    pub agentEnvs: Option<Vec<EnvVar>>,
    #[serde(default)]
    pub agentArgs: Option<Vec<String>>,
    #[serde(default)]
    pub workerLimit: Option<std::collections::BTreeMap<String, IntOrString>>,
    #[serde(default)]
    pub tlsSANs: Option<Vec<String>>,
    #[serde(default)]
    pub clusterCIDR: Option<String>,
    #[serde(default)]
    pub clusterDNS: Option<String>,
    #[serde(default)]
    pub priorityClass: Option<String>,
    #[serde(default)]
    pub serviceCIDR: Option<String>,
    #[serde(default)]
    pub tokenSecretRef: Option<TokenSecretRefSpec>,
    #[serde(default)]
    pub mirrorHostNodes: Option<bool>,
    #[serde(default)]
    pub sync: Option<SyncSpec>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct SyncSpec {
    #[serde(default)]
    pub configmaps: Option<SyncResourceSpec>,
    #[serde(default)]
    pub ingresses: Option<SyncResourceSpec>,
    #[serde(default)]
    pub persistentVolumeClaims: Option<SyncResourceSpec>,
    #[serde(default)]
    pub priorityClasses: Option<SyncResourceSpec>,
    #[serde(default)]
    pub secrets: Option<SyncResourceSpec>,
    #[serde(default)]
    pub services: Option<SyncResourceSpec>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct SyncResourceSpec {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub selector: Option<BTreeMap<String, String>,>
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct PersistenceSpec {
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub storageClassName: Option<String>,
    #[serde(default)]
    pub storageRequestSize: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct ExposeSpec {
    #[serde(default)]
    pub LoadBalancer: Option<ExposeLoadBalancer>,
    #[serde(default)]
    pub NodePort: Option<ExposeNodePort>,
    #[serde(default)]
    pub Ingress: Option<ExposeIngress>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct ExposeLoadBalancer {
    #[serde(default)]
    pub server_port: Option<i32>,
    #[serde(default)]
    pub etcd_port: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct ExposeNodePort {
    #[serde(default)]
    pub server_port: Option<i32>,
    #[serde(default)]
    pub etcd_port: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct ExposeIngress {
    #[serde(default)]
    pub annotations: Option<BTreeMap<String, String>>,
    #[serde(default)]
    pub ingressClassName: Option<String>,
}

/// Basic environment variable type
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct EnvVar {
    pub name: String,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct TokenSecretRefSpec {
    pub name: String,
    #[serde(default)]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct ClusterStatus {
    pub hostVersion: Option<String>,
    pub clusterCIDR: Option<String>,
    pub serviceCIDR: Option<String>,
    pub clusterDNS: Option<String>,
    pub persistence: Option<PersistenceSpec>,
    pub tlsSANs: Vec<String>,
}


impl Default for ClusterSpec {
    fn default() -> Self {
        ClusterSpec {
            agents: 0,
            expose: None,
            mode: String::from("shared"),
            persistence: Some(PersistenceSpec {
                r#type: Some("dynamic".to_string()),
                storageClassName: None,
                storageRequestSize: Some("2G".to_string()),
            }),
            servers: 1,
            tlsSANs: None,
            version: String::new(),
            mirrorHostNodes: Some(false),
            sync: Some(SyncSpec {
                configmaps: Some(SyncResourceSpec {
                    enabled: true,
                    selector: None,
                }),
                ingresses: Some(SyncResourceSpec {
                    enabled: false,
                    selector: None,
                }),
                persistentVolumeClaims: Some(SyncResourceSpec {
                    enabled: true,
                    selector: None,
                }),
                priorityClasses: Some(SyncResourceSpec {
                    enabled: false,
                    selector: None,
                }),
                secrets: Some(SyncResourceSpec {
                    enabled: true,
                    selector: None,
                }),
                services: Some(SyncResourceSpec {
                    enabled: true,
                    selector: None,
                }),
            }),

            nodeSelector: None,
            serverEnvs: None,
            agentEnvs: None,
            agentArgs: None,
            serverArgs: None,
            clusterCIDR: None,
            clusterDNS: None,
            priorityClass: None,
            serviceCIDR: None,
            tokenSecretRef: None,
            serverLimit: None,
            workerLimit: None,
        }
    }
}
