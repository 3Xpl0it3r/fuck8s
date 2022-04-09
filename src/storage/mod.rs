use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;


pub enum Bucket {
    Pod,    // 0
    Deployment, // 1
    Node,       // 2
    Service     // 3
}

pub type MemoryShareStorage = Arc<Vec<Mutex<HashMap<String, String>>>>;

pub fn shared_storage()->MemoryShareStorage{
    let mut db = Vec::with_capacity(4);
    for _ in 0..4 {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}
