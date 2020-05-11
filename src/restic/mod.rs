use std::error::Error;
use std::process::Command;

pub mod snapshots;

pub const RESTIC_DEFAULT_GROUP_BY: &str = "host";
pub const RESTIC_DEFAULT_BINARY_LOCATION: &str = "/usr/bin/restic";

pub fn run(binpath: Option<&str>, args: &[&str]) -> Result<Vec<u8>, Box<dyn Error>> {
    let bin = match binpath {
        Some(bin) => bin,
        None => RESTIC_DEFAULT_BINARY_LOCATION,
    };
    let output = Command::new(&bin).args(args).output()?;

    if !output.status.success() {
        let output_stderr = String::from_utf8(output.stderr)?;
        eprintln!("Stderr: {:?}", output_stderr);
        Err("Bad return code from Restic. See stderr above.")?;
    }

    Ok(output.stdout)
}
