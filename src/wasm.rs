use wasm_bindgen::prelude::*;

/// Checks if a base is valid by throwing an error if not.
#[wasm_bindgen]
pub fn checkBase(base: &str) -> Result<(), JsError> {
  crate::check_base(base).map_err(|err| JsError::new(&format!("{}", err)))
}

/// Convert a number from any base to an decimal number.
#[wasm_bindgen]
pub fn baseToDecimal(nbr: &str, fromBase: &str) -> Result<usize, JsError> {
  crate::base_to_decimal(nbr, fromBase).map_err(|err| JsError::new(&format!("{}", err)))
}

/// Convert an decimal number to any base.
#[wasm_bindgen]
pub fn decimalToBase(nbr: usize, toBase: &str) -> Result<String, JsError> {
  crate::decimal_to_base(nbr, toBase).map_err(|err| JsError::new(&format!("{}", err)))
}

/// Convert a number from any base to any base.
#[wasm_bindgen]
pub fn baseToBase(nbr: &str, fromBase: &str, toBase: &str) -> Result<String, JsError> {
  crate::base_to_base(nbr, fromBase, toBase).map_err(|err| JsError::new(&format!("{}", err)))
}
