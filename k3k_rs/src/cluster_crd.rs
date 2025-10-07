use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    #[serde(default = "shared")]
    pub mode: String,

    // spec.servers => >= 1
    #[serde(default = "1")]
    pub servers: i32,

    // spec.agents => >= 0 (not needed in shared)
    #[serde(default = "0")]
    pub agents: Option<i32>,

    // spec.version
    #[serde(default = "")]
    pub version: Option<String>,

    /// spec.nodeSelector
    #[serde(default = "{}")]
    pub node_selector: Option<String>,

    // obligatory spec.persistence
    #[serde(default)]
    pub persistence: Option<PersistenceSpec>,

    // spec.expose
    #[serde(default)]
    pub expose: Option<ExposeSpec>,

    // spec.serverEnvs
    #[serde(default)]
    pub server_envs: Option<Vec<EnvVar>>,

    // spec.agentEnvs
    #[serde(default)]
    pub agent_envs: Option<Vec<EnvVar>>,

    // spec.tlsSANs
    #[serde(default)]
    pub tlsSANs: Option<Vec<String>>,

    // spec.agentArgs
    #[serde(default)]
    pub agentArgs: Option<Vec<String>>,

    // spec.serverArgs
    #[serde(default)]
    pub serverArgs: Option<Vec<String>>,

    // spec.clusterCIDR
    #[serde(default)]
    pub clusterCIDR: Option<String>,

    // spec.clusterDNS
    #[serde(default)]
    pub clusterDNS: Option<String>,

    // spec.priorityClass
    #[serde(default)]
    pub priorityClass: Option<String>,

    // spec.serviceCIDR
    #[serde(default)]
    pub serviceCIDR: Option<String>,

    // spec.tokenSecretRef
    #[serde(default)]
    pub tokenSecretRef: Option<TokenSecretRefSpec>,

    // spec.serverLimit
    #[serde(default)]
    pub serverLimit: Option<String>,

    // spec.serviceCIDR
    #[serde(default)]
    pub serviceCIDR: Option<String>,

    // wow these comments are useless
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct PersistenceSpec {
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub storage_class_name: Option<String>,
    #[serde(default)]
    pub storage_request_size: Option<String>,
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
    pub annotations: Option<String>,
    #[serde(default)]
    pub ingressClassName: Option<String>,
}

/// Basic environment variable type
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnvVar {
    pub name: String,
    #[serde(default)]
    pub value: Option<String>,
}

pub struct TokenSecretRefSpec {
    pub name: String,
    #[serde(default)]
    pub namespace: Option<String>,
}

/// Cluster status
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClusterStatus {
    pub host_version: Option<String>,
    pub cluster_cidr: Option<String>,
    pub service_cidr: Option<String>,
    pub cluster_dns: Option<String>,
    pub persistence: Option<PersistenceSpec>,
    #[serde(default)]
    pub tls_sans: Vec<String>,
}
