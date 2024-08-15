// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DataChangeNotification {
    pub monitored_items: Option<Vec<super::monitored_item_notification::MonitoredItemNotification>>,
    pub diagnostic_infos: Option<Vec<crate::types::diagnostic_info::DiagnosticInfo>>,
}
impl crate::types::BinaryEncoder<DataChangeNotification> for DataChangeNotification {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.monitored_items.byte_len();
        size += self.diagnostic_infos.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(&self, stream: &mut S) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.monitored_items.encode(stream)?;
        size += self.diagnostic_infos.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let monitored_items = <Option<
            Vec<super::monitored_item_notification::MonitoredItemNotification>,
        > as crate::types::BinaryEncoder<
            Option<Vec<super::monitored_item_notification::MonitoredItemNotification>>,
        >>::decode(stream, decoding_options)?;
        let diagnostic_infos = <Option<
            Vec<crate::types::diagnostic_info::DiagnosticInfo>,
        > as crate::types::BinaryEncoder<
            Option<Vec<crate::types::diagnostic_info::DiagnosticInfo>>,
        >>::decode(stream, decoding_options)?;
        Ok(Self {
            monitored_items,
            diagnostic_infos,
        })
    }
}
