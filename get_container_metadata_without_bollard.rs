use std::time::Instant;

use hyper::{Client, body::HttpBody as _};
use hyperlocal::{UnixClientExt, Uri};
use serde_json::Value;
use tokio;
use serde::{Deserialize, Serialize};

enum ContainerType {
    Docker,
}

enum ContainerMetadataSource {
    Socket
}
struct ContainerMetadata {
    /// container id
    pub id: String,
    /// Container Name
    pub name: String,
    /// Container Image full name
    pub image_full_name: String,
    /// container image digest
    pub image_digest: String,
    /// container type
    pub container_type: ContainerType,
    /// Pod id
    pub pod_id: String,
    /// Creation time of container metadata
    pub container_creation_time: u128,
    /// Task arn
    pub task_arn: String,

    /// Data source indicate where this information is from
    pub data_source: ContainerMetadataSource,
    /// Container creation time (unit: epoch in nano second)
    pub created_at: i64,
    /// container start time (unit: epoch in nano second)
    pub started_at: i64,

}
#[derive(Serialize, Deserialize, Debug)]
struct DockerImage {
    Id: String,
    RepoTags: Vec<String>,
    Size: u64,
    // ... include other fields as necessary
}
#[tokio::main]
async fn main() {
    // Path to the Unix domain socket
    let socket_path = "/var/run/docker.sock";

    // Create a Hyper client capable of connecting to Unix sockets
    let client = Client::unix();

    // container id 2dc428a70cdc177991e69fd71747dee237b05567833be7431a8d6a04db1f1974
    let container_id = "2dc428a70cdc177991e69fd71747dee237b05567833be7431a8d6a04db1f1974";
    let url = Uri::new(socket_path, &format!("/containers/{}/json", container_id)).into();

    // Make the request and await the response
    match client.get(url).await {
        Ok(res) => {
            // Read the response body and parse it as JSON
            let body_bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
            let container_details: Value = serde_json::from_slice(&body_bytes).unwrap();
            
            let name = if let Some(v) = container_details.get("Name") {
                if let Some(s) = v.as_str() {
                    s.to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            let image_digest = if let Some(v) = container_details.get("Image") {
                if let Some(s) = v.as_str() {
                    s.to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            };
            let image_digest = image_digest.trim_start_matches("sha256:").to_string();
            // if !image_digest.is_empty() {
            let image_url: Uri = Uri::new(socket_path, &format!("/images/{}/json", image_digest));
                match client.get(image_url.into()).await {
                    Ok(res) => {
                        let body_bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
                        let image_details: Value = serde_json::from_slice(&body_bytes).unwrap();
                        // TODO: these depends on how you want to utilize these image information
                    }
                    Err(err) => {
                        println!("Image {err:?}");
                    }

            }

            let created_str = if let Some(v) = container_details.get("Created") {
                if let Some(v) = v.as_str() {
                    v.to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            };
            println!("The container is created: {created_str}");

            let started_str = if let Some(v) = container_details.get("State") {
                if let Some(start_at_value) = v.get("StartedAt") {
                    if let Some(v) = start_at_value.as_str() {
                        v.to_string()
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            } else {
                String::new()
            };
            println!("The container is started: {started_str}");

        },
        Err(err) => eprintln!("Error: {}", err),
    }
}
