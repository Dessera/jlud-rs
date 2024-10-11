use clap::Parser;

#[derive(Parser)]
pub struct AuthArgs {
  /// Specify the user authentication file (generated by `user` subcommand)
  #[arg(short, long)]
  pub file: String,
}
