use std::collections::HashMap;

use ark_circom::zkp::Input;
use ethabi::{decode, encode, ethereum_types::U256, ParamType, Token};
use num_bigint::{BigInt, Sign};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Game2048Input {
    pub board: Vec<Vec<u8>>,
    #[serde(rename = "packedBoard")]
    pub packed_board: Vec<String>,
    #[serde(rename = "packedDir")]
    pub packed_dir: String,
    pub direction: Vec<u8>,
    pub address: String,
    pub step: u64,
    #[serde(rename = "stepAfter")]
    pub step_after: u64,
    pub nonce: String,
}

const BOARD_STEP: u32 = 32;
const BOARD_LEN: usize = 16;
const DIR_STEP: u32 = 4;
const DIR_LEN: usize = 60;

fn unpack(t: Token, step: u32, len: usize) -> Vec<BigInt> {
    let mut d = t.into_uint().unwrap_or(U256::zero());
    let step = U256::from(step);
    let mut items = vec![];

    loop {
        if d < step {
            items.push(BigInt::from(d.as_u64()));
            break;
        }
        let (next, n) = d.div_mod(step);

        d = next;
        items.push(BigInt::from(n.as_u64()));
    }

    if items.len() < len {
        for _ in items.len()..len {
            items.push(BigInt::from(0));
        }
    }

    items.reverse();
    return items;
}

#[allow(dead_code)]
pub fn encode_prove_inputs(inputs: &[Game2048Input]) -> String {
    let mut t_inputs = vec![];
    for input in inputs {
        let mut packed_board = vec![];
        for x in input.packed_board.iter() {
            packed_board.push(Token::Uint(U256::from_dec_str(x).unwrap()))
        }

        let packed_dir = Token::Uint(U256::from_dec_str(&input.packed_dir).unwrap());
        let address = Token::Uint(U256::from_dec_str(&input.address).unwrap());
        let step = Token::Uint(U256::from(input.step));
        let step_after = Token::Uint(U256::from(input.step_after));
        let nonce = Token::Uint(U256::from_dec_str(&input.nonce).unwrap());

        packed_board.extend([
            packed_dir,
            address,
            step,
            step_after,
            nonce,
        ]);

        t_inputs.push(Token::FixedArray(packed_board));
    }

    let bytes = encode(&[Token::Array(t_inputs)]);
    format!("0x{}", hex::encode(&bytes))
}

pub fn decode_prove_inputs(bytes: &[u8]) -> Result<Vec<Input>, anyhow::Error> {
    let mut input_tokens = decode(
        &[ParamType::Array(Box::new(ParamType::Tuple(vec![
            ParamType::Uint(256),                             // packed_board_1
            ParamType::Uint(256),                             // packed_board_2
            ParamType::Uint(256),                             // packed_dir
            ParamType::Uint(256),                             // address
            ParamType::Uint(256),                             // step
            ParamType::Uint(256),                             // step_after
            ParamType::Uint(256),                             // nonce
        ])))],
        bytes,
    )?;
    let tokens = input_tokens.pop().unwrap().into_array().unwrap();

    let f_uint = |token: Token| -> BigInt {
        let mut bytes = [0u8; 32];
        token.into_uint().unwrap().to_big_endian(&mut bytes);
        BigInt::from_bytes_be(Sign::Plus, &bytes)
    };

    let mut inputs = vec![];
    for t_token in tokens {
        let token = t_token.into_tuple().unwrap();

        let mut board = vec![];
        let mut packed_board = vec![];
        let packed_board_1 = token[0].clone();
        let packed_board_2 = token[1].clone();
        board.extend(unpack(packed_board_1.clone(), BOARD_STEP, BOARD_LEN));
        board.extend(unpack(packed_board_2.clone(), BOARD_STEP, BOARD_LEN));
        packed_board.push(f_uint(packed_board_1));
        packed_board.push(f_uint(packed_board_2));

        let packed_token = token[2].clone();
        let direction = unpack(packed_token.clone(), DIR_STEP, DIR_LEN);
        let packed_dir = f_uint(packed_token);

        let address = f_uint(token[3].clone());
        let step = f_uint(token[4].clone());
        let step_after = f_uint(token[5].clone());
        let nonce = f_uint(token[6].clone());

        let mut maps = HashMap::new();
        maps.insert("board".to_string(), board);
        maps.insert("packedBoard".to_string(), packed_board);
        maps.insert("packedDir".to_string(), vec![packed_dir]);
        maps.insert("direction".to_string(), direction);
        maps.insert("address".to_string(), vec![address]);
        maps.insert("step".to_string(), vec![step]);
        maps.insert("stepAfter".to_string(), vec![step_after]);
        maps.insert("nonce".to_string(), vec![nonce]);

        inputs.push(Input { maps });
    }

    Ok(inputs)
}

#[cfg(test)]
mod test {
    use super::*;
    use ethabi::ethereum_types::U256;

    fn pack_board(board: &[u8]) -> U256 {
        let mut packed = U256::zero();
        let step = U256::from(32u32);
        for b in board {
            packed = packed * step + U256::from(*b);
        }
        return packed;
    }

    fn pack_direction(directions: &[u8]) -> U256 {
        let mut packed = U256::zero();
        let step = U256::from(4u32);
        for d in directions {
            packed = packed * step + U256::from(*d);
        }
        return packed;
    }

    fn unpack_direction(directions: &str) -> Vec<u8> {
        let mut d = U256::from_dec_str(directions).unwrap();
        let step = U256::from(4u32);
        let mut items = vec![];

        loop {
            if d < step {
                items.push(d.as_u64() as u8);
                break;
            }
            let (next, n) = d.div_mod(step);

            d = next;
            items.push(n.as_u64() as u8);
        }
        if items.len() < 60 {
            for _ in items.len()..60 {
                items.push(0);
            }
        }
        items.reverse();
        println!("{:?}", items);

        return items;
    }

    #[test]
    fn test_serialize() {
        let input = r##"
        {
            "board": [
                [0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
                [0, 2, 4, 6, 0, 1, 2, 4, 0, 0, 0, 5, 0, 0, 1, 3]
            ],
            "packedBoard": ["35218731827200", "2515675923718842875939"],
            "packedDir": "311800516178808354245949615821275955",
            "direction": [0, 3, 3, 0, 0, 0, 3, 0, 3, 3, 0, 3, 3, 0, 3, 0, 2, 0, 3, 3, 0, 2, 0, 3, 0, 0, 3, 0, 2, 0, 3, 3, 0, 0, 3, 0, 3, 3, 0, 3, 3, 3, 3, 3, 0, 0, 3, 2, 3, 3, 0, 3, 3, 0, 0, 3, 0, 3, 0, 3],
            "address": "6789",
            "step": 0,
            "stepAfter": 60,
            "nonce": "456"
        }
        "##;

        let input: Game2048Input = serde_json::from_str(input).unwrap();
        for b in &input.board {
            println!("{}", pack_board(b));
        }
        println!("{:?}", input.packed_board);
        println!("{}", pack_direction(&input.direction));
        println!("{}", input.packed_dir);

        println!("{:?}", input.direction);
        unpack_direction(&input.packed_dir);

        let hex = encode_prove_inputs(&[input.clone(), input]);
        println!("{}", hex);

        let input_hex = hex.trim_start_matches("0x");
        let input_bytes = hex::decode(input_hex).expect("Unable to decode input file");
        decode_prove_inputs(&input_bytes).expect("Unable to decode input");
    }
}
