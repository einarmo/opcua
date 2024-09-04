// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock

//! The OPC UA Core module holds functionality that is common to server and clients that make use of OPC UA.
//! It contains message chunking, cryptography / pki, communications and standard handshake messages.

#[macro_export]
macro_rules! supported_message_as {
    ($v: expr, $i: ident) => {
        if let SupportedMessage::$i(value) = $v {
            *value
        } else {
            panic!("Cannot convert to {:?}", stringify!($i));
        }
    };
}

/// Contains debugging utility helper functions
pub mod debug {
    use log::{log_enabled, trace};

    /// Prints out the content of a slice in hex and visible char format to aid debugging. Format
    /// is similar to corresponding functionality in node-opcua
    pub fn log_buffer(message: &str, buf: &[u8]) {
        // No point doing anything unless debug level is on
        if !log_enabled!(target: "hex", log::Level::Trace) {
            return;
        }

        let line_len = 32;
        let len = buf.len();
        let last_line_padding = ((len / line_len) + 1) * line_len - len;

        trace!(target: "hex", "{}", message);

        let mut char_line = String::new();
        let mut hex_line = format!("{:08x}: ", 0);

        for (i, b) in buf.iter().enumerate() {
            let value = { *b };
            if i > 0 && i % line_len == 0 {
                trace!(target: "hex", "{} {}", hex_line, char_line);
                hex_line = format!("{:08}: ", i);
                char_line.clear();
            }
            hex_line = format!("{} {:02x}", hex_line, value);
            char_line.push(if (32..=126).contains(&value) {
                value as char
            } else {
                '.'
            });
        }
        if last_line_padding > 0 {
            for _ in 0..last_line_padding {
                hex_line.push_str("   ");
            }
            trace!(target: "hex", "{} {}", hex_line, char_line);
        }
    }
}

#[cfg(test)]
pub mod tests;

pub mod constants {
    /// Default OPC UA port number. Used by a discovery server. Other servers would normally run
    /// on a different port. So OPC UA for Rust does not use this nr by default but it is used
    /// implicitly in opc.tcp:// urls and elsewhere.
    pub const DEFAULT_OPC_UA_SERVER_PORT: u16 = 4840;
}

pub mod comms;
pub mod config;
pub mod handle;
#[rustfmt::skip]
pub mod supported_message;

pub use supported_message::SupportedMessage;

/// Tracing macro for obtaining a lock on a `Mutex`. Sometimes deadlocks can happen in code,
/// and if they do, this macro is useful for finding out where they happened.
#[macro_export]
macro_rules! trace_lock {
    ( $x:expr ) => {
        {
//            use std::thread;
//            trace!("Thread {:?}, {} locking at {}, line {}", thread::current().id(), stringify!($x), file!(), line!());
            let v = $x.lock();
//            trace!("Thread {:?}, {} lock completed", thread::current().id(), stringify!($x));
            v
        }
    }
}

/// Tracing macro for obtaining a read lock on a `RwLock`.
#[macro_export]
macro_rules! trace_read_lock {
    ( $x:expr ) => {
        {
//            use std::thread;
//            trace!("Thread {:?}, {} read locking at {}, line {}", thread::current().id(), stringify!($x), file!(), line!());
            let v = $x.read();
//            trace!("Thread {:?}, {} read lock completed", thread::current().id(), stringify!($x));
            v
        }
    }
}

/// Tracing macro for obtaining a write lock on a `RwLock`.
#[macro_export]
macro_rules! trace_write_lock {
    ( $x:expr ) => {
        {
//            use std::thread;
//            trace!("Thread {:?}, {} write locking at {}, line {}", thread::current().id(), stringify!($x), file!(), line!());
            let v = $x.write();
//            trace!("Thread {:?}, {} write lock completed", thread::current().id(), stringify!($x));
            v
        }
    }
}

// Synchronization structs. This is a wrapper mod around `parking_lot` types so opcua users don't have
// to reference that other crate.
pub mod sync {
    pub type RwLock<T> = parking_lot::RwLock<T>;
    pub type Mutex<T> = parking_lot::Mutex<T>;
}
