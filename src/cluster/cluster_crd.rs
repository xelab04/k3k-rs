use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::cluster::extra_specs::*;

mod defaults {
    pub fn cluster_type() -> String { String::from("shared")}
    pub const fn zero() -> i32 { 0 }
    pub fn one() -> i32 { 1 }
    pub fn empty_vec<T>() -> Vec<T> { Vec::new() }
    pub fn empty_obj() -> String { String::from("{}") }
    pub fn empty_str() -> String { String::new() }
    pub fn cluster_dns() -> String { String::from("10.43.0.10")}
    pub fn service_cidr() -> String { String::from("10.43.0.0/16")}
    pub fn cluster_cidr() -> String { String::from("10.42.0.0/16")}

}

/// Represents a k3k cluster
#[derive(CustomResource, Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[kube(
    group = "k3k.io",
    version = "v1beta1",
    kind = "Cluster",
    plural = "clusters",
    namespaced
)]

#[kube(status = "ClusterStatus")]
pub struct ClusterSpec {

    #[serde(default)]
    pub addons: Vec<SecretSpec>,
    #[serde(default)]
    pub agentAffinity: Option<AffinitySpec>,
    #[serde(default)]
    pub agentArgs: Option<Vec<String>>,
    #[serde(default)]
    pub agentEnvs: Option<Vec<EnvVar>>,
    // spec.agents => >= 0 (not needed in shared)
    #[serde(default = "defaults::zero")]
    pub agents: i32,
    #[serde(default = "defaults::cluster_cidr")]
    pub clusterCIDR: String,
    #[serde(default = "defaults::cluster_dns")]
    pub clusterDNS: String,
    #[serde(default)]
    pub customCAs: Option<CustomCASpec>,
    #[serde(default)]
    pub expose: Option<ExposeSpec>,
    #[serde(default)]
    pub hostUsers: Option<bool>,
    #[serde(default)]
    pub mirrorHostNodes: Option<bool>,
    // spec.mode => shared | virtual
    #[serde(default = "defaults::cluster_type")]
    pub mode: String,
    #[serde(default)]
    pub nodeSelector: Option<BTreeMap<String, String>>,
    #[serde(default)]
    pub persistence: Option<PersistenceSpec>,
    #[serde(default)]
    pub priorityClass: Option<String>,
    #[serde(default)]
    pub runtimeClassName: Option<String>,
    #[serde(default)]
    pub secretMounts: Option<Vec<SecretMount>>,
    #[serde(default)]
    pub securityContext: Option<SecurityContextSpec>,
    #[serde(default)]
    pub serverAffinity: Option<AffinitySpec>,
    #[serde(default)]
    pub serverArgs: Option<Vec<String>>,
    #[serde(default)]
    pub serverEnvs: Option<Vec<EnvVar>>,
    #[serde(default)]
    pub serverLimit: Option<BTreeMap<String, IntOrString>>,
    #[serde(default)]
    pub serverResources: Option<ResourcesSpec>,
    #[serde(default = "defaults::one")]
    pub servers: i32,
    #[serde(default = "defaults::service_cidr")]
    pub serviceCIDR: String,
    #[serde(default)]
    pub sync: Option<SyncSpec>,
    #[serde(default)]
    pub tlsSANs: Option<Vec<String>>,
    #[serde(default)]
    pub tokenSecretRef: Option<TokenSecretRefSpec>,
    #[serde(default = "defaults::empty_str")]
    pub version: String,
    #[serde(default)]
    pub workerLimit: Option<BTreeMap<String, IntOrString>>,
    #[serde(default)]
    pub workerResources: Option<ResourcesSpec>,

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
                storageClasses: Some(SyncResourceSpec {
                    enabled: (true),
                    selector: None,
                })
            }),

            nodeSelector: None,
            serverEnvs: None,
            agentEnvs: None,
            agentArgs: None,
            serverArgs: None,
            clusterCIDR: defaults::cluster_cidr(),
            clusterDNS: defaults::cluster_dns(),
            priorityClass: None,
            serviceCIDR: defaults::service_cidr(),
            tokenSecretRef: None,
            serverLimit: None,
            workerLimit: None,

            addons: Vec::new(),
            agentAffinity: None,
            customCAs: None,
            hostUsers: None,
            runtimeClassName: None,
            secretMounts: None,
            securityContext: None,
            serverAffinity: None,
            serverResources: None,
            workerResources: None,
        }
    }
}
