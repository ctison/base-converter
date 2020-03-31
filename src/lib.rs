mod errs;
use std::error::Error;

pub static BASE2: &str = "01";
pub static BASE8: &str = "01234567";
pub static BASE10: &str = "0123456789";
pub static BASE16: &str = "0123456789ABCDEF";

fn check_base(base: &str) -> Result<(), Box<dyn Error>> {
  if base.len() < 2 {
    return Err(Box::new(errs::BaseLenTooShort(String::from(base))));
  }
  for c in base.chars() {
    if base.chars().filter(|c2| &c == c2).count() > 1 {
      return Err(Box::new(errs::DuplicateCharInBase {
        base: String::from(base),
        c,
      }));
    }
  }
  Ok(())
}

pub fn base_to_decimal(nbr: &str, from_base: &str) -> Result<usize, Box<dyn Error>> {
  check_base(from_base)?;
  let baselen = from_base.len();
  let mut result: usize = 0;
  for (c, i) in nbr.chars().zip((0..nbr.chars().count() as u32).rev()) {
    let x = from_base.chars().position(|x| x == c).ok_or_else(|| {
      Box::new(errs::CharNotFoundInBase {
        c,
        base: String::from(from_base),
      })
    })?;
    result += x
      * baselen
        .checked_pow(i)
        .ok_or_else(|| errs::ConversionOverflow {
          base: String::from(from_base),
          baselen,
          power: i,
        })? as usize;
  }
  Ok(result)
}

pub fn decimal_to_base(mut x: usize, to_base: &str) -> Result<String, Box<dyn Error>> {
  check_base(to_base)?;
  if x == 0 {
    return Ok(String::from("0"));
  }
  let baselen = to_base.chars().count();
  let mut result = String::new();
  while x > 0 {
    result.push(nbr_to_char(x % baselen, to_base)?);
    x /= baselen;
  }
  Ok(result.chars().rev().collect())
}

fn nbr_to_char(nbr: usize, base: &str) -> Result<char, String> {
  base.chars().nth(nbr).ok_or_else(|| {
    format!(
      "number {} should be inferior of base len: '{}'.chars().count() == {}",
      nbr,
      base,
      base.chars().count()
    )
  })
}

pub fn base_to_base(x: &str, from_base: &str, to_base: &str) -> Result<String, Box<dyn Error>> {
  let y = base_to_decimal(x, from_base)?;
  let z = decimal_to_base(y, to_base)?;
  Ok(z)
}

#[cfg(test)]
mod tests {
  use super::*;

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
  }
}
