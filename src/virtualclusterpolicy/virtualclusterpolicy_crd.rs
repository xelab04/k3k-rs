use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

mod defaults {
    pub const fn zero() -> i32 { 0 }
    pub fn defaultAllowedMode() -> String { String::from("shared") }

}

/// Represents a virtualclusterpolicy
#[derive(CustomResource, Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[kube(
    group = "k3k.io",
    version = "v1alpha1",
    kind = "VirtualClusterPolicy",
    plural = "virtualclusterpolicies",
    namespaced
)]
// #[kube(status = "VirtualClusterPolicyStatus")]
pub struct VirtualClusterPolicySpec {

    #[serde(default = "defaults::defaultAllowedMode")]
    pub allowedMode: String,

    #[serde(default)]
    pub defaultNodeSelector: Option<BTreeMap<String, String>>,

    #[serde(default)]
    pub defaultPriorityClass: Option<String>,

    #[serde(default)]
    pub disableNetworkPolicy: Option<bool>,

    #[serde(default)]
    pub limit: Option<LimitSpec>,

    #[serde(default)]
    pub podSecurityAdmissionLevel: Option<String>,

    #[serde(default)]
    pub quota: Option<QuotaSpec>,

    #[serde(default)]
    pub sync: Option<SyncSpec>,

}


#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct LimitSpec {
    pub limits: Vec<LimitsSpec>
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct LimitsSpec {
    pub default: Option<BTreeMap<String, IntOrString>>,
    pub defaultRequest: Option<BTreeMap<String, IntOrString>>,
    pub max: Option<BTreeMap<String, IntOrString>>,
    pub maxLimitRequestRatio: Option<BTreeMap<String, IntOrString>>,
    pub min: Option<BTreeMap<String, IntOrString>>,
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct QuotaSpec {
    #[serde(default)]
    pub hard: Option<BTreeMap<String, IntOrString>>,
    #[serde(default)]
    pub scopeSelector: Option<Vec<ScopeSelectorSpec>>,
    #[serde(default)]
    pub scopes: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct ScopeSelectorSpec {
    #[serde(default)]
    pub operator: Option<String>,
    #[serde(default)]
    pub scopeName: Option<String>,
    #[serde(default)]
    pub values: Option<Vec<String>>,
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


impl Default for VirtualClusterPolicySpec {

    fn default() -> Self {
        VirtualClusterPolicySpec {
            allowedMode: "shared".to_string(),
            defaultNodeSelector: None,
            defaultPriorityClass: None,
            disableNetworkPolicy: None,
            limit: None,
            podSecurityAdmissionLevel: None,
            quota: None,
            sync: Some(SyncSpec {
                configmaps: Some(SyncResourceSpec{enabled: true, selector: None}),
                ingresses: Some(SyncResourceSpec{enabled: false, selector: None}),
                persistentVolumeClaims: Some(SyncResourceSpec{enabled: true, selector: None}),
                priorityClasses: Some(SyncResourceSpec{enabled: false, selector: None}),
                secrets: Some(SyncResourceSpec{enabled: true, selector: None}),
                services: Some(SyncResourceSpec{enabled: true, selector: None})
            })
        }
    }

}
