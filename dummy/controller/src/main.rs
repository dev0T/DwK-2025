use anyhow::{Result, anyhow};
use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::core::v1::{Container, EnvVar, Pod, PodSpec};
use kube::{
    Client, CustomResource, Resource, ResourceExt, api::{Api, DeleteParams, ObjectMeta, PostParams, WatchEvent, WatchParams}, runtime::{conditions::{is_pod_running}, wait::{await_condition, delete::delete_and_finalize}}
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::*;

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(group = "stable.dwk", version = "v1", kind = "DummySite", namespaced)]
pub struct DummySiteSpec {
    pub website_url: String,
    pub image: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let client = Client::try_default().await?;

    let dummysites = Api::<DummySite>::default_namespaced(client.clone());
    let pods: Api<Pod> = Api::default_namespaced(client.clone());

    let mut stream = dummysites
        .watch(&WatchParams::default(), "0")
        .await?
        .boxed();

    while let Some(event) = stream.try_next().await? {
        match event {
            WatchEvent::Added(ds) => {
                let res = handle_added(pods.clone(), ds.clone()).await;
                match res {
                    Ok(_) => info!("Added: {}.", ds.name_any()),
                    Err(e) => info!("Error when handling Added event. {}", e),
                }
            }
            WatchEvent::Modified(ds) => {
                let res = handle_modified(pods.clone(), ds.clone()).await;
                match res {
                    Ok(_) => info!("Modified: {}", ds.name_any()),
                    Err(e) => info!("Error when handling Modified event. {}", e),
                }
            }
            WatchEvent::Deleted(ds) => {
                let res = handle_deleted(pods.clone(), ds.clone()).await;
                match res {
                    Ok(_) => info!("Deleted {}", ds.name_any()),
                    Err(e) => info!("Error when handling Deleted event. {}", e),
                }
            }
            WatchEvent::Bookmark(ds) => info!("Bookmark: {:?}", ds.types),
            WatchEvent::Error(ds) => info!("{}", ds),
        }
    }
    Ok(())
}

async fn deploy_pod(pods: Api<Pod>, ds: DummySite) -> Result<()> {
    let dummysite_name = ds.name_any();
    let pod_name = format!("{}-pod", dummysite_name);
    let p = Pod::default();
    let controller_ref = p.controller_owner_ref(&());

    let new_pod = Pod {
        metadata: ObjectMeta {
        name: Some(pod_name),
        owner_references: Some(controller_ref.into_iter().collect()),
        ..ObjectMeta::default()
        },
        spec: Some(PodSpec {
            containers: vec![Container {
                name: dummysite_name,
                image: Some(ds.spec.image),
                env: Some( vec![EnvVar {
                    name: "WEBSITE_URL".to_string(),
                    value: Some(ds.spec.website_url),
                    ..EnvVar::default()
                }]),
                ..Container::default()
            }],
            ..PodSpec::default()
        }),
        ..Pod::default()
    };

    match pods.create(&PostParams::default(), &new_pod).await {
        Ok(o) => {
            let name = o.name_any();
            assert_eq!(new_pod.name_any(), name);
            let establish = await_condition(pods.clone(), &name, is_pod_running());
            let _ = tokio::time::timeout(std::time::Duration::from_secs(15), establish).await?;
            info!("Created {}", name);
            Ok(())
        }
        Err(e) => Err(anyhow!("Failed to create pod: {}", e)),
    }
}

async fn remove_pod(pods: Api<Pod>, pod_name: String) -> Result<()> {


    let result = delete_and_finalize(pods.clone(), &pod_name, &DeleteParams::default()).await;

    match result {
        Ok(_) => {info!("Pod removed");
        Ok(())
    }
        Err(e) => Err(anyhow!("Failed to remove pod: {}", e)),
    }
}

async fn handle_added(pods: Api<Pod>, ds: DummySite) -> Result<()> {
    info!("Handling Added event");

    match pod_exists(pods.clone(), ds.clone()).await {
        Some(_) => {
            info!("Pod already present in cluster.");
            Ok(())},
        None => deploy_pod(pods.clone(), ds.clone()).await,
    }
}

async fn handle_modified(pods: Api<Pod>, ds: DummySite) -> Result<()> {
    info!("Handling Modified event");

    match pod_exists(pods.clone(), ds.clone()).await {
        Some(_) => replace_pod(pods, ds).await,
        None => Err(anyhow!("Failed to unable to find pod to modify.")),
    }
}

async fn handle_deleted(pods: Api<Pod>, ds: DummySite) -> Result<()> {
    info!("Handling Deleted event");

    match pod_exists(pods.clone(), ds.clone()).await {
        Some(pod) => remove_pod(pods, pod.name_any()).await,
        None => Err(anyhow!("Failed to unable to find pod to modify.")),
    }
}

async fn pod_exists(pods: Api<Pod>, ds: DummySite) -> Option<Pod> {
    let dummysite_name = ds.name_any();
    let pod_name = format!("{}-pod", dummysite_name).to_string();
    let pod = pods.get(&pod_name).await;
    match pod {
        Ok(pod) => Some(pod),
        Err(_) => None,
    }
}

async fn replace_pod(pods: Api<Pod>, ds: DummySite) -> Result<()> {
    info!("Replacing pod");
    let dummysite_name = ds.name_any();
    let pod_name = format!("{}-pod", dummysite_name).to_string();

     let result = remove_pod(pods.clone(), pod_name).await;

    match result {
        Ok(_) => deploy_pod(pods.clone(), ds.clone()).await,
        Err(e) => Err(anyhow!("Failed to unable to find pod to modify. {}", e)),
    }
}
