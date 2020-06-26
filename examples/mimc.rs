use ckb_zkp::{
    curve::bn_256::{Bn_256, Fr},
    gadget::mimc::{constants, MiMC},
    math::ToBytes,
    prove, prove_to_bytes,
    scheme::groth16::generate_random_parameters,
    verify, verify_from_bytes, Curve, Gadget, Scheme,
};
use rand::prelude::*;
use std::time::Instant;

/// test for use groth16 & bn_256 & mimc gadget.
fn main() {
    let bytes = vec![1, 2, 3, 4, 5]; // this is your secret.
    let mut rng = thread_rng();

    // TRUSTED SETUP
    println!("TRUSTED SETUP...");
    let constants = constants::<Fr>();
    let c = MiMC::<Fr> {
        xl: None,
        xr: None,
        constants: &constants,
    };
    let params = generate_random_parameters::<Bn_256, _, _>(c, &mut rng).unwrap();

    // you need save this prove key,
    // when prove, use it as a params.
    let mut pk_bytes = vec![];
    params.write(&mut pk_bytes).unwrap();

    // you need save this verify key,
    // when verify, use it as a params.
    let mut vk_bytes = vec![];
    params.vk.write(&mut vk_bytes).unwrap();

    let new_bytes = bytes.clone();
    println!("START PROVE...");
    let proof = prove(
        Gadget::MiMC(new_bytes),
        Scheme::Groth16,
        Curve::Bn_256,
        &pk_bytes,
        rng,
    )
    .unwrap();

    println!("START VERIFY...");
    let is_ok = verify(proof, &vk_bytes);
    assert!(is_ok);

    println!("ANOTHER USE BYTES START PROVE...");
    let p_start = Instant::now();
    let proof_bytes = prove_to_bytes(
        Gadget::MiMC(bytes),
        Scheme::Groth16,
        Curve::Bn_256,
        &pk_bytes,
        rng,
    )
    .unwrap();
    let p_time = p_start.elapsed();
    println!("PROVE TIME: {:?}", p_time);

    println!("PROOF FILE LENGTH: {}", proof_bytes.len());

    println!("ANOTHER USE BYTES START VERIFY...");
    let v_start = Instant::now();
    let is_ok2 = verify_from_bytes(&proof_bytes, &vk_bytes);
    let v_time = v_start.elapsed();
    println!("VERIFY TIME: {:?}", v_time);
    assert!(is_ok2);

    println!("all is ok");
}
