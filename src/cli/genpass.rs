use crate::process::parse_length;
use crate::{generate_password, CmdExecutor};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    /// Length of the password
    #[arg(long, default_value_t = 12, value_parser = parse_length)]
    pub length: u8,
    /// Use lowercase letters
    #[arg(short, long, action = clap::ArgAction::Set, default_value_t = true)]
    pub lowercase: bool,
    /// Use uppercase letters
    #[arg(short, long, action = clap::ArgAction::Set, default_value_t = true)]
    pub uppercase: bool,
    /// Use numbers
    #[arg(short, long, action = clap::ArgAction::Set, default_value_t = true)]
    pub numbers: bool,
    /// Use special characters
    #[arg(short, long, action = clap::ArgAction::Set, default_value_t = true)]
    pub special: bool,
}

impl CmdExecutor for GenPassOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let password = generate_password(
            self.length,
            self.lowercase,
            self.uppercase,
            self.numbers,
            self.special,
        )?;
        println!("{}", password);
        let score = zxcvbn::zxcvbn(password.as_str(), &[])?.score();
        eprintln!("Password strength: {}", score);
        Ok(())
    }
}
