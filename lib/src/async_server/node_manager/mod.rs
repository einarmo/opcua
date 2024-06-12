use std::sync::Arc;

use async_trait::async_trait;

use crate::server::prelude::{
    MonitoredItemModifyResult, MonitoringMode, NodeId, ReadAnnotationDataDetails,
    ReadAtTimeDetails, ReadEventDetails, ReadProcessedDetails, ReadRawModifiedDetails, StatusCode,
    TimestampsToReturn,
};

mod attributes;
mod context;
mod history;
pub mod memory;
mod type_tree;
mod view;

use self::view::ExternalReferenceRequest;

use super::{subscriptions::CreateMonitoredItem, MonitoredItemHandle, SubscriptionCache};

pub use {
    attributes::{ReadNode, WriteNode},
    context::RequestContext,
    history::{HistoryNode, HistoryResult, HistoryUpdateDetails, HistoryUpdateNode},
    type_tree::TypeTree,
    view::{BrowseContinuationPoint, BrowseNode, BrowsePathItem, RegisterNodeItem},
};

pub(crate) use context::resolve_external_references;
pub(crate) use history::HistoryReadDetails;
pub(crate) use view::ExternalReferencesContPoint;

/// Trait for a type that implements logic for responding to requests.
/// Implementations of this trait may make external calls for node information,
/// or do other complex tasks.
///
/// Note that each request is passed to every node manager concurrently.
/// It is up to each node manager to avoid responding to requests for nodes
/// managed by a different node manager.
///
/// Requests are spawned on the tokio thread pool. Avoid making blocking calls in
/// methods on this trait. If you need to do blocking work use `tokio::spawn_blocking`,
/// though you should use async IO as much as possible.
///
/// For a simpler interface see InMemoryNodeManager, use this trait directly
/// if you need to control how all node information is stored.
#[allow(unused_variables)]
#[async_trait]
pub trait NodeManager {
    /// Return whether this node manager owns the given node, this is used for
    /// propagating service-level errors.
    ///
    /// If a service returns an error, all nodes it owns will get that error,
    /// even if this is a cross node-manager request like Browse.
    fn owns_node(&self, id: &NodeId) -> bool;

    /// Name of this node manager, for debug purposes.
    fn name(&self) -> &str;

    /// Return whether this node manager owns events on the server.
    /// The first node manager that returns true here will be called when
    /// reading or updating historical server events.
    fn owns_server_events(&self) -> bool {
        false
    }

    /// Perform any necessary loading of nodes, should populate the type tree if
    /// needed.
    async fn init(&self, type_tree: &mut TypeTree, subscriptions: Arc<SubscriptionCache>);

    /// Resolve a list of references given by a different node manager.
    async fn resolve_external_references(
        &self,
        context: &RequestContext,
        items: &mut [&mut ExternalReferenceRequest],
    ) {
    }

    // ATTRIBUTES
    /// Execute the Read service. This should set results on the given nodes_to_read as needed.
    async fn read(
        &self,
        context: &RequestContext,
        max_age: f64,
        timestamps_to_return: TimestampsToReturn,
        nodes_to_read: &mut [&mut ReadNode],
    ) -> Result<(), StatusCode> {
        Err(StatusCode::BadServiceUnsupported)
    }

    /// Perform the history read raw modified service. This should write results
    /// to the `nodes` list of type either `HistoryData` or `HistoryModifiedData`
    async fn history_read_raw_modified(
        &self,
        context: &RequestContext,
        details: &ReadRawModifiedDetails,
        nodes: &mut [&mut HistoryNode],
        timestamps_to_return: TimestampsToReturn,
    ) -> Result<(), StatusCode> {
        Err(StatusCode::BadHistoryOperationUnsupported)
    }

    /// Perform the history read processed service. This should write results
    /// to the `nodes` list of type `HistoryData`.
    async fn history_read_processed(
        &self,
        context: &RequestContext,
        details: &ReadProcessedDetails,
        nodes: &mut [&mut HistoryNode],
        timestamps_to_return: TimestampsToReturn,
    ) -> Result<(), StatusCode> {
        Err(StatusCode::BadHistoryOperationUnsupported)
    }

    /// Perform the history read processed service. This should write results
    /// to the `nodes` list of type `HistoryData`.
    async fn history_read_at_time(
        &self,
        context: &RequestContext,
        details: &ReadAtTimeDetails,
        nodes: &mut [&mut HistoryNode],
        timestamps_to_return: TimestampsToReturn,
    ) -> Result<(), StatusCode> {
        Err(StatusCode::BadHistoryOperationUnsupported)
    }

    /// Perform the history read events service. This should write results
    /// to the `nodes` list of type `HistoryEvent`.
    async fn history_read_events(
        &self,
        context: &RequestContext,
        details: &ReadEventDetails,
        nodes: &mut [&mut HistoryNode],
        timestamps_to_return: TimestampsToReturn,
    ) -> Result<(), StatusCode> {
        Err(StatusCode::BadHistoryOperationUnsupported)
    }

    /// Perform the history read annotations data service. This should write
    /// results to the `nodes` list of type `Annotation`.
    async fn history_read_annotations(
        &self,
        context: &RequestContext,
        details: &ReadAnnotationDataDetails,
        nodes: &mut [&mut HistoryNode],
        timestamps_to_return: TimestampsToReturn,
    ) -> Result<(), StatusCode> {
        Err(StatusCode::BadHistoryOperationUnsupported)
    }

    /// Perform the write service. This should write results
    /// to the `nodes_to_write` list. The default result is `BadNodeIdUnknown`
    async fn write(
        &self,
        context: &RequestContext,
        nodes_to_write: &mut [&mut WriteNode],
    ) -> Result<(), StatusCode> {
        Err(StatusCode::BadServiceUnsupported)
    }

    /// Perform the HistoryUpdate service. This should write result
    /// status codes to the `nodes` list as appropriate.
    async fn history_update(
        &self,
        context: &RequestContext,
        nodes: &mut [&mut HistoryUpdateNode],
    ) -> Result<(), StatusCode> {
        Err(StatusCode::BadHistoryOperationUnsupported)
    }

    // VIEW
    /// Perform the Browse or BrowseNext service.
    async fn browse(
        &self,
        context: &RequestContext,
        nodes_to_browse: &mut [BrowseNode],
    ) -> Result<(), StatusCode> {
        Err(StatusCode::BadServiceUnsupported)
    }

    /// Perform the translate browse paths to node IDs service.
    async fn translate_browse_paths_to_node_ids(
        &self,
        context: &RequestContext,
        nodes: &mut [&mut BrowsePathItem],
    ) -> Result<(), StatusCode> {
        Err(StatusCode::BadServiceUnsupported)
    }

    /// Perform the register nodes service. The default behavior for this service is to
    /// do nothing and pretend the nodes were registered.
    async fn register_nodes(
        &self,
        context: &RequestContext,
        nodes: &mut [&mut RegisterNodeItem],
    ) -> Result<(), StatusCode> {
        // Most servers don't actually do anything with node registration, it is reasonable
        // to just pretend the nodes are registered.
        for node in nodes {
            node.set_registered(true);
        }

        Ok(())
    }

    /// Perform the unregister nodes service. The default behavior for this service is to
    /// do nothing.
    async fn unregister_nodes(
        &self,
        context: &RequestContext,
        _nodes: &[&NodeId],
    ) -> Result<(), StatusCode> {
        // Again, just do nothing
        Ok(())
    }

    /// Prepare for monitored item creation, the node manager must take action to
    /// sample data for each produced monitored item, according to the parameters.
    /// Monitored item parameters have already been revised according to server limits,
    /// but the node manager is allowed to further revise sampling interval.
    ///
    /// The node manager should also read the initial value of each monitored item,
    /// and set the status code if monitored item creation failed.
    ///
    /// The node manager is responsible for tracking the subscription no matter what
    /// the value of monitoring_mode is, but should only sample if monitoring_mode
    /// is not Disabled.
    async fn create_monitored_items(
        &self,
        context: &RequestContext,
        items: &mut [&mut CreateMonitoredItem],
    ) -> Result<(), StatusCode> {
        Err(StatusCode::BadServiceUnsupported)
    }

    /// Modify monitored items. This method is purely informative for the node manager,
    /// to let it modify sampling intervals, apply a new filter, or similar.
    ///
    /// Node managers are not required to take any action here, and this method is not
    /// allowed to fail.
    async fn modify_monitored_items(
        &self,
        context: &RequestContext,
        items: &[(&MonitoredItemModifyResult, &NodeId, u32)],
    ) {
    }

    /// Modify monitored items. This method is purely informative for the node manager,
    /// to let it pause or resume sampling. Note that this should _not_ delete context
    /// stored from `create_monitored_items`, since it may be called again to resume sampling.
    ///
    /// The node manager should sample so long as monitoring mode is not `Disabled`, the difference
    /// between `Reporting` and `Sampling` is handled by the server.
    ///
    /// Node managers are not required to take any action here, and this method is not
    /// allowed to fail.
    async fn set_monitoring_mode(
        &self,
        context: &RequestContext,
        mode: MonitoringMode,
        items: &[(MonitoredItemHandle, &NodeId, u32)],
    ) {
    }

    /// Delete monitored items. This method is purely informative for the node manager,
    /// to let it stop sampling, or similar.
    ///
    /// Node managers are not required to take any action here, and this method is not
    /// allowed to fail. Most node managers that implement subscriptions will want to do
    /// something with this.
    ///
    /// This method may be given monitored items that were never created, or were
    /// created for a different node manager. Attempting to delete a monitored item
    /// that does not exist is handled elsewhere and should be a no-op here.
    async fn delete_monitored_items(
        &self,
        context: &RequestContext,
        items: &[(MonitoredItemHandle, &NodeId, u32)],
    ) {
    }
}
