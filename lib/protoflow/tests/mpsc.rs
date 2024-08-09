// This is free and unencumbered software released into the public domain.

use protoflow::{
    blocks::{Const, Drop},
    runtimes::StdRuntime,
    transports::MpscTransport,
    Runtime, System,
};

#[test]
fn execute_mpsc_transport() -> Result<(), ()> {
    let transport = MpscTransport::new();
    let mut runtime = StdRuntime::new(transport).unwrap();
    let system = System::new(&runtime);
    let constant = system.block(Const {
        output: system.output(),
        value: 42,
    });
    let blackhole = system.block(Drop::new(system.input()));
    system.connect(&constant.output, &blackhole.input);
    let process = runtime.execute(system).unwrap();
    process.join().unwrap();
    Ok(())
}
