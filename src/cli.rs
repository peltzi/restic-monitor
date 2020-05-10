use restic_monitor::restic::snapshots;
use restic_monitor::restic::*;
use seahorse::{Command, Context, Flag, FlagType};

fn handle_ensure_snapshots_age(c: &Context) {
    let repo = match c.string_flag("repo") {
        Some(s) => s,
        None => {
            eprintln!("You must give Restic repo string!");
            std::process::exit(1)
        }
    };

    let group_by = match c.string_flag("group-by") {
        Some(s) => s,
        None => RESTIC_DEFAULT_GROUP_BY.to_string(),
    };

    let older_hours: i64 = match c.int_flag("newer-than") {
        Some(s) => s as i64,
        None => 24,
    };

    println!("Grouping by: {}", group_by);

    let mut snapshots_by_group = match snapshots::get_by_group(&repo, &group_by) {
        Ok(snapshots_by_group) => snapshots_by_group,
        Err(error) => {
            eprintln!("Error while listing snapshots: {:?}", error);
            std::process::exit(1)
        }
    };

    let mut errors = false;

    for group in &mut snapshots_by_group {
        match group.latest_snapshot() {
            Some(snap) => println!("Group latest snapshot: {:?}", snap),
            None => (),
        }
        if group.latest_snapshot_older_than(&older_hours) {
            errors = true;
        };
    }

    if errors {
        println!(
            "ERROR: Groups have snapshots older than {} hours!",
            older_hours
        );
        std::process::exit(1)
    } else {
        println!(
            "OK: All groups have snaphots newer than {} hours!",
            older_hours
        );
        std::process::exit(0)
    }
}

pub fn ensure_snapshots_age_cmd() -> Command {
    Command::new()
        .name("ensure-snapshots-newer-than")
        .action(handle_ensure_snapshots_age)
        .flag(Flag::new("repo", "--repo [string]", FlagType::String).alias("r"))
        .flag(
            Flag::new(
                "group-by",
                "--group-by [field,field] (default: host)",
                FlagType::String,
            )
            .alias("g"),
        )
        .flag(Flag::new(
            "newer-than",
            "--newer-than [hours] (default: 24)",
            FlagType::Int,
        ))
}
