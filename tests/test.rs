use gstd::ActorId;
use gtest::{Log, ProgramBuilder, System};

const USER: u64 = 3;

#[test]
fn test() {
    let system = System::new();
    system.init_logger();

    let program =
        ProgramBuilder::from_file("target/wasm32-unknown-unknown/debug/send_value.opt.wasm")
            .build(&system);

    let id: ActorId = USER.into();
    let value = 2000_000_000_000_000;
    system.mint_to(id, value);

    let msg_id = program.send_bytes(USER, []);
    let res = system.run_next_block();
    assert!(res.succeed.contains(&msg_id));

    system.transfer(id, program.id(), value/2, true);
    let old_balance = system.balance_of(id);
    println!("\n OLD BALANCE: {:?} \n", old_balance);
    let msg_id = program.send(USER,id);
    println!("\n HERE 1 \n");
    let res = system.run_next_block();
    println!("\n HERE 2 \n");
    assert!(res.succeed.contains(&msg_id));

    let log = Log::builder().dest(id);
    let mailbox = system.get_mailbox(id);
    assert!(mailbox.contains(&log));
    assert!(mailbox.claim_value(log).is_ok());
    let new_balance = system.balance_of(id);
    assert_eq!(
        new_balance,
        old_balance - res.spent_value() + 10_000_000_000_000
    );
}
