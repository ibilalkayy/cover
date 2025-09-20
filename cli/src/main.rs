pub mod commands;
pub mod flags;

use crate::commands::commands::{Command, Cover};
use archive::ArchiveData;
use clap::Parser;
use clean::CleanData;
use list::ListData;
use restore::RestoreData;
use schedule::ScheduleData;
use sync::SyncData;

fn handle_commands() {
    let cover = Cover::parse();
    match cover.command {
        Command::Sync(s) => {
            let sync_data = SyncData {
                source: s.source,
                destination: s.destination,
                incremental: s.incremental,
                delete: s.delete,
                dry_run: s.dry_run,
                verbose: s.verbose,
                hash: s.hash,
            };
            sync_data.sync_options();
        }
        Command::Archive(a) => {
            let archive_data = ArchiveData {
                zip: a.zip,
                tar: a.tar,
                encrypt: a.encrypt,
                timestamp: a.timestamp,
            };
            archive_data.archive_options();
        }
        Command::Restore(r) => {
            let restore_data = RestoreData {
                overwrite: r.overwrite,
                to: r.to,
                select: r.select,
            };
            restore_data.restore_options();
        }
        Command::Schedule(s) => {
            let schedule_data = ScheduleData {
                daily: s.daily,
                weekly: s.weekly,
                interval: s.interval,
                command: s.command,
            };
            schedule_data.schedule_options();
        }
        Command::List(l) => {
            let list_data = ListData {
                archives: l.archives,
                schedules: l.schedules,
                details: l.details,
            };
            list_data.list_options();
        }
        Command::Clean(c) => {
            let clean_data = CleanData {
                keep_last: c.keep_last,
                older_than: c.older_than,
                dry_run: c.dry_run,
            };
            clean_data.clean_options();
        }
    }
}

fn main() {
    handle_commands();
}
