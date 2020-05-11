use ::std::process;
use seahorse::Context;

/// Returns a value of given String flag from context or exits the program
pub fn require_sflag<'a>(c: &'a Context, flag: &str) -> String {
    require_flag(flag, c.string_flag(flag))
}

/// Returns a value of given Integer flag from context or exits the program
#[allow(dead_code)]
pub fn require_iflag<'a>(c: &'a Context, flag: &str) -> isize {
    require_flag(flag, c.int_flag(flag))
}

/// Returns a value of given Float flag from context or exits the program
#[allow(dead_code)]
pub fn require_fflag<'a>(c: &'a Context, flag: &str) -> f64 {
    require_flag(flag, c.float_flag(flag))
}

/// Generic implementation. Check if passed flag option has value and if not,
/// exit the program.
fn require_flag<T>(flag: &str, flag_value: Option<T>) -> T {
    match flag_value {
        Some(val) => val,
        None => {
            eprintln!("Flag '{}' needs to be set!", flag);
            process::exit(1)
        }
    }
}
