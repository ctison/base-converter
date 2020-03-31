use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct CharNotFoundInBase {
  pub base: String,
  pub c: char,
}

impl Display for CharNotFoundInBase {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "char '{}' not found in base '{}'", self.c, self.base)
  }
}

impl Error for CharNotFoundInBase {}

#[derive(Debug)]
pub struct ConversionOverflow {
  pub base: String,
  pub baselen: usize,
  pub power: u32,
}

impl Display for ConversionOverflow {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(
      f,
      "base '{}' of length {} ** {} overflowed",
      self.base, self.baselen, self.power
    )
  }
}

impl Error for ConversionOverflow {}

#[derive(Debug)]
pub struct BaseLenTooShort(pub String);

impl Display for BaseLenTooShort {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "base '{}' length must be at least be 2", self.0,)
  }
}

impl Error for BaseLenTooShort {}

#[derive(Debug)]
pub struct DuplicateCharInBase {
  pub base: String,
  pub c: char,
}

impl Display for DuplicateCharInBase {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(
      f,
      "base '{}' has at least 2 occurences of char '{}'",
      self.base, self.c
    )
  }
}

impl Error for DuplicateCharInBase {}
