use seahorse::App;
use std::env;

mod cli;
mod restic;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new()
        .name(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .command(cli::snapshots::ensure_age_cmd());

    app.run(args);
}
