#![no_std]

#[cfg(feature = "wasm-binary")]
#[cfg(not(target_arch = "wasm32"))]
pub use code::WASM_BINARY_OPT as WASM_BINARY;

use sails_rs::{gstd::{debug, msg}, prelude::*};

struct SendValueService(());

#[sails_rs::service]
impl SendValueService {
    pub fn new() -> Self {
        Self(())
    }

    // Service's method (command)
    pub fn send_value(&mut self, to: ActorId, value: u128) {
        msg::send_with_gas(to, "", 0, value).expect("Error in sending the value");
        debug!("The value ({:?}) has been sent to: {:?}", value, to);
    }
}

pub struct SendValueProgram(());

#[sails_rs::program]
impl SendValueProgram {
    // Program's constructor
    pub fn new() -> Self {
        Self(())
    }

    // Exposed service
    pub fn send_value(&self) -> SendValueService {
        SendValueService::new()
    }
}

#[cfg(feature = "wasm-binary")]
#[cfg(not(target_arch = "wasm32"))]
mod code {
    include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
}
