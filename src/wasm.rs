use wasm_bindgen::prelude::*;

/// Checks if a base is valid by throwing an error if not.
#[wasm_bindgen]
pub fn check_base(base: &str) -> Result<(), JsError> {
  crate::check_base(base).map_err(|err| JsError::new(&format!("{}", err)))
}

/// Convert a number from any base to an decimal number.
#[wasm_bindgen]
pub fn base_to_decimal(nbr: &str, from_base: &str) -> Result<usize, JsError> {
  crate::base_to_decimal(nbr, from_base).map_err(|err| JsError::new(&format!("{}", err)))
}

/// Convert an decimal number to any base.
#[wasm_bindgen]
pub fn decimal_to_base(nbr: usize, to_base: &str) -> Result<String, JsError> {
  crate::decimal_to_base(nbr, to_base).map_err(|err| JsError::new(&format!("{}", err)))
}

/// Convert a number from any base to any base.
#[wasm_bindgen]
pub fn base_to_base(nbr: &str, from_base: &str, to_base: &str) -> Result<String, JsError> {
  crate::base_to_base(nbr, from_base, to_base).map_err(|err| JsError::new(&format!("{}", err)))
}
