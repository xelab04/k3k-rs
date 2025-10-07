
# K3k-rs

This is a Rust crate built on top of [kube-rs](https://kube.rs/) to simplify handling (K3k)[https://github.com/rancher/k3k] virtual clusters. The hope is to avoid you having to re-implement the K3k CRDs from scratch. Additionally, there are a few convenient defaults so even cluster creation is a fast and easy process!

## Contributing

Just check the issues, leave a comment asking to address it, then contribute!


## Usage/Examples
Refer to the examples directory to see more examples.

#### List k3k clusters
```rust
let client = Client::try_default().await?;
let list: Vec<cluster::Cluster> = cluster::list::namespaced(&client, "k3k-namespace").await?;
```

#### Create cluster
```rust
let client = Client::try_default().await?;

let cluster_schema = k3k_rs::cluster::Cluster {
    metadata: kube::core::ObjectMeta {
        name: Some("test-cluster".to_string()),
        namespace: Some("k3k-namespace".to_string()),
        ..Default::default()
    },
    spec: ClusterSpec {
        // servers: 1, (default)
        // agents: 0, (default)
        // mode: "shared".to_string(), (default)
        // persistence: Some(PersistenceSpec {
        //     r#type: Some("dynamic".to_string()),
        //     storage_class_name: None,
        //     storage_request_size: Some("1G".to_string()),
        // }),
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

cluster::create(&client, namespace, &cluster_schema).await?;
```
