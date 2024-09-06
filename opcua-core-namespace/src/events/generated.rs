// This file was autogenerated from tools/schema/schemas/1.05/Opc.Ua.NodeSet2.xml by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua {
    pub use opcua_nodes as nodes;
    pub use opcua_nodes::{Event, EventField};
    pub use opcua_types as types;
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2881")]
pub struct AcknowledgeableConditionType {
    pub base: ConditionType,
    pub confirm: opcua::nodes::MethodEventField,
    pub confirmed_state: TwoStateVariableType,
    pub acked_state: TwoStateVariableType,
    pub enabled_state: TwoStateVariableType,
    pub acknowledge: opcua::nodes::MethodEventField,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2915")]
pub struct AlarmConditionType {
    pub base: AcknowledgeableConditionType,
    pub suppress: opcua::nodes::MethodEventField,
    pub re_alarm_repeat_count: i16,
    pub silence_state: TwoStateVariableType,
    pub input_node: opcua::types::NodeId,
    pub suppressed_state: TwoStateVariableType,
    pub unsuppress: opcua::nodes::MethodEventField,
    pub out_of_service_state: TwoStateVariableType,
    pub remove_from_service: opcua::nodes::MethodEventField,
    pub shelving_state: ShelvedStateMachineType,
    pub first_in_group: AlarmGroupType,
    pub audible_enabled: bool,
    pub place_in_service: opcua::nodes::MethodEventField,
    pub suppressed_or_shelved: bool,
    pub off_delay: opcua::types::Duration,
    pub place_in_service_2: opcua::nodes::MethodEventField,
    pub reset_2: opcua::nodes::MethodEventField,
    pub first_in_group_flag: bool,
    pub audible_sound: AudioVariableType,
    pub remove_from_service_2: opcua::nodes::MethodEventField,
    pub get_group_memberships: opcua::nodes::MethodEventField,
    pub enabled_state: TwoStateVariableType,
    pub on_delay: opcua::types::Duration,
    pub re_alarm_time: opcua::types::Duration,
    pub unsuppress_2: opcua::nodes::MethodEventField,
    pub active_state: TwoStateVariableType,
    pub max_time_shelved: opcua::types::Duration,
    pub reset: opcua::nodes::MethodEventField,
    pub silence: opcua::nodes::MethodEventField,
    pub suppress_2: opcua::nodes::MethodEventField,
    pub latched_state: TwoStateVariableType,
}
#[derive(Debug, opcua::EventField, Default)]
pub struct AlarmGroupType {
    pub node_id: opcua::types::NodeId,
}
#[derive(Debug, opcua::EventField, Default)]
pub struct AudioVariableType {
    pub node_id: opcua::types::NodeId,
    pub value: opcua::types::AudioDataType,
    pub agency_id: opcua::types::UAString,
    pub list_id: opcua::types::UAString,
    pub version_id: opcua::types::UAString,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2075")]
pub struct AuditActivateSessionEventType {
    pub base: AuditSessionEventType,
    pub secure_channel_id: opcua::types::UAString,
    pub user_identity_token: opcua::types::UserIdentityToken,
    pub client_software_certificates: opcua::types::SignedSoftwareCertificate,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2091")]
pub struct AuditAddNodesEventType {
    pub base: AuditNodeManagementEventType,
    pub nodes_to_add: opcua::types::AddNodesItem,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2095")]
pub struct AuditAddReferencesEventType {
    pub base: AuditNodeManagementEventType,
    pub references_to_add: opcua::types::AddReferencesItem,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2078")]
pub struct AuditCancelEventType {
    pub base: AuditSessionEventType,
    pub request_handle: u32,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2082")]
pub struct AuditCertificateDataMismatchEventType {
    pub base: AuditCertificateEventType,
    pub invalid_uri: opcua::types::UAString,
    pub invalid_hostname: opcua::types::UAString,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2080")]
pub struct AuditCertificateEventType {
    pub base: AuditSecurityEventType,
    pub certificate: opcua::types::ByteString,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2085")]
pub struct AuditCertificateExpiredEventType {
    pub base: AuditCertificateEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2086")]
pub struct AuditCertificateInvalidEventType {
    pub base: AuditCertificateEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2089")]
pub struct AuditCertificateMismatchEventType {
    pub base: AuditCertificateEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2088")]
pub struct AuditCertificateRevokedEventType {
    pub base: AuditCertificateEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2087")]
pub struct AuditCertificateUntrustedEventType {
    pub base: AuditCertificateEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2059")]
pub struct AuditChannelEventType {
    pub base: AuditSecurityEventType,
    pub secure_channel_id: opcua::types::UAString,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=23606")]
pub struct AuditClientEventType {
    pub base: AuditEventType,
    pub server_uri: opcua::types::UriString,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=23926")]
pub struct AuditClientUpdateMethodResultEventType {
    pub base: AuditClientEventType,
    pub status_code_id: opcua::types::StatusCode,
    pub input_arguments: opcua::types::Variant,
    pub output_arguments: opcua::types::Variant,
    pub object_id: opcua::types::NodeId,
    pub method_id: opcua::types::NodeId,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=8944")]
pub struct AuditConditionAcknowledgeEventType {
    pub base: AuditConditionEventType,
    pub condition_event_id: opcua::types::ByteString,
    pub comment: opcua::types::LocalizedText,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2829")]
pub struct AuditConditionCommentEventType {
    pub base: AuditConditionEventType,
    pub comment: opcua::types::LocalizedText,
    pub condition_event_id: opcua::types::ByteString,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=8961")]
pub struct AuditConditionConfirmEventType {
    pub base: AuditConditionEventType,
    pub comment: opcua::types::LocalizedText,
    pub condition_event_id: opcua::types::ByteString,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2803")]
pub struct AuditConditionEnableEventType {
    pub base: AuditConditionEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2790")]
pub struct AuditConditionEventType {
    pub base: AuditUpdateMethodEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=17259")]
pub struct AuditConditionOutOfServiceEventType {
    pub base: AuditConditionEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=15013")]
pub struct AuditConditionResetEventType {
    pub base: AuditConditionEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=8927")]
pub struct AuditConditionRespondEventType {
    pub base: AuditConditionEventType,
    pub selected_response: u32,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=11093")]
pub struct AuditConditionShelvingEventType {
    pub base: AuditConditionEventType,
    pub shelving_time: opcua::types::Duration,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=17242")]
pub struct AuditConditionSilenceEventType {
    pub base: AuditConditionEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=17225")]
pub struct AuditConditionSuppressionEventType {
    pub base: AuditConditionEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2071")]
pub struct AuditCreateSessionEventType {
    pub base: AuditSessionEventType,
    pub revised_session_timeout: opcua::types::Duration,
    pub client_certificate: opcua::types::ByteString,
    pub client_certificate_thumbprint: opcua::types::UAString,
    pub secure_channel_id: opcua::types::UAString,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2093")]
pub struct AuditDeleteNodesEventType {
    pub base: AuditNodeManagementEventType,
    pub nodes_to_delete: opcua::types::DeleteNodesItem,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2097")]
pub struct AuditDeleteReferencesEventType {
    pub base: AuditNodeManagementEventType,
    pub references_to_delete: opcua::types::DeleteReferencesItem,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2052")]
pub struct AuditEventType {
    pub base: opcua::nodes::BaseEventType,
    pub client_user_id: opcua::types::UAString,
    pub status: bool,
    pub client_audit_entry_id: opcua::types::UAString,
    pub action_time_stamp: opcua::types::UtcTime,
    pub server_id: opcua::types::UAString,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=19095")]
pub struct AuditHistoryAnnotationUpdateEventType {
    pub base: AuditHistoryUpdateEventType,
    pub old_values: opcua::types::Annotation,
    pub new_values: opcua::types::Annotation,
    pub perform_insert_replace: opcua::types::PerformUpdateType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=3019")]
pub struct AuditHistoryAtTimeDeleteEventType {
    pub base: AuditHistoryDeleteEventType,
    pub old_values: opcua::types::DataValue,
    pub req_times: opcua::types::UtcTime,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=32803")]
pub struct AuditHistoryBulkInsertEventType {
    pub base: AuditEventType,
    pub start_time: opcua::types::UtcTime,
    pub updated_node: opcua::types::NodeId,
    pub end_time: opcua::types::UtcTime,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=32758")]
pub struct AuditHistoryConfigurationChangeEventType {
    pub base: AuditEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=3012")]
pub struct AuditHistoryDeleteEventType {
    pub base: AuditHistoryUpdateEventType,
    pub updated_node: opcua::types::NodeId,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=3022")]
pub struct AuditHistoryEventDeleteEventType {
    pub base: AuditHistoryDeleteEventType,
    pub event_ids: opcua::types::ByteString,
    pub old_values: opcua::types::HistoryEventFieldList,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2999")]
pub struct AuditHistoryEventUpdateEventType {
    pub base: AuditHistoryUpdateEventType,
    pub filter: opcua::types::EventFilter,
    pub old_values: opcua::types::HistoryEventFieldList,
    pub updated_node: opcua::types::NodeId,
    pub new_values: opcua::types::HistoryEventFieldList,
    pub perform_insert_replace: opcua::types::PerformUpdateType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=3014")]
pub struct AuditHistoryRawModifyDeleteEventType {
    pub base: AuditHistoryDeleteEventType,
    pub is_delete_modified: bool,
    pub old_values: opcua::types::DataValue,
    pub start_time: opcua::types::UtcTime,
    pub end_time: opcua::types::UtcTime,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2104")]
pub struct AuditHistoryUpdateEventType {
    pub base: AuditUpdateEventType,
    pub parameter_data_type_id: opcua::types::NodeId,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=3006")]
pub struct AuditHistoryValueUpdateEventType {
    pub base: AuditHistoryUpdateEventType,
    pub perform_insert_replace: opcua::types::PerformUpdateType,
    pub updated_node: opcua::types::NodeId,
    pub old_values: opcua::types::DataValue,
    pub new_values: opcua::types::DataValue,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2090")]
pub struct AuditNodeManagementEventType {
    pub base: AuditEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2060")]
pub struct AuditOpenSecureChannelEventType {
    pub base: AuditChannelEventType,
    pub client_certificate_thumbprint: opcua::types::UAString,
    pub requested_lifetime: opcua::types::Duration,
    pub security_policy_uri: opcua::types::UAString,
    pub certificate_error_event_id: opcua::types::ByteString,
    pub security_mode: opcua::types::MessageSecurityMode,
    pub request_type: opcua::types::SecurityTokenRequestType,
    pub client_certificate: opcua::types::ByteString,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=11856")]
pub struct AuditProgramTransitionEventType {
    pub base: AuditUpdateStateEventType,
    pub transition_number: u32,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2058")]
pub struct AuditSecurityEventType {
    pub base: AuditEventType,
    pub status_code_id: opcua::types::StatusCode,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2069")]
pub struct AuditSessionEventType {
    pub base: AuditSecurityEventType,
    pub session_id: opcua::types::NodeId,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2099")]
pub struct AuditUpdateEventType {
    pub base: AuditEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2127")]
pub struct AuditUpdateMethodEventType {
    pub base: AuditEventType,
    pub input_arguments: opcua::types::Variant,
    pub method_id: opcua::types::NodeId,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2315")]
pub struct AuditUpdateStateEventType {
    pub base: AuditUpdateMethodEventType,
    pub new_state_id: opcua::types::Variant,
    pub old_state_id: opcua::types::Variant,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2748")]
pub struct AuditUrlMismatchEventType {
    pub base: AuditCreateSessionEventType,
    pub endpoint_url: opcua::types::UAString,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2100")]
pub struct AuditWriteUpdateEventType {
    pub base: AuditUpdateEventType,
    pub attribute_id: u32,
    pub old_value: opcua::types::Variant,
    pub index_range: opcua::types::NumericRange,
    pub new_value: opcua::types::Variant,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2132")]
pub struct BaseModelChangeEventType {
    pub base: opcua::nodes::BaseEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=13225")]
pub struct CertificateExpirationAlarmType {
    pub base: SystemOffNormalAlarmType,
    pub certificate_type: opcua::types::NodeId,
    pub expiration_limit: opcua::types::Duration,
    pub certificate: opcua::types::ByteString,
    pub expiration_date: opcua::types::DateTime,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=12620")]
pub struct CertificateUpdatedAuditEventType {
    pub base: AuditEventType,
    pub certificate_type: opcua::types::NodeId,
    pub certificate_group: opcua::types::NodeId,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=32306")]
pub struct CertificateUpdateRequestedAuditEventType {
    pub base: AuditUpdateMethodEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2782")]
pub struct ConditionType {
    pub base: opcua::nodes::BaseEventType,
    pub enabled_state: TwoStateVariableType,
    pub supports_filtered_retain: bool,
    pub quality: ConditionVariableType,
    pub last_severity: ConditionVariableType,
    pub branch_id: opcua::types::NodeId,
    pub condition_sub_class_name: opcua::types::LocalizedText,
    pub disable: opcua::nodes::MethodEventField,
    pub condition_refresh: opcua::nodes::MethodEventField,
    pub condition_sub_class_id: opcua::types::NodeId,
    pub condition_class_name: opcua::types::LocalizedText,
    pub condition_refresh_2: opcua::nodes::MethodEventField,
    pub retain: bool,
    pub enable: opcua::nodes::MethodEventField,
    pub add_comment: opcua::nodes::MethodEventField,
    pub condition_name: opcua::types::UAString,
    pub comment: ConditionVariableType,
    pub condition_class_id: opcua::types::NodeId,
    pub client_user_id: opcua::types::UAString,
}
#[derive(Debug, opcua::EventField, Default)]
pub struct ConditionVariableType {
    pub node_id: opcua::types::NodeId,
    pub value: opcua::types::Variant,
    pub source_timestamp: opcua::types::UtcTime,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2131")]
pub struct DeviceFailureEventType {
    pub base: SystemEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2830")]
pub struct DialogConditionType {
    pub base: ConditionType,
    pub cancel_response: i32,
    pub ok_response: i32,
    pub dialog_state: TwoStateVariableType,
    pub respond: opcua::nodes::MethodEventField,
    pub default_response: i32,
    pub response_option_set: opcua::types::LocalizedText,
    pub respond_2: opcua::nodes::MethodEventField,
    pub prompt: opcua::types::LocalizedText,
    pub last_response: i32,
    pub enabled_state: TwoStateVariableType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=17080")]
pub struct DiscrepancyAlarmType {
    pub base: AlarmConditionType,
    pub target_value_node: opcua::types::NodeId,
    pub expected_time: opcua::types::Duration,
    pub tolerance: f64,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=10523")]
pub struct DiscreteAlarmType {
    pub base: AlarmConditionType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=3035")]
pub struct EventQueueOverflowEventType {
    pub base: opcua::nodes::BaseEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=9764")]
pub struct ExclusiveDeviationAlarmType {
    pub base: ExclusiveLimitAlarmType,
    pub base_setpoint_node: opcua::types::NodeId,
    pub setpoint_node: opcua::types::NodeId,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=9482")]
pub struct ExclusiveLevelAlarmType {
    pub base: ExclusiveLimitAlarmType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=9341")]
pub struct ExclusiveLimitAlarmType {
    pub base: LimitAlarmType,
    pub active_state: TwoStateVariableType,
    pub limit_state: ExclusiveLimitStateMachineType,
}
#[derive(Debug, opcua::EventField, Default)]
pub struct ExclusiveLimitStateMachineType {
    pub base: FiniteStateMachineType,
    pub node_id: opcua::types::NodeId,
    pub high: StateType,
    pub low_low_to_low: TransitionType,
    pub low_to_low_low: TransitionType,
    pub low: StateType,
    pub low_low: StateType,
    pub high_high_to_high: TransitionType,
    pub high_to_high_high: TransitionType,
    pub high_high: StateType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=9623")]
pub struct ExclusiveRateOfChangeAlarmType {
    pub base: ExclusiveLimitAlarmType,
    pub engineering_units: opcua::types::EUInformation,
}
#[derive(Debug, opcua::EventField, Default)]
pub struct FiniteStateMachineType {
    pub base: StateMachineType,
    pub node_id: opcua::types::NodeId,
    pub last_transition: FiniteTransitionVariableType,
    pub current_state: FiniteStateVariableType,
    pub available_transitions: opcua::types::NodeId,
    pub available_states: opcua::types::NodeId,
}
#[derive(Debug, opcua::EventField, Default)]
pub struct FiniteStateVariableType {
    pub base: StateVariableType,
    pub node_id: opcua::types::NodeId,
    pub id: opcua::types::NodeId,
}
#[derive(Debug, opcua::EventField, Default)]
pub struct FiniteTransitionVariableType {
    pub base: TransitionVariableType,
    pub node_id: opcua::types::NodeId,
    pub id: opcua::types::NodeId,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2133")]
pub struct GeneralModelChangeEventType {
    pub base: BaseModelChangeEventType,
    pub changes: opcua::types::ModelChangeStructureDataType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=18347")]
pub struct InstrumentDiagnosticAlarmType {
    pub base: OffNormalAlarmType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=18011")]
pub struct KeyCredentialAuditEventType {
    pub base: AuditUpdateMethodEventType,
    pub resource_uri: opcua::types::UAString,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=18047")]
pub struct KeyCredentialDeletedAuditEventType {
    pub base: KeyCredentialAuditEventType,
    pub resource_uri: opcua::types::UAString,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=18029")]
pub struct KeyCredentialUpdatedAuditEventType {
    pub base: KeyCredentialAuditEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2955")]
pub struct LimitAlarmType {
    pub base: AlarmConditionType,
    pub severity_high_high: u16,
    pub high_high_limit: f64,
    pub severity_low: u16,
    pub low_limit: f64,
    pub high_limit: f64,
    pub base_low_low_limit: f64,
    pub base_high_limit: f64,
    pub low_low_deadband: f64,
    pub high_high_deadband: f64,
    pub low_low_limit: f64,
    pub low_deadband: f64,
    pub severity_high: u16,
    pub high_deadband: f64,
    pub base_low_limit: f64,
    pub base_high_high_limit: f64,
    pub severity_low_low: u16,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=10368")]
pub struct NonExclusiveDeviationAlarmType {
    pub base: NonExclusiveLimitAlarmType,
    pub base_setpoint_node: opcua::types::NodeId,
    pub setpoint_node: opcua::types::NodeId,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=10060")]
pub struct NonExclusiveLevelAlarmType {
    pub base: NonExclusiveLimitAlarmType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=9906")]
pub struct NonExclusiveLimitAlarmType {
    pub base: LimitAlarmType,
    pub active_state: TwoStateVariableType,
    pub high_state: TwoStateVariableType,
    pub high_high_state: TwoStateVariableType,
    pub low_low_state: TwoStateVariableType,
    pub low_state: TwoStateVariableType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=10214")]
pub struct NonExclusiveRateOfChangeAlarmType {
    pub base: NonExclusiveLimitAlarmType,
    pub engineering_units: opcua::types::EUInformation,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=10637")]
pub struct OffNormalAlarmType {
    pub base: DiscreteAlarmType,
    pub normal_state: opcua::types::NodeId,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=3806")]
pub struct ProgramTransitionAuditEventType {
    pub base: AuditUpdateStateEventType,
    pub transition: FiniteTransitionVariableType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2378")]
pub struct ProgramTransitionEventType {
    pub base: TransitionEventType,
    pub intermediate_result: opcua::types::Variant,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=11436")]
pub struct ProgressEventType {
    pub base: opcua::nodes::BaseEventType,
    pub context: opcua::types::Variant,
    pub progress: u16,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=15563")]
pub struct PubSubCommunicationFailureEventType {
    pub base: PubSubStatusEventType,
    pub error: opcua::types::StatusCode,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=15535")]
pub struct PubSubStatusEventType {
    pub base: SystemEventType,
    pub state: opcua::types::PubSubState,
    pub group_id: opcua::types::NodeId,
    pub connection_id: opcua::types::NodeId,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=15548")]
pub struct PubSubTransportLimitsExceedEventType {
    pub base: PubSubStatusEventType,
    pub maximum: u32,
    pub actual: u32,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2788")]
pub struct RefreshEndEventType {
    pub base: SystemEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2789")]
pub struct RefreshRequiredEventType {
    pub base: SystemEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2787")]
pub struct RefreshStartEventType {
    pub base: SystemEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=17641")]
pub struct RoleMappingRuleChangedAuditEventType {
    pub base: AuditUpdateMethodEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2738")]
pub struct SemanticChangeEventType {
    pub base: opcua::nodes::BaseEventType,
    pub changes: opcua::types::SemanticChangeStructureDataType,
}
#[derive(Debug, opcua::EventField, Default)]
pub struct ShelvedStateMachineType {
    pub base: FiniteStateMachineType,
    pub node_id: opcua::types::NodeId,
    pub one_shot_shelved: StateType,
    pub one_shot_shelve_2: opcua::nodes::MethodEventField,
    pub timed_shelve_2: opcua::nodes::MethodEventField,
    pub unshelved: StateType,
    pub unshelve_time: opcua::types::Duration,
    pub timed_shelved: StateType,
    pub unshelved_to_one_shot_shelved: TransitionType,
    pub one_shot_shelved_to_timed_shelved: TransitionType,
    pub timed_shelve: opcua::nodes::MethodEventField,
    pub timed_shelved_to_one_shot_shelved: TransitionType,
    pub unshelve: opcua::nodes::MethodEventField,
    pub unshelve_2: opcua::nodes::MethodEventField,
    pub one_shot_shelve: opcua::nodes::MethodEventField,
    pub timed_shelved_to_unshelved: TransitionType,
    pub one_shot_shelved_to_unshelved: TransitionType,
    pub unshelved_to_timed_shelved: TransitionType,
}
#[derive(Debug, opcua::EventField, Default)]
pub struct StateMachineType {
    pub node_id: opcua::types::NodeId,
    pub current_state: StateVariableType,
    pub last_transition: TransitionVariableType,
}
#[derive(Debug, opcua::EventField, Default)]
pub struct StateType {
    pub node_id: opcua::types::NodeId,
    pub state_number: u32,
}
#[derive(Debug, opcua::EventField, Default)]
pub struct StateVariableType {
    pub node_id: opcua::types::NodeId,
    pub value: opcua::types::LocalizedText,
    pub effective_display_name: opcua::types::LocalizedText,
    pub number: u32,
    pub id: opcua::types::Variant,
    pub name: opcua::types::QualifiedName,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=18496")]
pub struct SystemDiagnosticAlarmType {
    pub base: OffNormalAlarmType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2130")]
pub struct SystemEventType {
    pub base: opcua::nodes::BaseEventType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=11753")]
pub struct SystemOffNormalAlarmType {
    pub base: OffNormalAlarmType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=11446")]
pub struct SystemStatusChangeEventType {
    pub base: SystemEventType,
    pub system_state: opcua::types::ServerState,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=2311")]
pub struct TransitionEventType {
    pub base: opcua::nodes::BaseEventType,
    pub to_state: StateVariableType,
    pub transition: TransitionVariableType,
    pub from_state: StateVariableType,
}
#[derive(Debug, opcua::EventField, Default)]
pub struct TransitionType {
    pub node_id: opcua::types::NodeId,
    pub transition_number: u32,
}
#[derive(Debug, opcua::EventField, Default)]
pub struct TransitionVariableType {
    pub node_id: opcua::types::NodeId,
    pub value: opcua::types::LocalizedText,
    pub transition_time: opcua::types::UtcTime,
    pub id: opcua::types::Variant,
    pub number: u32,
    pub name: opcua::types::QualifiedName,
    pub effective_transition_time: opcua::types::UtcTime,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=10751")]
pub struct TripAlarmType {
    pub base: OffNormalAlarmType,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=19297")]
pub struct TrustListOutOfDateAlarmType {
    pub base: SystemOffNormalAlarmType,
    pub update_frequency: opcua::types::Duration,
    pub trust_list_id: opcua::types::NodeId,
    pub last_update_time: opcua::types::UtcTime,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=12561")]
pub struct TrustListUpdatedAuditEventType {
    pub base: AuditEventType,
    pub trust_list_id: opcua::types::NodeId,
}
#[derive(Debug, opcua::Event)]
#[opcua(identifier = "i=32260")]
pub struct TrustListUpdateRequestedAuditEventType {
    pub base: AuditUpdateMethodEventType,
}
#[derive(Debug, opcua::EventField, Default)]
pub struct TwoStateVariableType {
    pub base: StateVariableType,
    pub node_id: opcua::types::NodeId,
    pub id: bool,
    pub false_state: opcua::types::LocalizedText,
    pub true_state: opcua::types::LocalizedText,
    pub effective_transition_time: opcua::types::UtcTime,
    pub transition_time: opcua::types::UtcTime,
}
