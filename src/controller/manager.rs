use std::thread;
// for k8s
use kube::{api::{Api, ListParams, ResourceExt, WatchEvent}, Client, runtime::{reflector, utils::try_flatten_applied, watcher}};
use k8s_openapi::{api::{core::v1::{Pod, Node, Service}, apps::v1::{Deployment, StatefulSet}}};
use futures::{StreamExt, TryStreamExt, FutureExt};
// for tokio
use tokio::sync::mpsc;

use tokio::time::sleep;

use log::{debug, info, error, warn};

use eyre::Result;
use tokio::runtime::Runtime;
use std::option::Option::Some;
use std::any::Any;
use tokio::sync::mpsc::{Sender, Receiver};
use crate::storage::{MemoryShareStorage, Bucket, Resource, KubeNodeStatus};


/// Manager a kubernetes controller use to monitor kube resource
pub struct Manager {
    // kube_client: KubeClient,
    kube_client: Client,
    // sender_channel: mpsc::Sender<>, 
    // rx: tokio::sync::mpsc::Receiver<Box<dyn Any>>,
    storage: MemoryShareStorage,

    // for k8s resources 
    pod_api: Api<Pod>,          // pod api used to watch pod resource changed
    deployment_api: Api<Deployment>,    //   used to watch deployment resouce status
    node_api: Api<Node>,
    // service_api: Api<Service>,     // statefulset_api

}

impl Manager {
    pub async fn new(db : MemoryShareStorage) ->Self{

        let kube_client = Client::try_default().await.expect("Err: Create Client failed");
        let pod_api = Api::<Pod>::all(kube_client.clone());
        let dpl_api  = Api::<Deployment>::all(kube_client.clone());
        let node_api = Api::<Node>::all(kube_client.clone());
        // let (tx, rx) = mpsc::channel(100);
        let (_tx, rx)= mpsc::channel::<String>(1000);
        Manager{kube_client,pod_api, deployment_api:dpl_api,node_api, storage: db}
    }

    pub async fn run(&self) ->Result<()> {
        let pod_informer = self.run_pod_informer();
        let dpl_informer  = self.run_deployment_informer();
        let node_informer = self.run_node_status_informer();
        tokio::join!(pod_informer, dpl_informer, node_informer);
        Ok(())
    }

    pub async fn run_pod_informer(& self)->Result<()>{
        try_flatten_applied(watcher(self.pod_api.clone(), ListParams::default()))
            .try_for_each(|p| async move {
                let mut bucket = self.storage[0].lock().await;
                info!("pod {}", p.name());
                Ok(())
            }).await?;
        Ok(())
    }

    pub async fn run_deployment_informer(& self)-> Result<()>{
        try_flatten_applied(watcher(self.deployment_api.clone(), ListParams::default()))
            .try_for_each(|p| async move {
                let mut bucket = self.storage[1].lock().await;
                info!("deployment {}", p.name());
                Ok(())
            })
            .await;
        Ok(())
    }

    pub async fn run_node_status_informer(&self)-> Result<()>{
        try_flatten_applied(watcher(self.node_api.clone(), ListParams::default()))
            .try_for_each(|node| async move {
                let mut bucket = self.storage[2].lock().await;
                bucket.insert(node.name(), Resource::KubeNode(KubeNodeStatus::from(node)));
                Ok(())
            })
            .await;
        Ok(())
    }


}

