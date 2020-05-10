use chrono::{DateTime, Duration, FixedOffset, Local};
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;
use std::process::Command as SysCommand;

#[derive(Debug, Deserialize)]
pub struct Snapshot {
    pub time: DateTime<FixedOffset>,
    pub hostname: String,
    pub id: String,
    pub tags: Vec<String>,
    pub paths: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    pub group_key: Value,
    pub snapshots: Vec<Snapshot>,
}

impl Group {
    pub fn latest_snapshot(&mut self) -> Option<&Snapshot> {
        if self.snapshots.is_empty() {
            return None;
        }

        self.sort_snapshots_by_datetime();
        Some(&self.snapshots[0])
    }

    pub fn sort_snapshots_by_datetime(&mut self) {
        self.snapshots.sort_unstable_by(|a, b| b.time.cmp(&a.time));
    }

    pub fn latest_snapshot_older_than(&mut self, older_hours: &i64) -> bool {
        let latest_snapshot = match self.latest_snapshot() {
            Some(snap) => snap,
            None => return true,
        };

        let local_time = Local::now();
        let old_time = match local_time.checked_sub_signed(Duration::hours(*older_hours)) {
            Some(dt) => dt,
            None => return true,
        };
        println!("Old time: {}", old_time);
        println!(
            "Local time with snapshot timezone: {:?}",
            local_time
                .with_timezone(&latest_snapshot.time.timezone())
                .to_rfc3339()
        );

        if latest_snapshot.time < old_time {
            println!("Snapshot older than limit");
            return true;
        } else {
            return false;
        }
    }
}

pub fn get_by_group(repo: &str, group_by: &str) -> Result<Vec<Group>, Box<dyn Error>> {
    let output = SysCommand::new("/usr/bin/restic")
        .args(&["-r", repo, "snapshots", "--json", "--group-by", group_by])
        .output()?;

    if !output.status.success() {
        let output_stderr = String::from_utf8(output.stderr)?;
        eprintln!("Stderr: {:?}", output_stderr);
        Err("Bad return code from Restic. See stderr above.")?;
    }

    let output_string = String::from_utf8(output.stdout)?;
    let groups: Vec<Group> = serde_json::from_str(&output_string)?;

    Ok(groups)
}
