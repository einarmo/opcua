use crate::{
    Array, AttributeId, BinaryEncoder, ByteString, DataValue, DateTime, DiagnosticInfo,
    ExpandedNodeId, ExtensionObject, Guid, LocalizedText, MessageInfo, NodeId, NumericRange,
    QualifiedName, StatusCode, UAString, Variant,
};

/// Trait implemented by any type that can be a field in an event.
pub trait EventField {
    /// Get the variant representation of this field, using the given index range.
    ///
    /// # Arguments
    ///
    ///  * `attribute_id` - the attribute to get. Should be either `NodeId` or `Value`.
    ///  * `index_range` - the range of the value to get.
    ///  * `remaining_path` - the remaining path to the actual value to retrieve.
    fn get_value(
        &self,
        attribute_id: AttributeId,
        index_range: NumericRange,
        remaining_path: &[QualifiedName],
    ) -> Variant;
}

impl<T> EventField for T
where
    T: BinaryEncoder + MessageInfo,
{
    fn get_value(
        &self,
        attribute_id: AttributeId,
        index_range: NumericRange,
        remaining_path: &[QualifiedName],
    ) -> Variant {
        if remaining_path.len() != 0
            || attribute_id != AttributeId::Value
            || index_range != NumericRange::None
        {
            return Variant::Empty;
        }
        ExtensionObject::from_message(self).into()
    }
}

impl<T> EventField for Option<T>
where
    T: EventField,
{
    fn get_value(
        &self,
        attribute_id: AttributeId,
        index_range: NumericRange,
        remaining_path: &[QualifiedName],
    ) -> Variant {
        let Some(val) = self.as_ref() else {
            return Variant::Empty;
        };
        val.get_value(attribute_id, index_range, remaining_path)
    }
}

impl<T> EventField for Vec<T>
where
    T: EventField + Clone,
{
    fn get_value(
        &self,
        attribute_id: AttributeId,
        index_range: NumericRange,
        remaining_path: &[QualifiedName],
    ) -> Variant {
        if remaining_path.len() != 0 {
            return Variant::Empty;
        }

        let values: Vec<_> = match index_range {
            NumericRange::None => self
                .iter()
                .map(|v| v.get_value(attribute_id, NumericRange::None, &[]))
                .collect(),
            NumericRange::Index(i) => {
                return self.get(i as usize).cloned().get_value(
                    attribute_id,
                    NumericRange::None,
                    &[],
                );
            }
            NumericRange::Range(s, e) => {
                if e <= s {
                    return Variant::Empty;
                }
                let Some(r) = self.get((s as usize)..(e as usize)) else {
                    return Variant::Empty;
                };
                r.iter()
                    .map(|v| v.get_value(attribute_id, NumericRange::None, &[]))
                    .collect()
            }
            NumericRange::MultipleRanges(r) => {
                let mut values = Vec::new();
                for range in r {
                    match range {
                        NumericRange::Index(i) => {
                            values.push(self.get(i as usize).cloned().get_value(
                                attribute_id,
                                NumericRange::None,
                                &[],
                            ));
                        }
                        NumericRange::Range(s, e) => {
                            if e <= s {
                                return Variant::Empty;
                            }
                            let Some(r) = self.get((s as usize)..(e as usize)) else {
                                continue;
                            };
                            values.extend(
                                r.iter()
                                    .map(|v| v.get_value(attribute_id, NumericRange::None, &[])),
                            )
                        }
                        _ => return Variant::Empty,
                    }
                }
                values
            }
        };

        if let Some(first) = values.first() {
            let Ok(arr) = Array::new(first.type_id(), values) else {
                return Variant::Empty;
            };
            arr.into()
        } else {
            Variant::Empty
        }
    }
}

macro_rules! basic_field_impl {
    ($ty:ty) => {
        impl EventField for $ty {
            fn get_value(
                &self,
                attribute_id: AttributeId,
                index_range: NumericRange,
                remaining_path: &[QualifiedName],
            ) -> Variant {
                if remaining_path.len() != 0 || attribute_id != AttributeId::Value {
                    return Variant::Empty;
                }
                let val: Variant = self.clone().into();
                val.range_of_owned(index_range).unwrap_or(Variant::Empty)
            }
        }
    };
}

basic_field_impl!(bool);
basic_field_impl!(u8);
basic_field_impl!(i8);
basic_field_impl!(u16);
basic_field_impl!(i16);
basic_field_impl!(i32);
basic_field_impl!(u32);
basic_field_impl!(i64);
basic_field_impl!(u64);
basic_field_impl!(f32);
basic_field_impl!(f64);
basic_field_impl!(UAString);
basic_field_impl!(String);
basic_field_impl!(DateTime);
basic_field_impl!(Guid);
basic_field_impl!(StatusCode);
basic_field_impl!(ByteString);
basic_field_impl!(QualifiedName);
basic_field_impl!(LocalizedText);
basic_field_impl!(NodeId);
basic_field_impl!(ExpandedNodeId);
basic_field_impl!(ExtensionObject);
basic_field_impl!(DataValue);
basic_field_impl!(DiagnosticInfo);
basic_field_impl!(Array);

impl EventField for Variant {
    fn get_value(
        &self,
        attribute_id: AttributeId,
        index_range: NumericRange,
        remaining_path: &[QualifiedName],
    ) -> Variant {
        if remaining_path.len() != 0 || attribute_id != AttributeId::Value {
            return Variant::Empty;
        }
        self.clone()
            .range_of_owned(index_range)
            .unwrap_or(Variant::Empty)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        crypto::random,
        server::{node_manager::NamespaceMap, BaseEventType, Event, EventField},
        types::{
            AttributeId, DecodingOptions, EUInformation, KeyValuePair, LocalizedText, NodeId,
            NumericRange, ObjectTypeId, QualifiedName, StatusCode, UAString, Variant,
        },
    };

    use crate as opcua;

    #[derive(Event)]
    #[opcua(identifier = "s=myevent", namespace = "uri:my:namespace")]
    struct BasicValueEvent {
        base: BaseEventType,
        own_namespace_index: u16,
        // Some primitives
        float: f32,
        double: f64,
        string: String,
        status: StatusCode,
        // Option
        int: Option<i64>,
        int2: Option<u64>,
        // Vec
        vec: Vec<i64>,
        // OptVec
        optvec: Option<Vec<i32>>,
        // Complex type with message info
        kvp: KeyValuePair,
        euinfo: EUInformation,
    }

    fn namespace_map() -> NamespaceMap {
        let mut map = NamespaceMap::new();
        map.add_namespace("uri:my:namespace");
        map
    }

    fn get(id: &NodeId, evt: &dyn Event, field: &str) -> Variant {
        evt.get_field(&id, AttributeId::Value, NumericRange::None, &[field.into()])
    }

    fn get_nested(id: &NodeId, evt: &dyn Event, fields: &[&str]) -> Variant {
        let fields: Vec<QualifiedName> = fields.iter().map(|f| (*f).into()).collect();
        evt.get_field(&id, AttributeId::Value, NumericRange::None, &fields)
    }

    #[test]
    fn test_basic_values() {
        let namespaces = namespace_map();
        let mut evt = BasicValueEvent::new_event_now(
            BasicValueEvent::event_type_id(&namespaces),
            random::byte_string(128),
            "Some message",
            &namespaces,
        );
        evt.float = 1.0;
        evt.double = 2.0;
        evt.string = "foo".to_owned();
        evt.status = StatusCode::BadMaxAgeInvalid;
        evt.kvp = KeyValuePair {
            key: "Key".into(),
            value: 123.into(),
        };
        evt.int = None;
        evt.int2 = Some(5);
        evt.vec = vec![1, 2, 3];
        evt.optvec = Some(vec![3, 2, 1]);
        evt.euinfo = EUInformation {
            namespace_uri: "uri:my:namespace".into(),
            unit_id: 15,
            display_name: "Some unit".into(),
            description: "Some unit desc".into(),
        };
        let id = BasicValueEvent::event_type_id(&namespaces);

        // Get for some other event
        assert_eq!(
            evt.get_field(
                &ObjectTypeId::ProgressEventType.into(),
                AttributeId::Value,
                NumericRange::None,
                &["Message".into()]
            ),
            Variant::Empty
        );
        // Get a field that doesn't exist
        assert_eq!(
            evt.get_field(
                &id,
                AttributeId::Value,
                NumericRange::None,
                &["FooBar".into()]
            ),
            Variant::Empty
        );
        // Get a child of a field without children
        assert_eq!(
            evt.get_field(
                &id,
                AttributeId::Value,
                NumericRange::None,
                &["Float".into(), "Child".into()]
            ),
            Variant::Empty
        );
        // Get a non-value attribute
        assert_eq!(
            evt.get_field(
                &id,
                AttributeId::NodeId,
                NumericRange::None,
                &["Float".into()]
            ),
            Variant::Empty
        );

        // Test equality for each field
        assert_eq!(get(&id, &evt, "Float"), Variant::from(1f32));
        assert_eq!(get(&id, &evt, "Double"), Variant::from(2.0));
        assert_eq!(get(&id, &evt, "String"), Variant::from("foo"));
        assert_eq!(
            get(&id, &evt, "Status"),
            Variant::from(StatusCode::BadMaxAgeInvalid)
        );
        let kvp: KeyValuePair = match get(&id, &evt, "Kvp") {
            Variant::ExtensionObject(o) => o.decode_inner(&DecodingOptions::test()).unwrap(),
            _ => panic!("Wrong variant type"),
        };
        assert_eq!(kvp.key, "Key".into());
        assert_eq!(kvp.value, 123.into());

        assert_eq!(get(&id, &evt, "Int"), Variant::Empty);
        assert_eq!(get(&id, &evt, "Int2"), Variant::from(5u64));
        assert_eq!(get(&id, &evt, "Vec"), Variant::from(vec![1i64, 2i64, 3i64]));
        assert_eq!(
            get(&id, &evt, "Optvec"),
            Variant::from(vec![3i32, 2i32, 1i32])
        );
        let euinfo: EUInformation = match get(&id, &evt, "Euinfo") {
            Variant::ExtensionObject(o) => o.decode_inner(&DecodingOptions::test()).unwrap(),
            _ => panic!("Wrong variant type"),
        };
        assert_eq!(euinfo.namespace_uri.as_ref(), "uri:my:namespace");
        assert_eq!(euinfo.unit_id, 15);
        assert_eq!(euinfo.display_name, "Some unit".into());
        assert_eq!(euinfo.description, "Some unit desc".into());
    }

    #[derive(EventField, Default, Debug)]
    struct ComplexEventField {
        float: f32,
    }

    #[derive(EventField, Default, Debug)]
    struct SubComplexEventField {
        base: ComplexEventField,
        node_id: NodeId,
        #[opcua(rename = "gnirtS")]
        string: UAString,
        #[opcua(ignore)]
        data: i32,
    }

    #[derive(EventField, Default, Debug)]
    struct ComplexVariable {
        node_id: NodeId,
        value: i32,
        id: u32,
    }

    #[derive(Event)]
    #[opcua(identifier = "s=mynestedevent", namespace = "uri:my:namespace")]
    struct NestedEvent {
        base: BasicValueEvent,
        own_namespace_index: u16,
        complex: ComplexEventField,
        sub_complex: SubComplexEventField,
        var: ComplexVariable,
        #[opcua(ignore)]
        ignored: i32,
        #[opcua(rename = "Fancy Name")]
        renamed: String,
    }

    #[test]
    fn test_nested_values() {
        let namespaces = namespace_map();
        let mut evt = NestedEvent::new_event_now(
            NestedEvent::event_type_id(&namespaces),
            random::byte_string(128),
            "Some message",
            &namespaces,
        );
        let id = NestedEvent::event_type_id(&namespaces);
        evt.base.float = 2f32;
        evt.complex.float = 3f32;
        evt.sub_complex.base.float = 4f32;
        evt.sub_complex.string = "foo".into();
        evt.sub_complex.data = 15;
        evt.ignored = 16;
        evt.renamed = "bar".to_owned();
        evt.sub_complex.node_id = NodeId::new(0, 15);
        evt.var.node_id = NodeId::new(0, 16);
        evt.var.value = 20;

        // Get field from middle event type
        assert_eq!(get(&id, &evt, "Float"), Variant::from(2f32));
        // Get from grandparent
        assert_eq!(
            get(&id, &evt, "Message"),
            Variant::from(LocalizedText::from("Some message"))
        );
        // Ignored fields should be skipped
        assert_eq!(get(&id, &evt, "Ignored"), Variant::Empty);
        assert_eq!(
            get_nested(&id, &evt, &["SubComplex", "Data"]),
            Variant::Empty
        );
        // Get renamed
        assert_eq!(get(&id, &evt, "Fancy Name"), Variant::from("bar"));
        assert_eq!(
            get_nested(&id, &evt, &["SubComplex", "gnirtS"]),
            Variant::from("foo")
        );
        // Get complex
        assert_eq!(
            get_nested(&id, &evt, &["Complex", "Float"]),
            Variant::from(3f32)
        );
        assert_eq!(
            get_nested(&id, &evt, &["SubComplex", "Float"]),
            Variant::from(4f32)
        );

        // Get node IDs
        assert_eq!(
            evt.get_field(
                &id,
                AttributeId::NodeId,
                NumericRange::None,
                &["SubComplex".into()]
            ),
            Variant::from(NodeId::new(0, 15))
        );
        assert_eq!(
            evt.get_field(
                &id,
                AttributeId::NodeId,
                NumericRange::None,
                &["Var".into()]
            ),
            Variant::from(NodeId::new(0, 16))
        );
        assert_eq!(
            evt.get_field(&id, AttributeId::Value, NumericRange::None, &["Var".into()]),
            Variant::from(20i32)
        );
    }
}
