use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Rectangle{
    pub length: u32,
    pub width: u32,
}

#[wasm_bindgen]
pub fn add_one() -> Rectangle {
    let res = Rectangle{
        length: 8,
        width: 9
    };
    res
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}
