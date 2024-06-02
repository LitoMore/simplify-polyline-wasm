use simplify_polyline::{point, simplify as simp, Point};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn simplify(points: JsValue, tolerance: f32, high_quality: bool) -> JsValue {
    let p: Vec<(f32, f32)> = serde_wasm_bindgen::from_value(points).unwrap();
    let p: Vec<Point<2, f32>> = p.into_iter().map(|(x, y)| point!(x, y)).collect();
    let mut p = simp(&p, tolerance, high_quality);
    p.reverse();
    let p: Vec<(f32, f32)> = p.into_iter().map(|p| (p.vec[0], p.vec[1])).collect();
    serde_wasm_bindgen::to_value(&p).unwrap()
}
