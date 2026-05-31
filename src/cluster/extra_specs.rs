use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
    #[serde(default)]
    pub storageClasses: Option<SyncResourceSpec>,
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
    pub value: String,
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
    pub tlsSANs: Option<Vec<String>>,
}

// New additions with 1.1.0

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct SecretSpec {
    #[serde(default)]
    pub secretRef: Option<String>,
    #[serde(default)]
    pub secretNamespace: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct AffinitySpec {
    #[serde(default)]
    pub nodeAffinity: Option<k8s_openapi::api::core::v1::NodeAffinity>,
    #[serde(default)]
    pub podAffinity: Option<k8s_openapi::api::core::v1::PodAffinity>,
    #[serde(default)]
    pub podAntiAffinity: Option<k8s_openapi::api::core::v1::PodAntiAffinity>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct CustomCASpec {
    pub enabled: bool,
    pub sources: CustomCASourcesSpec,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct CustomCASourcesSpec {
    pub clientCA: CASecretRef,
    pub etcdPeerCA: CASecretRef,
    pub etcdServerCA: CASecretRef,
    pub requestHeaderCA: CASecretRef,
    pub serverCA: CASecretRef,
    pub serviceAccountToken: CASecretRef,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct CASecretRef {
    pub secretName: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct SecretMountsSpec {
    #[serde(default)]
    pub mounts: Vec<SecretMount>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct SecretMount {
    #[serde(default)]
    pub secretName: Option<String>,
    #[serde(default)]
    pub mountPath: Option<String>,
    #[serde(default)]
    pub subPath: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub optional: Option<bool>,
    #[serde(default)]
    pub defaultMode: Option<i32>,
    #[serde(default)]
    pub items: Option<Vec<KeyToPath>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct KeyToPath {
    pub key: String,
    pub path: String,
    #[serde(default)]
    pub mode: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct SecurityContextSpec {
    #[serde(default)]
    pub privileged: Option<bool>,
    #[serde(default)]
    pub runAsUser: Option<i64>,
    #[serde(default)]
    pub runAsGroup: Option<i64>,
    #[serde(default)]
    pub runAsNonRoot: Option<bool>,
    #[serde(default)]
    pub readOnlyRootFilesystem: Option<bool>,
    #[serde(default)]
    pub allowPrivilegeEscalation: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct ResourcesSpec {
    #[serde(default)]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    #[serde(default)]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}
