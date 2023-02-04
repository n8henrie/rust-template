use std::process::Command;
use std::{env, error, result};

type Error = Box<dyn error::Error + Send + Sync>;
type Result<T> = result::Result<T, Error>;

fn remac_macos() -> Result<String> {
    let output = Command::new("/sbin/ifconfig").output()?;
    let stdout = std::str::from_utf8(&output.stdout)?;
    println!("{}", stdout);
    Ok(stdout.to_owned())
}

fn main() -> Result<()> {
    if env::consts::OS == "macos" {
        remac_macos()?;
    };
    Ok(())
}
