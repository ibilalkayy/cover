use super::sync::SyncData;

impl SyncData {
    pub fn src_dest_dir_present(&self) -> bool {
        let src_not_empty = !self.source.to_string_lossy().trim().is_empty();
        let dest_not_empty = !self.destination.to_string_lossy().trim().is_empty();

        if !self.source.is_dir() || !self.destination.is_dir() {
            return false;
        }

        if src_not_empty && dest_not_empty {
            return true;
        } else {
            return false;
        }
    }

    pub fn single_command_selected(&mut self) -> bool {
        let source_contains = !self.source.to_string_lossy().trim().is_empty();
        let destination_contains = !self.destination.to_string_lossy().trim().is_empty();

        if !self.source.exists() && !self.destination.exists() {
            return true;
        }

        if !self.source.is_dir() && !self.destination.is_dir() {
            return false;
        }

        if !(source_contains && destination_contains) {
            return false;
        }

        let mut count = 0;
        if self.changed_only {
            count += 1;
        }
        if self.delete {
            count += 1;
        }
        if self.dry_run {
            count += 1;
        }
        if self.verbose {
            count += 1;
        }

        count == 1
    }
}
