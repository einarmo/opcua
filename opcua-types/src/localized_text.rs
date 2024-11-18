// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock

//! Contains the definition of `LocalizedText`.
use std::{
    fmt,
    io::{Read, Write},
};

use crate::{encoding::*, string::*};

/// JSON encoding
///  Locale    The Localeportion of LocalizedTextvalues shall be encoded as a JSON string
///
/// Text       The Textportion of LocalizedTextvalues shall be encoded as a JSON string.

/// A human readable text with an optional locale identifier.
#[derive(PartialEq, Default, Debug, Clone)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all = "PascalCase"))]
pub struct LocalizedText {
    /// The locale. Omitted from stream if null or empty
    #[cfg_attr(
        feature = "json",
        serde(skip_serializing_if = "UAString::is_null", default)
    )]
    pub locale: UAString,
    /// The text in the specified locale. Omitted frmo stream if null or empty.
    #[cfg_attr(
        feature = "json",
        serde(skip_serializing_if = "UAString::is_null", default)
    )]
    pub text: UAString,
}

#[cfg(feature = "json")]
mod json {
    use std::io::{Read, Write};

    use struson::{
        reader::{JsonReader, JsonStreamReader},
        writer::{JsonStreamWriter, JsonWriter},
    };

    use crate::json::{Context, JsonDecodable, JsonEncodable};

    use super::{EncodingResult, LocalizedText, UAString};

    impl JsonEncodable for LocalizedText {
        fn encode(
            &self,
            stream: &mut JsonStreamWriter<&mut dyn Write>,
            ctx: &Context<'_>,
        ) -> EncodingResult<()> {
            stream.begin_object()?;
            if !self.locale.is_null_json() {
                stream.name("Locale")?;
                self.locale.encode(stream, ctx)?;
            }
            if !self.text.is_null_json() {
                stream.name("Text")?;
                self.text.encode(stream, ctx)?;
            }
            stream.end_object()?;
            Ok(())
        }

        fn is_null_json(&self) -> bool {
            self.text.is_null()
        }
    }

    impl JsonDecodable for LocalizedText {
        fn decode(
            stream: &mut JsonStreamReader<&mut dyn Read>,
            ctx: &Context<'_>,
        ) -> EncodingResult<Self> {
            stream.begin_object()?;
            let mut locale: Option<UAString> = None;
            let mut text: Option<UAString> = None;
            while stream.has_next()? {
                match stream.next_name()? {
                    "Locale" => locale = Some(JsonDecodable::decode(stream, ctx)?),
                    "Text" => text = Some(JsonDecodable::decode(stream, ctx)?),
                    _ => stream.skip_value()?,
                }
            }
            stream.end_object()?;
            Ok(Self {
                locale: locale.unwrap_or_default(),
                text: text.unwrap_or_default(),
            })
        }
    }
}

impl<'a> From<&'a str> for LocalizedText {
    fn from(value: &'a str) -> Self {
        Self {
            locale: UAString::null(),
            text: UAString::from(value),
        }
    }
}

impl From<&String> for LocalizedText {
    fn from(value: &String) -> Self {
        Self {
            locale: UAString::null(),
            text: UAString::from(value),
        }
    }
}

impl From<String> for LocalizedText {
    fn from(value: String) -> Self {
        Self {
            locale: UAString::null(),
            text: UAString::from(value),
        }
    }
}

impl fmt::Display for LocalizedText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl BinaryEncodable for LocalizedText {
    fn byte_len(&self) -> usize {
        let mut size = 1;
        if !self.locale.is_empty() {
            size += self.locale.byte_len();
        }
        if !self.text.is_empty() {
            size += self.text.byte_len();
        }
        size
    }

    fn encode<S: Write + ?Sized>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        // A bit mask that indicates which fields are present in the stream.
        // The mask has the following bits:
        // 0x01    Locale
        // 0x02    Text
        let mut encoding_mask: u8 = 0;
        if !self.locale.is_empty() {
            encoding_mask |= 0x1;
        }
        if !self.text.is_empty() {
            encoding_mask |= 0x2;
        }
        size += encoding_mask.encode(stream)?;
        if !self.locale.is_empty() {
            size += self.locale.encode(stream)?;
        }
        if !self.text.is_empty() {
            size += self.text.encode(stream)?;
        }
        Ok(size)
    }
}

impl BinaryDecodable for LocalizedText {
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let encoding_mask = u8::decode(stream, decoding_options)?;
        let locale = if encoding_mask & 0x1 != 0 {
            UAString::decode(stream, decoding_options)?
        } else {
            UAString::null()
        };
        let text = if encoding_mask & 0x2 != 0 {
            UAString::decode(stream, decoding_options)?
        } else {
            UAString::null()
        };
        Ok(LocalizedText { locale, text })
    }
}

impl LocalizedText {
    pub fn new(locale: &str, text: &str) -> LocalizedText {
        LocalizedText {
            locale: UAString::from(locale),
            text: UAString::from(text),
        }
    }

    pub fn null() -> LocalizedText {
        LocalizedText {
            locale: UAString::null(),
            text: UAString::null(),
        }
    }
}
