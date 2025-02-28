use wasm_minimal_protocol::*;

initiate_protocol!();

#[wasm_func]
pub fn debug_print(arg: &[u8]) -> Vec<u8> {
    format!("{arg:?}").into_bytes()
}
