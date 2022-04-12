

pub enum StateMachine {
    Empty,
    HomeBlock,       // 首页状态机
    WorkloadBlock,   //  工作负载页面状态机
    NetworkBlock, // 网络管理界面的状态机
}

// 首页状态机器
pub enum   HomeBlock{
    Empty,
    NodeList,      //  节点列表查看
    NodeOps,       //  节点操作Block
    NodeStatus,    //  节点状态窗口状态机
}
// 工作负载的状态机
pub enum WorkloadBlock{

}
//  网络管理的状态机器
pub enum NetworkBlock {

}

