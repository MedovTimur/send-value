use sails_rs::{calls::*, gtest::calls::*};

use send_value_client::traits::*;

const ACTOR_ID: u64 = 42;

#[tokio::test]
async fn send_value() {
    let remoting = GTestRemoting::new(ACTOR_ID.into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(send_value::WASM_BINARY);

    let program_factory = send_value_client::SendValueFactory::new(remoting.clone());

    let program_id = program_factory
        .new() // Call program's constructor (see src/lib.rs:27)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = send_value_client::SendValue::new(remoting.clone());

    let value = 10_000_000_000_000;
    remoting.clone().system().mint_to(program_id, value*10);

    let old_balance = remoting.clone().system().balance_of(remoting.actor_id());

    service_client
        .send_value(ACTOR_ID.into(), value) // Call service's method (see src/lib.rs:17)
        .send_recv(program_id)
        .await
        .unwrap();

    let new_balance = remoting.clone().system().balance_of(remoting.actor_id());
    println!("\n NEW BALANCE: {:?} \n", new_balance);
    // assert_eq!(new_balance - old_balance, value);
}
