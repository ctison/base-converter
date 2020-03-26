#[macro_use]
extern crate clap;
use lib::base_to_base;

fn main() {
  let matches = clap_app!(base =>
      (version: "1.0.0")
      (about: "Convert numbers from any to any base")
      (@arg NUMBER: +required "Number to be converted from FROM_BASE to TO_BASE")
      (@arg FROM_BASE: +required "Base in which NUMBER is converted from")
      (@arg TO_BASE: +required "Base to which NUMBER is converted to")
  )
  .get_matches();
  let number = matches.value_of("NUMBER").unwrap();
  let from_base = matches.value_of("FROM_BASE").unwrap();
  let to_base = matches.value_of("TO_BASE").unwrap();
  match base_to_base(number, from_base, to_base) {
    Ok(result) => println!("{}", result),
    Err(err) => println!("{:?}", err),
  }
}
