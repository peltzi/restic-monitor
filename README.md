# Restic-monitor - a small Restic CLI wrapper

[Restic][1] is a backup program that does easy repository based backups. It is 
a single binary solution and repositories can exist in multiple.

Restic-monitor calls restic binary and does additional analytics from the 
output.

Currently restic-monitor only supports one function: listing snapshots from 
restic repository and returning status based on whether the snapshots are newer 
than X hours (default: 24).

[1]: https://restic.net/

## Usage

```
    $ restic-monitor ensure-snapshots-newer-than --repo ~/restic_test_repo --newer-than 24
    Group latest snapshot: Snapshot { time: 2020-05-10T10:35:02.237388366+03:00, hostname: "peltzi-t580", id: "0a49c667e38e51bc9b41a2697fca14b8f8d095789a0930ff5952241e82d0dd58", tags: ["test"], paths: ["/home/peltzi/kazam_32vi1cv7.movie"] }
    Old time: 2020-08-31 14:54:23.557959759 +03:00
    Local time with snapshot timezone: "2020-09-01T14:54:23.557959759+03:00"
    Snapshot older than limit
    Group latest snapshot: Snapshot { time: 2020-05-09T19:37:25.693602256+03:00, hostname: "esamatti", id: "df11961e285b886b5920a5bf54874718984e054bd4927b338f8a56c1b06d1f66", tags: ["test"], paths: ["/home/peltzi/kazam_32vi1cv7.movie"] }
    Old time: 2020-08-31 14:54:23.558128346 +03:00
    Local time with snapshot timezone: "2020-09-01T14:54:23.558128346+03:00"
    Snapshot older than limit
    ERROR: Groups have snapshots older than 24 hours!
    $ echo $?
    1
```
