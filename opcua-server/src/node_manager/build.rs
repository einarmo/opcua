use std::sync::Arc;

use super::{DynNodeManager, NodeManager, ServerContext};

pub trait NodeManagerBuilder {
    fn build(self: Box<Self>, context: ServerContext) -> Arc<DynNodeManager>;
}

impl<T, R: NodeManager + Send + Sync + 'static> NodeManagerBuilder for T
where
    T: FnOnce(ServerContext) -> R,
{
    fn build(self: Box<Self>, context: ServerContext) -> Arc<DynNodeManager> {
        Arc::new(self(context))
    }
}
