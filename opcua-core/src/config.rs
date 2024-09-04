// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::result::Result;

use serde;
use serde_yaml;

use opcua_types::{
    service_types::{ApplicationDescription, ApplicationType},
    LocalizedText, UAString,
};

/// Error returned from saving or loading config objects.
#[derive(Debug)]
pub enum ConfigError {
    // TODO: Make the config validation actually return something useful.
    /// Configuration is invalid.
    ConfigInvalid,
    /// Reading or writing file failed.
    IO(std::io::Error),
    /// Failed to serialize or deserialize config object.
    Yaml(serde_yaml::Error),
}

impl From<std::io::Error> for ConfigError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::Yaml(value)
    }
}

/// A trait that handles the loading / saving and validity of configuration information for a
/// client and/or server.
pub trait Config: serde::Serialize {
    fn save(&self, path: &Path) -> Result<(), ConfigError> {
        if !self.is_valid() {
            return Err(ConfigError::ConfigInvalid);
        }
        let s = serde_yaml::to_string(&self)?;
        let mut f = File::create(path)?;
        f.write_all(s.as_bytes())?;
        Ok(())
    }

    fn load<A>(path: &Path) -> Result<A, ConfigError>
    where
        for<'de> A: Config + serde::Deserialize<'de>,
    {
        let mut f = File::open(path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(serde_yaml::from_str(&s)?)
    }

    fn is_valid(&self) -> bool;

    fn application_name(&self) -> UAString;

    fn application_uri(&self) -> UAString;

    fn product_uri(&self) -> UAString;

    fn application_type(&self) -> ApplicationType;

    fn discovery_urls(&self) -> Option<Vec<UAString>> {
        None
    }

    fn application_description(&self) -> ApplicationDescription {
        ApplicationDescription {
            application_uri: self.application_uri(),
            application_name: LocalizedText::new("", self.application_name().as_ref()),
            application_type: self.application_type(),
            product_uri: self.product_uri(),
            gateway_server_uri: UAString::null(),
            discovery_profile_uri: UAString::null(),
            discovery_urls: self.discovery_urls(),
        }
    }
}
