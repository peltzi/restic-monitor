use seahorse::App;
use std::env;

mod cli;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new()
        .name(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .command(cli::get_restic_latest_snapshots_cmd());

    app.run(args);
}
