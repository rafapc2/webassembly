mod utils;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]

static ALLOC:wee_alloc:Wee_Alloc
#[wasm_bindgen]

pub fn greet(){
    alert("Hola Rust Web Assembly")
}