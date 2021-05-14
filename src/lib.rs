use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() -> f64{
    let mut x:f64 = 0.0; 
    for _i in 1..100000{
        x = x + 1.0;
    }
    return x
}
