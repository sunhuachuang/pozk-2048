mod input;

use ark_circom::zkp::{
    init_bn254_circom_from_bytes, init_bn254_params_from_bytes, multiple_proofs_to_abi_bytes,
    prove_bn254, verify_bn254, decode_multiple_prove_publics
};
use input::{decode_prove_inputs};

const WASM_BYTES: &[u8] = include_bytes!("../../../materials/game2048_60.wasm");
const R1CS_BYTES: &[u8] = include_bytes!("../../../materials/game2048_60.r1cs");
const ZKEY_BYTES: &[u8] = include_bytes!("../../../materials/game2048_60.zkey");

// const WASM_BYTES: &[u8] = include_bytes!("../materials/game2048_60_bls.wasm");
// const R1CS_BYTES: &[u8] = include_bytes!("../materials/game2048_60_bls.r1cs");
// const ZKEY_BYTES: &[u8] = include_bytes!("../materials/game2048_60_bls.zkey");

/// INPUT=http://localhost:9098/tasks/1 cargo run --release
#[tokio::main]
async fn main() {
    let input_path = std::env::var("INPUT").expect("env INPUT missing");
    let bytes = reqwest::get(&input_path)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    // parse inputs and publics
    let mut input_len_bytes = [0u8; 4];
    input_len_bytes.copy_from_slice(&bytes[0..4]);
    let input_len = u32::from_be_bytes(input_len_bytes) as usize;
    let input_bytes = &bytes[4..input_len + 4];
    let publics_bytes = &bytes[input_len + 4..];

    let inputs = decode_prove_inputs(input_bytes).expect("Unable to decode inputs");
    let publics = decode_multiple_prove_publics(publics_bytes, 7).expect("Unable to decode publics");
    assert_eq!(inputs.len(), publics.len());

    let mut proofs = vec![];
    let params = init_bn254_params_from_bytes(ZKEY_BYTES, false).unwrap();
    let mut i = 0;
    for input in inputs {
        let circom = init_bn254_circom_from_bytes(WASM_BYTES, R1CS_BYTES).unwrap();
        let (_pi, proof) = prove_bn254(&params, circom, input).unwrap();

        assert!(verify_bn254(&params.vk, &publics[i], &proof).unwrap());

        proofs.push(proof);
        i += 1;
    }

    let bytes = multiple_proofs_to_abi_bytes(&proofs).unwrap();
    let client = reqwest::Client::new();
    client.post(&input_path).body(bytes).send().await.unwrap();
}
