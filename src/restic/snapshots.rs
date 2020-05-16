use chrono::{DateTime, Duration, FixedOffset, Local};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Snapshot {
    pub time: DateTime<FixedOffset>,
    pub hostname: String,
    pub id: String,
    pub tags: Vec<String>,
    pub paths: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Group {
    pub group_key: Value,
    pub snapshots: Vec<Snapshot>,
}

impl Group {
    pub fn sort_snapshots_by_datetime(&mut self) {
        self.snapshots.sort_unstable_by(|a, b| b.time.cmp(&a.time));
    }

    pub fn latest_snapshot(&mut self) -> Option<&Snapshot> {
        if self.snapshots.is_empty() {
            return None;
        }

        self.sort_snapshots_by_datetime();
        Some(&self.snapshots[0])
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

        if latest_snapshot.time < old_time {
            return true;
        } else {
            return false;
        }
    }
}

pub fn get_by_group(repo: &str, group_by: &str) -> Result<Vec<Group>, Box<dyn Error>> {
    let output_string = String::from_utf8(super::run(
        None,
        &["-r", repo, "snapshots", "--json", "--group-by", group_by],
    )?)?;
    let groups: Vec<Group> = serde_json::from_str(&output_string)?;

    Ok(groups)
}
