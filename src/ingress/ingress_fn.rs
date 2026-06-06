use k8s_openapi::api::networking::v1::{
    HTTPIngressPath, HTTPIngressRuleValue, Ingress, IngressBackend, IngressRule,
    IngressServiceBackend, IngressSpec, ServiceBackendPort,
};

use kube::{
    Client,
    api::{Api, ObjectMeta, PostParams},
    core::{ApiResource, GroupVersionKind},
};

pub async fn cluster_ingress(
    client: &Client,
    cluster_name: &str,
    namespace: &str,
    host: &str,
    ingress_class: &str,
) -> anyhow::Result<()> {
    let ingress_handler: Api<Ingress> = Api::namespaced(client.clone(), namespace);

    let ingress_schema = Ingress {
        metadata: ObjectMeta {
            name: Some(format!("{}-ingress", cluster_name)),
            namespace: Some(namespace.to_string()),
            ..Default::default()
        },
        spec: Some(IngressSpec {
            ingress_class_name: Some(String::from(ingress_class)),
            // default_backend: Some(IngressBackend{}),
            rules: Some(Vec::from([IngressRule {
                host: Some(String::from(host)),
                http: Some(HTTPIngressRuleValue {
                    paths: Vec::from([HTTPIngressPath {
                        backend: IngressBackend {
                            service: Some(IngressServiceBackend {
                                name: format!("{}-service", cluster_name),
                                port: Some(ServiceBackendPort {
                                    name: None,
                                    number: Some(443),
                                }),
                            }),
                            resource: None,
                        },
                        path: Some(String::from("/")),
                        path_type: String::from("Prefix"),
                    }]),
                }),
            }])),
            tls: None,
            default_backend: None,
        }),
        status: None,
    };

    let mut pp = PostParams::default();
    let obj = ingress_handler.create(&pp, &ingress_schema).await?;

    Ok(())
}
