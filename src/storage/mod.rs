use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;
use k8s_openapi::api::core::v1::Node;
use kube::ResourceExt;
use std::fmt::Display;
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;

use log::{warn};


pub enum Bucket {
    Pod,    // 0
    Deployment, // 1
    Node,       // 2
    Service     // 3
}

pub enum Resource{
    KubeNode(KubeNodeStatus)
}

#[derive(Debug)]
pub struct KubeNodeStatus {
    pub node_name: String,
    pub capacity_cpu: String,
    pub capacity_mem: String,
    pub capacity_disk: String,
    pub external_ip: String,
    pub os_image: String,
    pub architecture: String,
}

impl From<Node> for KubeNodeStatus{
    fn from(node: Node) -> Self{
        let node_info = node.status.clone().unwrap().node_info.unwrap();
        let os_image = node_info.os_image;
        let architecture = node_info.architecture;

        let capacity = node.status.clone().unwrap().capacity.unwrap();
        let  capacity_cpu = capacity.get("cpu").unwrap().0.clone();
        let  capacity_mem = capacity.get("memory").unwrap().0.clone();
        let capacity_disk = capacity.get("ephemeral-storage").unwrap().0.clone();

        let address = node.status.clone().unwrap().addresses.unwrap();

        let mut external_ip = String::from("not found ip");

        for address  in address.iter(){
            if address.type_ == "InternalIP".to_string(){
                external_ip = address.address.to_string()
            }
        }
        let node_name = node.name();

        KubeNodeStatus{
            node_name,
            capacity_cpu,
            capacity_mem,
            capacity_disk,
            external_ip,
            os_image,
            architecture
        }
    }
}

impl Clone for KubeNodeStatus{
    fn clone(&self) -> KubeNodeStatus{
        KubeNodeStatus{
            node_name: self.node_name.to_owned(),
            capacity_cpu: self.capacity_cpu.to_owned(),
            capacity_mem: self.capacity_mem.to_owned(),
            capacity_disk: self.capacity_disk.to_owned(),
            external_ip: self.external_ip.to_owned(),
            os_image: self.os_image.to_owned(),
            architecture: self.architecture.to_owned(),
        }
    }
}

pub struct KubeDeploymentStatus {
    pub name: String,
    pub replicas: i32,
}

pub struct KubePodStatus {
    pub name: String,
}

pub type MemoryShareStorage = Arc<Vec<Mutex<HashMap<String, Resource>>>>;

pub fn shared_storage()->MemoryShareStorage{
    let mut db = Vec::with_capacity(4);
    for _ in 0..4 {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}
