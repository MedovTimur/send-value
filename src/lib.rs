#![no_std]
use gstd::{msg, debug, ActorId};

#[no_mangle]
extern "C" fn handle() {
    let to: ActorId = msg::load().expect("Unable to decode");
    msg::send_with_gas(to, "", 0, 10_000_000_000_000).expect("Error in sending the value");
    debug!("The value has been sent to: {:?}", to);

}
