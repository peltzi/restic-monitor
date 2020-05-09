use seahorse::{Command, Context};

pub fn get_restic_latest_snapshots(c: &Context) {
    let message = "Lollol";

    println!("{:?}", message);
}

pub fn get_restic_latest_snapshots_cmd() -> Command {
    Command::new()
        .name("get_latest_snapshots")
        .usage("blalba")
        .action(get_restic_latest_snapshots)
}
