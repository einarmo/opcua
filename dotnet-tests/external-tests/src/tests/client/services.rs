use std::{collections::HashMap, sync::Arc};

use opcua::{
    client::Session,
    types::{
        BrowseDescription, BrowseResultMask, CallMethodRequest, EUInformation, NodeClass, NodeId,
        QualifiedName, ReadValueId, ReferenceTypeId, StatusCode, VariableTypeId,
    },
};

pub async fn test_read(session: Arc<Session>) {
    let r = session
        .read(
            &[
                ReadValueId::new_value(NodeId::new(2, "VarDouble")),
                ReadValueId::new_value(NodeId::new(2, "VarString")),
                ReadValueId::new_value(NodeId::new(2, "VarEuInfo")),
            ],
            opcua::types::TimestampsToReturn::Both,
            0.0,
        )
        .await
        .unwrap();
    assert_eq!(3, r.len());
    assert_eq!(
        r[0].value.clone().unwrap().try_cast_to::<f64>().unwrap(),
        0.0f64
    );
    assert_eq!(
        r[1].value.clone().unwrap().try_cast_to::<String>().unwrap(),
        "test 0"
    );
    assert_eq!(
        r[2].value
            .clone()
            .unwrap()
            .try_cast_to::<EUInformation>()
            .unwrap(),
        EUInformation {
            namespace_uri: "opc.tcp://test.localhost".into(),
            unit_id: 0,
            display_name: "Degrees C".into(),
            description: "Temperature degrees Celsius".into()
        }
    );
}

pub async fn test_browse(session: Arc<Session>) {
    let r = session
        .browse(
            &[BrowseDescription {
                node_id: NodeId::new(2, "CoreBase"),
                browse_direction: opcua::types::BrowseDirection::Forward,
                reference_type_id: ReferenceTypeId::HierarchicalReferences.into(),
                include_subtypes: true,
                node_class_mask: 0,
                result_mask: BrowseResultMask::All as u32,
            }],
            100,
            None,
        )
        .await
        .unwrap();

    assert_eq!(1, r.len());
    let refs = r.into_iter().next().unwrap().references.unwrap();
    assert_eq!(4, refs.len());
    let mut by_id: HashMap<_, _> = refs
        .into_iter()
        .map(|r| (r.node_id.node_id.clone(), r))
        .collect();

    let n = by_id.remove(&NodeId::new(2, "VarDouble")).unwrap();
    assert_eq!(n.browse_name, QualifiedName::new(2, "VarDouble"));
    assert_eq!(n.display_name, "VarDouble".into());
    assert_eq!(n.reference_type_id, ReferenceTypeId::HasComponent);
    assert!(n.is_forward);
    assert_eq!(
        n.type_definition.node_id,
        VariableTypeId::BaseDataVariableType
    );
    assert_eq!(n.node_class, NodeClass::Variable);

    let n = by_id.remove(&NodeId::new(2, "VarString")).unwrap();
    assert_eq!(n.browse_name, QualifiedName::new(2, "VarString"));
    assert_eq!(n.display_name, "VarString".into());
    assert_eq!(n.reference_type_id, ReferenceTypeId::HasComponent);
    assert!(n.is_forward);
    assert_eq!(
        n.type_definition.node_id,
        VariableTypeId::BaseDataVariableType
    );
    assert_eq!(n.node_class, NodeClass::Variable);

    let n = by_id.remove(&NodeId::new(2, "VarEuInfo")).unwrap();
    assert_eq!(n.browse_name, QualifiedName::new(2, "VarEuInfo"));
    assert_eq!(n.display_name, "VarEuInfo".into());
    assert_eq!(n.reference_type_id, ReferenceTypeId::HasComponent);
    assert!(n.is_forward);
    assert_eq!(n.type_definition.node_id, VariableTypeId::PropertyType);
    assert_eq!(n.node_class, NodeClass::Variable);

    let n = by_id.remove(&NodeId::new(2, "EchoMethod")).unwrap();
    assert_eq!(n.browse_name, QualifiedName::new(2, "EchoMethod"));
    assert_eq!(n.display_name, "EchoMethod".into());
    assert_eq!(n.reference_type_id, ReferenceTypeId::HasComponent);
    assert!(n.is_forward);
    assert_eq!(n.node_class, NodeClass::Method);
}

pub async fn test_call(session: Arc<Session>) {
    let r = session
        .call_one(CallMethodRequest {
            object_id: NodeId::new(2, "CoreBase"),
            method_id: NodeId::new(2, "EchoMethod"),
            input_arguments: Some(vec!["Hello there".into()]),
        })
        .await
        .unwrap();

    assert!(r.status_code.is_good());
    let out = r.output_arguments.unwrap();
    assert_eq!(1, out.len());
    assert_eq!(
        out[0].clone().try_cast_to::<String>().unwrap(),
        "Echo: Hello there"
    );
}

pub async fn test_big_request(session: Arc<Session>) {
    let items: Vec<_> = (0..1000)
        .map(|n| ReadValueId::new_value(NodeId::new(2, format!("{n}{}", "c".repeat(100)))))
        .collect();

    let r = session
        .read(&items, opcua::types::TimestampsToReturn::Both, 0.0)
        .await
        .unwrap();

    for n in r {
        assert_eq!(n.status, Some(StatusCode::BadNodeIdUnknown));
    }
}
