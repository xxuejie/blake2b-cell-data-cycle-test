use rand::{rngs::StdRng, Rng, SeedableRng};
use ckb_testtool::context::Context;
use ckb_testtool::ckb_types::{
    bytes::Bytes,
    core::TransactionBuilder,
    packed::*,
    prelude::*,
};
use ckb_testtool::ckb_hash::new_blake2b;

const MAX_CYCLES: u64 = 50_000_000;


fn main() {
    let seed: u64 = match std::env::var("SEED") {
        Ok(val) => str::parse(&val).expect("parsing number"),
        Err(_) => std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64,
    };
    println!("Seed: {}", seed);

    let mut rng = StdRng::seed_from_u64(seed);

    let contract_bin: Bytes = std::fs::read("test").unwrap().into();

    run(&mut rng, 50 * 1024, contract_bin.clone());
    run(&mut rng, 100 * 1024, contract_bin.clone());
    run(&mut rng, 200 * 1024, contract_bin.clone());
    run(&mut rng, 500 * 1024, contract_bin.clone());
}

fn run(rng: &mut StdRng, size: usize, contract_bin: Bytes) {
    let mut context = Context::default();

    let mut data = vec![0; size];
    rng.fill(&mut data[..]);
    let data: Bytes = data.into();

    let digest: Bytes = {
        let mut hasher = new_blake2b();
        hasher.update(&(data.len() as u32).to_le_bytes());
        hasher.update(&data);
        let mut result = [0u8; 32];
        hasher.finalize(&mut result);
        result.to_vec().into()
    };

    // deploy contract
    let out_point = context.deploy_cell(contract_bin);

    // prepare scripts and cell dep
    let lock_script = context
        .build_script(&out_point, digest)
        .expect("script");

    // prepare input cell
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(((1000 + data.len()) as u64).pack())
            .lock(lock_script.clone())
            .build(),
        data,
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();

    // outputs
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script)
            .build(),
    ];

    let outputs_data = vec![Bytes::new(); 2];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .build();

    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("Size: {} bytes, consume cycles: {}", size, cycles);
}
