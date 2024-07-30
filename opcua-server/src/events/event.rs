use opcua_types::{
    event_field::EventField, AttributeId, ByteString, DateTime, LocalizedText, NodeId,
    NumericRange, ObjectTypeId, QualifiedName, TimeZoneDataType, UAString, Variant,
};

pub trait Event: EventField {
    fn get_field(
        &self,
        type_definition_id: &NodeId,
        attribute_id: AttributeId,
        index_range: NumericRange,
        browse_path: &[QualifiedName],
    ) -> Variant {
        if !self.matches_type_id(type_definition_id) {
            return Variant::Empty;
        }
        self.get_value(attribute_id, index_range, browse_path)
    }

    fn time(&self) -> &DateTime;

    fn matches_type_id(&self, id: &NodeId) -> bool;
}

#[derive(Debug, Default)]
/// This corresponds to BaseEventType definition in OPC UA Part 5
pub struct BaseEventType {
    /// A unique identifier for an event, e.g. a GUID in a byte string
    pub event_id: ByteString,
    /// Event type describes the type of event
    pub event_type: NodeId,
    /// Source node identifies the node that the event originated from or null.
    pub source_node: NodeId,
    /// Source name provides the description of the source of the event,
    /// e.g. the display of the event source
    pub source_name: UAString,
    /// Time provides the time the event occurred. As close
    /// to the event generator as possible.
    pub time: DateTime,
    /// Receive time provides the time the OPC UA server received
    /// the event from the underlying device of another server.
    pub receive_time: DateTime,
    /// Local time (optional) is a structure containing
    /// the offset and daylightsaving flag.
    pub local_time: Option<TimeZoneDataType>,
    /// Message provides a human readable localizable text description
    /// of the event.
    pub message: LocalizedText,
    /// Severity is an indication of the urgency of the event. Values from 1 to 1000, with 1 as the lowest
    /// severity and 1000 being the highest. A value of 1000 would indicate an event of catastrophic nature.
    ///
    /// Guidance:
    ///
    /// * 801-1000 - High
    /// * 601-800 - Medium High
    /// * 401-600 - Medium
    /// * 201-400 - Medium Low
    /// * 1-200 - Low
    pub severity: u16,
    /// Condition Class Id specifies in which domain this Event is used.
    pub condition_class_id: Option<NodeId>,
    /// Condition class name specifies the name of the condition class of this event, if set.
    pub condition_class_name: Option<LocalizedText>,
    /// ConditionSubClassId specifies additional class[es] that apply to the Event.
    /// It is the NodeId of the corresponding subtype of BaseConditionClassType.
    pub condition_sub_class_id: Option<Vec<NodeId>>,
    /// Condition sub class name specifies the names of additional classes that apply to the event.
    pub condition_sub_class_name: Option<Vec<LocalizedText>>,
}

impl EventField for BaseEventType {
    fn get_value(
        &self,
        attribute_id: AttributeId,
        index_range: NumericRange,
        remaining_path: &[QualifiedName],
    ) -> Variant {
        if remaining_path.len() != 1 || attribute_id != AttributeId::Value {
            // Field is not from base event type.
            return Variant::Empty;
        }
        let field = &remaining_path[0];
        if field.namespace_index != 0 {
            return Variant::Empty;
        }

        match field.name.as_ref() {
            "EventId" => self.event_id.get_value(attribute_id, index_range, &[]),
            "EventType" => self.event_type.get_value(attribute_id, index_range, &[]),
            "SourceNode" => self.source_node.get_value(attribute_id, index_range, &[]),
            "SourceName" => self.source_name.get_value(attribute_id, index_range, &[]),
            "Time" => self.time.get_value(attribute_id, index_range, &[]),
            "ReceiveTime" => self.receive_time.get_value(attribute_id, index_range, &[]),
            "LocalTime" => self.local_time.get_value(attribute_id, index_range, &[]),
            "Message" => self.message.get_value(attribute_id, index_range, &[]),
            "Severity" => self.severity.get_value(attribute_id, index_range, &[]),
            "ConditionClassId" => self
                .condition_class_id
                .get_value(attribute_id, index_range, &[]),
            "ConditionClassName" => {
                self.condition_class_name
                    .get_value(attribute_id, index_range, &[])
            }
            "ConditionSubClassId" => {
                self.condition_sub_class_id
                    .get_value(attribute_id, index_range, &[])
            }
            "ConditionSubClassName" => {
                self.condition_sub_class_name
                    .get_value(attribute_id, index_range, &[])
            }
            _ => Variant::Empty,
        }
    }
}

impl Event for BaseEventType {
    fn time(&self) -> &DateTime {
        &self.time
    }

    fn matches_type_id(&self, id: &NodeId) -> bool {
        let own_type_id: NodeId = ObjectTypeId::BaseEventType.into();
        id == &own_type_id
    }
}

impl BaseEventType {
    pub fn new_now(
        type_id: impl Into<NodeId>,
        event_id: ByteString,
        message: impl Into<LocalizedText>,
    ) -> Self {
        let time = DateTime::now();
        Self::new(type_id, event_id, message, time)
    }

    pub fn new(
        type_id: impl Into<NodeId>,
        event_id: ByteString,
        message: impl Into<LocalizedText>,
        time: DateTime,
    ) -> Self {
        Self {
            event_id,
            event_type: type_id.into(),
            message: message.into(),
            time,
            receive_time: time,
            ..Default::default()
        }
    }

    pub fn set_source_node(mut self, source_node: NodeId) -> Self {
        self.source_node = source_node;
        self
    }

    pub fn set_source_name(mut self, source_name: UAString) -> Self {
        self.source_name = source_name;
        self
    }

    pub fn set_receive_time(mut self, receive_time: DateTime) -> Self {
        self.receive_time = receive_time;
        self
    }

    pub fn set_severity(mut self, severity: u16) -> Self {
        self.severity = severity;
        self
    }
}
