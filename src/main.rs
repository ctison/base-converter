use base_converter::base_to_base;
use clap::{builder::styling::AnsiColor, Parser};
use std::process::ExitCode;

#[derive(Parser)]
#[command(
  author,
  version,
  about,
  arg_required_else_help(true),
  styles(clap::builder::Styles::styled()
  .header(AnsiColor::Green.on_default())
  .usage(AnsiColor::Green.on_default())
  .literal(AnsiColor::Cyan.on_default())
  .placeholder(AnsiColor::Blue.bright(true).on_default()))
)]
struct Cli {
  /// Number in base FROM_BASE to convert in base TO_BASE
  number: String,
  /// Base to convert NUMBER from
  from_base: String,
  /// Base to convert NUMBER to
  to_base: String,
}

fn main() -> ExitCode {
  let cli = Cli::parse();
  match base_to_base(&cli.number, &cli.from_base, &cli.to_base) {
    Err(err) => {
      eprintln!("Error: {err:#}");
      ExitCode::FAILURE
    }
    Ok(result) => {
      println!("{}", result);
      ExitCode::SUCCESS
    }
  }
}
