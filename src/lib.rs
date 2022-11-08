//! Convert numbers from base to base.

#[cfg(feature = "wasm")]
pub mod wasm;

use anyhow::{ensure, Result};
use thiserror::Error;

/// 01
pub static BASE2: &str = "01";
/// 01234567
pub static BASE8: &str = "01234567";
/// 0123456789
pub static BASE10: &str = "0123456789";
/// 0123456789ABCDEF
pub static BASE16: &str = "0123456789ABCDEF";

/// Errors that can be returned by [check_base] and any function that takes
/// a base as argument.
#[derive(Error, Debug)]
pub enum CheckBaseError {
  /// Error occurring when a base length is inferior at 2.
  #[error("length of base '{0}' must be at least 2")]
  BaseLenTooShort(String),
  /// Error occurring when a character appears more than once in a base.
  #[error("base '{base}' has at least 2 occurrences of char '{c}'")]
  DuplicateCharInBase { base: String, c: char },
}

/// Checks if a base is valid.
///
/// # Errors
/// - [CheckBaseError]
///
/// # Examples
/// ```
/// use base_converter::check_base;
/// check_base("0123456789").expect("valid hardcoded base");
/// ```
/// ```
/// use base_converter::{check_base, CheckBaseError};
/// let err = check_base("0").unwrap_err(); // Base must have at least 2 characters
/// assert_eq!(format!("{}", err), format!("{}", CheckBaseError::BaseLenTooShort("0".into())));
/// ```
pub fn check_base(base: &str) -> Result<()> {
  ensure!(
    base.chars().count() >= 2,
    CheckBaseError::BaseLenTooShort(base.into())
  );
  for c in base.chars() {
    ensure!(
      base.chars().filter(|c2| &c == c2).count() == 1,
      CheckBaseError::DuplicateCharInBase {
        base: base.into(),
        c,
      }
    )
  }
  Ok(())
}

/// Errors that can be returned when converting a number between bases.
#[derive(Error, Debug)]
pub enum ConversionError {
  /// Error occurring when a char in a number is not found from the base it should be in.
  #[error("char '{c}' not found in base '{base}'")]
  CharNotFoundInBase { base: String, c: char },
  /// Error occurring when a number encoded in a base cannot be represented in an [usize].
  #[error("base '{base}' of length {base_length} ** {power} overflowed")]
  ConversionOverflow {
    base: String,
    base_length: usize,
    power: u32,
  },
}

/// Converts a number from any base to an [usize].
///
/// # Errors
/// - [CheckBaseError]
/// - [ConversionError]
///
/// # Examples
/// ```
/// use base_converter::base_to_decimal;
/// let nbr = base_to_decimal("101010", "01").unwrap();
/// assert_eq!(nbr, 42);
/// ```
pub fn base_to_decimal(nbr: &str, from_base: &str) -> Result<usize> {
  check_base(from_base)?;
  let base_length = from_base.chars().count();
  let mut result: usize = 0;
  for (c, i) in nbr.chars().zip((0..nbr.chars().count() as u32).rev()) {
    let x = from_base.chars().position(|x| x == c).ok_or_else(|| {
      ConversionError::CharNotFoundInBase {
        base: from_base.into(),
        c,
      }
    })?;
    result += x
      * base_length
        .checked_pow(i)
        .ok_or_else(|| ConversionError::ConversionOverflow {
          base: from_base.into(),
          base_length,
          power: i,
        })?;
  }
  Ok(result)
}

/// Converts an [usize] to another base.
///
/// # Errors
/// - [CheckBaseError]
///
/// # Examples
/// ```
/// use base_converter::decimal_to_base;
/// let nbr = decimal_to_base(51966, "0123456789ABCDEF").unwrap();
/// assert_eq!(nbr, "CAFE");
/// ```
pub fn decimal_to_base(mut nbr: usize, to_base: &str) -> Result<String> {
  check_base(to_base)?;
  if nbr == 0 {
    return Ok(to_base.chars().next().unwrap().into());
  }
  let base_length = to_base.chars().count();
  let mut result = String::new();
  while nbr > 0 {
    result.push(to_base.chars().nth(nbr % base_length).unwrap());
    nbr /= base_length;
  }
  Ok(result.chars().rev().collect())
}

/// Converts a number from any base to any other base.
///
/// # Errors
/// - [CheckBaseError]
/// - [ConversionError]
///
/// # Examples
/// ```
/// use base_converter::base_to_base;
/// let nbr = base_to_base("🚀🚀🚀🦀🚀🦀🦀🚀🚀🦀🚀🚀🦀🦀🦀🚀🚀🦀🦀🦀🚀🦀🚀🦀🚀🚀🦀🦀🦀🦀🚀🚀🚀🚀🦀🚀🚀🦀🚀🚀🦀🦀🦀🚀🦀🦀🦀🦀", "🦀🚀", "abcdefghijklmnopqrstuvwxyz !🦀").unwrap();
/// assert_eq!(nbr, "rust ftw 🦀")
/// ```
pub fn base_to_base(nbr: &str, from_base: &str, to_base: &str) -> Result<String> {
  let nbr = base_to_decimal(nbr, from_base)?;
  let nbr = decimal_to_base(nbr, to_base)?;
  Ok(nbr)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_check_base() {
    struct TestCase {
      base: &'static str,
      err: CheckBaseError,
    }
    let test_cases = vec![
      TestCase {
        base: "",
        err: CheckBaseError::BaseLenTooShort("".into()),
      },
      TestCase {
        base: "x",
        err: CheckBaseError::BaseLenTooShort("x".into()),
      },
      TestCase {
        base: "xx",
        err: CheckBaseError::DuplicateCharInBase {
          base: "xx".into(),
          c: 'x',
        },
      },
    ];
    for test_case in &test_cases {
      assert_eq!(
        format!("{}", check_base(test_case.base).unwrap_err()),
        format!("{}", test_case.err)
      );
    }
  }

  #[test]
  fn test_base_to_decimal() {
    assert_eq!(51966, base_to_decimal("CAFE", BASE16).unwrap());
    assert_eq!(42, base_to_decimal("101010", BASE2).unwrap());
    assert_eq!(0, base_to_decimal("0", BASE8).unwrap());
    assert_eq!(0, base_to_decimal("", BASE8).unwrap());
  }

  #[test]
  fn test_decimal_to_base() {
    assert_eq!("CAFE", decimal_to_base(51966, BASE16).unwrap());
    assert_eq!("0", decimal_to_base(0, BASE8).unwrap());
    assert_eq!("x", decimal_to_base(0, "xyz").unwrap());
  }

  #[test]
  fn test_base_to_base() {
    let err = base_to_base("25", "01234", "1123").unwrap_err();
    assert_eq!(
      format!("{}", err),
      format!(
        "{}",
        ConversionError::CharNotFoundInBase {
          base: "01234".into(),
          c: '5'
        }
      )
    );
  }
}
