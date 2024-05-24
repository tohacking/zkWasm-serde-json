use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::wasm_input;
use zkwasm_rust_sdk::wasm_output;

// #[derive(Deserialize, Debug)]
// struct User {
//     fingerprint: String,
//     location: String,
// }

use parity_scale_codec::Decode;
use parity_scale_codec::Encode;
#[cfg(not(feature = "derive"))]
use parity_scale_codec_derive::Decode;
#[cfg(not(feature = "derive"))]
use parity_scale_codec_derive::Encode;

#[derive(Debug, PartialEq, Encode, Decode, Clone, Copy)]
enum EnumType {
    #[codec(index = 15)]
    A,
    B(u32, u64),
    C {
        a: u32,
        b: u64,
    },
}

// pub fn dummy_json_operation() {
//     let j = "
//     {
//         \"fingerprint\": \"0xF9BA143B95FF6D82\",
//         \"location\": \"Menlo Park, CA\"
//     }";

//     let u: User = serde_json::from_str(j).unwrap();
// }

pub fn dummy_codec_operation() {
    let a = EnumType::A;
    let b = EnumType::B(1, 2);
    let c = EnumType::C { a: 1, b: 2 };

    a.using_encoded(|ref slice| {
        assert_eq!(slice, &b"\x0f");
    });

    b.using_encoded(|ref slice| {
        assert_eq!(slice, &b"\x01\x01\0\0\0\x02\0\0\0\0\0\0\0");

        let decoded_b = EnumType::decode(&mut &slice[..]).ok().unwrap();
        assert_eq!(decoded_b, EnumType::B(1, 2));

        if let EnumType::B(first, second) = decoded_b {
            // Use the first and second items here
            unsafe {
                zkwasm_rust_sdk::dbg!("b.0: {}\n", first);
                zkwasm_rust_sdk::dbg!("b.1: {}\n", second);
            }
        }
    });

    c.using_encoded(|ref slice| {
        assert_eq!(slice, &b"\x02\x01\0\0\0\x02\0\0\0\0\0\0\0");
    });
}

#[wasm_bindgen]
pub fn zkmain() -> i64 {
    // specify the public inputs
    // let total_steps: u64 = unsafe { wasm_input(1) };
    // let current_position: u64 = unsafe { wasm_input(1) };
    // dummy_json_operation();
    dummy_codec_operation();
    // let init_param = RustInitializationParameters {
    //     total_steps,
    //     current_position,
    // };
    // // init game
    // initialize_game(&init_param);

    // // specify the private inputs
    // let input_length = unsafe { wasm_input(0) };

    // for _i in 0..input_length {
    //     let input_length = unsafe { wasm_input(0) };
    //     // step(input_length);
    // }

    // unsafe {
    //     let final_game_state: RustGameState = _get_game_state();

    //     zkwasm_rust_sdk::dbg!("final_game_state: {}\n", final_game_state);

    //     // specify the output
    //     wasm_output(final_game_state.total_steps as u64);
    //     wasm_output(final_game_state.current_position as u64);
    // }

    0
}
