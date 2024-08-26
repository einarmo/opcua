use std::{
    sync::{Arc, OnceLock},
    time::{Duration, Instant},
};

use opcua_core::sync::Mutex;
use opcua_types::{
    AttributeId, BuildInfo, DataValue, DateTime, ExtensionObject, LocalizedText, MonitoringMode,
    NodeId, ServerState, ServerStatusDataType, VariableId,
};

use crate::{node_manager::SyncSampler, SubscriptionCache};

/// Wrapper for managing the `ServerStatus` variable on the server.
pub struct ServerStatusWrapper {
    status: Arc<Mutex<ServerStatusDataType>>,
    subscriptions: Arc<SubscriptionCache>,
    sampler: SyncSampler,
    shutdown: Arc<OnceLock<ShutdownTarget>>,
}

struct ShutdownTarget {
    reason: LocalizedText,
    deadline: Instant,
    time: DateTime,
}

impl ServerStatusWrapper {
    pub(crate) fn new(build_info: BuildInfo, subscriptions: Arc<SubscriptionCache>) -> Self {
        let sampler = SyncSampler::new();
        sampler.run(Duration::from_secs(1), subscriptions.clone());

        Self {
            status: Arc::new(Mutex::new(ServerStatusDataType {
                start_time: DateTime::null(),
                current_time: DateTime::now(),
                state: opcua_types::ServerState::Shutdown,
                build_info,
                seconds_till_shutdown: 0,
                shutdown_reason: LocalizedText::null(),
            })),
            subscriptions,
            sampler,
            shutdown: Arc::new(OnceLock::new()),
        }
    }

    pub(crate) fn get_managed_id(&self, id: &NodeId) -> Option<VariableId> {
        let Ok(var_id) = id.as_variable_id() else {
            return None;
        };
        if matches!(
            var_id,
            VariableId::Server_ServerStatus
                | VariableId::Server_ServerStatus_CurrentTime
                | VariableId::Server_ServerStatus_SecondsTillShutdown
                | VariableId::Server_ServerStatus_ShutdownReason
        ) {
            Some(var_id)
        } else {
            None
        }
    }

    pub(crate) fn subscribe_to_component(
        &self,
        id: VariableId,
        mode: MonitoringMode,
        handle: crate::MonitoredItemHandle,
        sampling_interval: Duration,
    ) {
        let status = self.status.clone();
        let shutdown = self.shutdown.clone();
        match id {
            VariableId::Server_ServerStatus => self.sampler.add_sampler(
                id.into(),
                AttributeId::Value,
                move || {
                    let mut status = status.lock();
                    status.current_time = DateTime::now();
                    Some(DataValue::new_now(ExtensionObject::from_message(&*status)))
                },
                mode,
                handle,
                sampling_interval,
            ),
            VariableId::Server_ServerStatus_CurrentTime => self.sampler.add_sampler(
                id.into(),
                AttributeId::Value,
                || Some(DataValue::new_now(DateTime::now())),
                mode,
                handle,
                sampling_interval,
            ),
            VariableId::Server_ServerStatus_SecondsTillShutdown => self.sampler.add_sampler(
                id.into(),
                AttributeId::Value,
                move || {
                    if let Some(v) = shutdown.get() {
                        let now = Instant::now();
                        let left = if now < v.deadline {
                            (v.deadline - now).as_secs()
                        } else {
                            0
                        };
                        Some(DataValue::new_now(left as u32))
                    } else {
                        None
                    }
                },
                mode,
                handle,
                sampling_interval,
            ),
            VariableId::Server_ServerStatus_ShutdownReason => self.sampler.add_sampler(
                id.into(),
                AttributeId::Value,
                move || {
                    if let Some(v) = shutdown.get() {
                        Some(DataValue::new_at(v.reason.clone(), v.time))
                    } else {
                        None
                    }
                },
                mode,
                handle,
                sampling_interval,
            ),
            _ => return,
        }
    }

    pub(crate) fn sampler(&self) -> &SyncSampler {
        &self.sampler
    }

    fn notify_status_object_change(&self) {
        self.subscriptions.maybe_notify(
            [(&VariableId::Server_ServerStatus.into(), AttributeId::Value)].into_iter(),
            |_, _, n, _| {
                if n.has_range() {
                    None
                } else {
                    Some(DataValue::new_now(ExtensionObject::from_message(
                        &*self.status.lock(),
                    )))
                }
            },
        )
    }

    pub fn set_state(&self, state: ServerState) {
        self.status.lock().state = state;
        self.subscriptions.notify_data_change(
            [(
                DataValue::new_now(state as i32),
                &VariableId::Server_ServerStatus_State.into(),
                AttributeId::Value,
            )]
            .into_iter(),
        );
        self.notify_status_object_change();
    }

    pub(crate) fn set_start_time(&self, time: DateTime) {
        self.status.lock().start_time = time;
    }

    pub(crate) fn set_server_started(&self) {
        self.set_state(ServerState::Running);
        self.set_start_time(DateTime::now());
    }

    pub(crate) fn schedule_shutdown(&self, reason: LocalizedText, deadline: Instant) {
        let _ = self.shutdown.set(ShutdownTarget {
            time: DateTime::now(),
            reason,
            deadline,
        });
    }

    pub fn build_info(&self) -> BuildInfo {
        self.status.lock().build_info.clone()
    }

    pub fn state(&self) -> ServerState {
        self.status.lock().state.clone()
    }

    pub fn start_time(&self) -> DateTime {
        self.status.lock().start_time.clone()
    }

    pub fn seconds_till_shutdown(&self) -> Option<u32> {
        if let Some(v) = self.shutdown.get() {
            let now = Instant::now();
            let left = if now < v.deadline {
                (v.deadline - now).as_secs()
            } else {
                0
            };
            Some(left as u32)
        } else {
            None
        }
    }

    pub fn shutdown_reason(&self) -> Option<LocalizedText> {
        if let Some(v) = self.shutdown.get() {
            Some(v.reason.clone())
        } else {
            None
        }
    }

    pub fn full_status_obj(&self) -> ExtensionObject {
        ExtensionObject::from_message(&*self.status.lock())
    }
}
