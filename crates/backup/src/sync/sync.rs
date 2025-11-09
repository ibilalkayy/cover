use std::fs::read_to_string;
use std::fs::{copy, metadata, remove_file};
use std::path::Path;
use std::time::UNIX_EPOCH;
use std::{fs::read_dir, path::PathBuf};

pub struct SyncData {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub changed_only: bool,
    pub delete: bool,
    pub dry_run: bool,
    pub verbose: bool,
}

impl SyncData {
    pub fn src_dest_dir_present(&self) -> bool {
        let src_not_empty = !self.source.to_string_lossy().trim().is_empty();
        let dest_not_empty = !self.destination.to_string_lossy().trim().is_empty();

        if !self.source.is_dir() && !self.destination.is_dir() {
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

impl SyncData {
    pub fn list_source_files(&self) -> Vec<PathBuf> {
        let mut files_list = Vec::new();

        if !self.source.is_dir() {
            eprintln!("Err: source is not a directory: {:?}", self.source);
            return Vec::new();
        }

        let paths = read_dir(&self.source).expect("Err: failed to read the directory");
        for entry in paths {
            match entry {
                Ok(entry_path) => {
                    let pathway = entry_path.path();
                    if pathway.is_file() {
                        files_list.push(pathway.to_path_buf());
                    }
                }
                Err(error) => eprintln!("Err: {}", error),
            }
        }
        return files_list;
    }

    pub fn list_destination_files(&self) -> Vec<PathBuf> {
        let mut files_list = Vec::new();

        if !self.destination.is_dir() {
            eprintln!(
                "Err: destination is not a directory: {:?}",
                self.destination
            );
            return Vec::new();
        }

        let paths = read_dir(&self.destination).expect("Err: failed to read the directory");
        for entry in paths {
            match entry {
                Ok(entry_path) => {
                    let pathway = entry_path.path();
                    if pathway.is_file() {
                        files_list.push(pathway.to_path_buf());
                    }
                }
                Err(error) => eprintln!("Err: {}", error),
            }
        }
        return files_list;
    }
}

impl SyncData {
    pub fn file_duration_since(&self, file_name: &PathBuf) -> Option<(f64, f64)> {
        let src_path = Path::new(&self.source).join(file_name);
        let dest_path = Path::new(&self.destination).join(file_name);

        if !src_path.exists() {
            return None;
        }

        let src_meta = metadata(&src_path)
            .ok()
            .expect("Err: failed to get the source metadata");
        let dest_meta = metadata(&dest_path).ok();

        let src_numeric = src_meta
            .modified()
            .ok()
            .expect("Err: failed to check the modification")
            .duration_since(UNIX_EPOCH)
            .ok()
            .expect("Err: failed to get the duration")
            .as_secs() as f64;

        let dest_numeric = if let Some(dest_meta) = dest_meta {
            dest_meta
                .modified()
                .ok()
                .expect("Err: failed to check the modification")
                .duration_since(UNIX_EPOCH)
                .ok()
                .expect("Err: failed to get the duration")
                .as_secs() as f64
        } else {
            0.0
        };

        Some((src_numeric, dest_numeric))
    }

    pub fn src_file_created(&self) -> bool {
        let src_entries = read_dir(&self.source).expect("Err: failed to read the source dir");
        for entry in src_entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let dest_path = Path::new(&self.destination).join(&file_name);
            if !dest_path.exists() {
                return true;
            }
        }
        false
    }

    pub fn src_file_modified(&self) -> (PathBuf, bool) {
        let src_entries = read_dir(&self.source).expect("Err: failed to read the source dir");
        let mut is_modified = false;
        let mut modified_file = PathBuf::new();
        let mut last_modify_numeric: Vec<f64> = Vec::new();

        for entry in src_entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if let Some((src_time, dest_time)) =
                self.file_duration_since(&PathBuf::from(&file_name))
            {
                if src_time > dest_time && dest_time != 0.0 {
                    last_modify_numeric.push(src_time);
                    if let Some(max_value) = last_modify_numeric
                        .iter()
                        .cloned()
                        .fold(None, |max, val| Some(max.map_or(val, |m: f64| m.max(val))))
                    {
                        if max_value == src_time {
                            modified_file = PathBuf::from(&file_name);
                            is_modified = true;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        (modified_file, is_modified)
    }

    pub fn dest_file_created(&self) -> bool {
        let dest_entries =
            read_dir(&self.destination).expect("Err: failed to read destination dir");
        for entry in dest_entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let src_path = Path::new(&self.source).join(&file_name);
            if !src_path.exists() {
                return true;
            }
        }
        false
    }

    pub fn dest_file_modified(&self) -> (PathBuf, bool) {
        let dest_entries = read_dir(&self.destination).expect("Err: failed to read dest dir");
        let mut is_modified = false;
        let mut modified_file = PathBuf::new();
        let mut last_modify_numeric: Vec<f64> = Vec::new();

        for entry in dest_entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if let Some((src_time, dest_time)) =
                self.file_duration_since(&PathBuf::from(&file_name))
            {
                if dest_time > src_time && src_time != 0.0 {
                    last_modify_numeric.push(dest_time);
                    if let Some(max_value) = last_modify_numeric
                        .iter()
                        .cloned()
                        .fold(None, |max, val| Some(max.map_or(val, |m: f64| m.max(val))))
                    {
                        if max_value == dest_time {
                            modified_file = PathBuf::from(&file_name);
                            is_modified = true;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        (modified_file, is_modified)
    }
}

impl SyncData {
    pub fn copy_src_to_destination(&self) {
        let src_files = self.list_source_files();
        for src_file in src_files {
            let file_name = src_file
                .file_name()
                .expect("Err: failed to get the filenames");
            let dest_path = Path::new(&self.destination).join(file_name);
            if !dest_path.exists() {
                copy(&src_file, &dest_path).expect("Err: failed to copy the files");
            }
        }
    }

    pub fn remove_dest_file(&self) {
        let dest_files = self.list_destination_files();
        for dest_file in dest_files {
            let file_name = dest_file
                .file_name()
                .expect("Err: failed to get the file name");
            let dest_path = Path::new(&self.destination).join(file_name);
            remove_file(&dest_path).expect("Err: failed to remove a file");
        }
        self.copy_src_to_destination();
    }

    pub fn update_dest_file(&self, file_name: PathBuf) {
        let dest_files = self.list_destination_files();
        for dest_file in dest_files {
            let found_dest_file = dest_file
                .file_name()
                .expect("Err: failed to get the file name");
            let modified_src_file = file_name
                .file_name()
                .expect("Err: failed to get the file name");

            if found_dest_file == modified_src_file {
                remove_file(dest_file).expect("Err: failed to remove the file");
                break;
            }
        }

        let src_files = self.list_source_files();
        for src_file in src_files {
            let found_src_file = src_file
                .file_name()
                .expect("Err: failed to get the file name");
            let modified_src_file = file_name
                .file_name()
                .expect("Err: failed to get the file name");

            if found_src_file == modified_src_file {
                let dest_path = Path::new(&self.destination).join(&file_name);
                copy(&src_file, &dest_path).expect("Err: failed to copy the file");
            }
        }
    }

    pub fn overwrite_with_src(&self, file_name: PathBuf) {
        let src_files = self.list_source_files();
        let dest_files = self.list_destination_files();
        let mut found = false;

        for src_file in src_files {
            for dest_file in &dest_files {
                let src_file_content =
                    read_to_string(&src_file).expect("Err: failed to read the file");
                let dest_file_content =
                    read_to_string(&dest_file).expect("Err: failed to read the file");

                let given_dest_file = file_name
                    .file_name()
                    .expect("Err: failed to get the file name");

                let found_dest_file = dest_file
                    .file_name()
                    .expect("Err: failed to get the file name");

                if given_dest_file == found_dest_file && src_file_content != dest_file_content {
                    remove_file(&dest_file).expect("Err: failed to remove the file");

                    let src_file = Path::new(&self.source).join(file_name.clone());
                    copy(&src_file, &dest_file).expect("Err: failed to copy the file");

                    println!("Msg: destination file(s) modification not allowed");
                    found = true;
                }
            }
        }
        if !found {
            eprintln!("Err: no changes detected");
        }
    }
}

impl SyncData {
    pub fn remove_all_dest_files(&self) {
        let dest_files = self.list_destination_files();
        for dest_file in dest_files {
            let file_name = dest_file
                .file_name()
                .expect("Err: failed to get the file name");
            let dest_path = Path::new(&self.destination).join(file_name);
            remove_file(&dest_path).expect("Err: failed to remove a file");
        }
    }
}

impl SyncData {
    fn log_for_source_creation(&self) {
        let list_src_files = self.list_source_files();
        let list_dest_files = self.list_destination_files();
        println!("<---------LOGS OF ACTION--------->");

        println!(
            "Checking Directory: {}",
            self.source.to_string_lossy().to_string()
        );

        print!("List of source files: ");
        list_src_files
            .iter()
            .filter_map(|f| f.file_name()?.to_str())
            .for_each(|name| print!("{}\t", name));
        println!();

        print!("List of destination files: ");
        if list_dest_files.is_empty() {
            println!("Empty");
            println!("Status: Not matched");
            print!("Copied: ");
            self.copy_src_to_destination();
            list_src_files
                .iter()
                .filter_map(|f| f.file_name()?.to_str())
                .for_each(|name| print!("{} ", name));
            println!("-> {}", self.destination.to_string_lossy().to_string());
        } else {
            println!("Not empty");
            println!("Status: Not matched");
            print!("Copied: ");
            self.copy_src_to_destination();
            list_src_files
                .iter()
                .filter_map(|f| f.file_name()?.to_str())
                .for_each(|name| print!("{} ", name));
            println!("-> {}", self.destination.to_string_lossy().to_string());
        }
    }

    fn log_for_source_modification(&self, filename: PathBuf) {
        println!("<---------LOGS OF ACTION--------->");

        print!("Checking Destination Directory: ");

        let list_dest_files = self.list_destination_files();
        list_dest_files
            .iter()
            .filter_map(|f| f.file_name()?.to_str())
            .for_each(|name| print!("{}\t", name));
        println!();

        println!(
            "Modified source file: {}",
            filename.to_string_lossy().to_string()
        );

        for dest_file in list_dest_files {
            if dest_file.file_name() == filename.file_name() {
                println!("Modified source file is equal to the destination file");
            }
        }
        self.update_dest_file(filename);
        println!("Updated the source file with the destination file");
    }

    fn log_for_dest_creation(&self) {
        println!("<---------LOGS OF ACTION--------->");

        print!("Checking source directory: ");

        let list_src_files = self.list_source_files();
        let list_dest_files = self.list_destination_files();

        list_src_files
            .iter()
            .filter_map(|f| f.file_name()?.to_str())
            .for_each(|name| print!("{}\t", name));
        println!();

        print!("Checking destination directory: ");
        list_dest_files
            .iter()
            .filter_map(|f| f.file_name()?.to_str())
            .for_each(|name| print!("{}\t", name));
        println!();

        for src_files in list_src_files {
            for dest_files in list_dest_files.clone() {
                if src_files.file_name().unwrap() == dest_files.file_name().unwrap() {
                    println!(
                        "Msg: {} file will stay as it is",
                        src_files.file_name().unwrap().to_string_lossy().to_string()
                    );
                }

                if src_files.file_name().unwrap() != dest_files.file_name().unwrap() {
                    println!("Status: Not matched");
                    println!(
                        "Action: {} file will be removed",
                        dest_files
                            .file_name()
                            .unwrap()
                            .to_string_lossy()
                            .to_string()
                    );
                }
            }
        }

        self.remove_dest_file();
        println!("Msg: Source and destination files are matched");
    }

    fn log_for_dest_modification(&self) {
        println!("<---------LOGS OF ACTION--------->");
        print!("Checking source directory: ");

        let list_src_files = self.list_source_files();
        let list_dest_files = self.list_destination_files();

        list_src_files
            .iter()
            .filter_map(|f| f.file_name()?.to_str())
            .for_each(|name| print!("{}\t", name));
        println!();

        print!("Checking destination directory: ");
        list_dest_files
            .iter()
            .filter_map(|f| f.file_name()?.to_str())
            .for_each(|name| print!("{}\t", name));
        println!();

        let (modified_dest_file, _) = self.dest_file_modified();
        for src_file in list_src_files {
            for dest_file in &list_dest_files {
                let src_file_content =
                    read_to_string(&src_file).expect("Err: failed to read the file");
                let dest_file_content =
                    read_to_string(&dest_file).expect("Err: failed to read the file");

                print!("Source file content: \n{}", src_file_content);
                print!("Destination file content: \n{}", dest_file_content);

                if src_file_content != dest_file_content {
                    println!("Status: Content not matched");
                    println!("Success: Overwrite complete");
                    self.overwrite_with_src(modified_dest_file.clone());
                } else {
                    println!("Msg: No overwrite needed. All files matched");
                }
            }
        }
    }
}

impl SyncData {
    pub fn sync_output(&mut self) {
        if !self.single_command_selected() {
            eprintln!("Err: entering multiple flags are not allowed");
            return;
        }

        match (self.changed_only, self.delete, self.dry_run, self.verbose) {
            (true, _, _, _) => {
                if !self.src_dest_dir_present() {
                    eprintln!("Err: missing source or destination directories");
                    return;
                }

                let src_created = self.src_file_created();
                let (modified_src_file, src_modified) = self.src_file_modified();

                let dest_created = self.dest_file_created();
                let (modified_dest_file, dest_modified) = self.dest_file_modified();

                match (src_created, src_modified, dest_created, dest_modified) {
                    (true, _, _, _) => {
                        self.copy_src_to_destination();
                        println!("Success: source file(s) successfully copied");
                    }
                    (_, true, _, _) => {
                        self.update_dest_file(modified_src_file);
                        println!("Success: destination file(s) successfully updated");
                    }
                    (_, _, true, _) => {
                        self.remove_dest_file();
                        println!("Msg: destination file(s) creation not allowed");
                    }
                    (_, _, _, true) => {
                        self.overwrite_with_src(modified_dest_file);
                    }
                    _ => eprintln!("Err: no changes detected"),
                }
            }
            (_, true, _, _) => {
                self.remove_all_dest_files();
                println!("Success: Destination files are successfully deleted");
            }
            (_, _, true, _) => {
                if !self.src_dest_dir_present() {
                    eprintln!("[DRY RUN]: Would create the source and destination directories");
                    return;
                }

                let src_created = self.src_file_created();
                let (_, src_modified) = self.src_file_modified();

                let dest_created = self.dest_file_created();
                let (_, dest_modified) = self.dest_file_modified();

                match (src_created, src_modified, dest_created, dest_modified) {
                    (true, _, _, _) => {
                        println!("[DRY RUN]: Would copy the source file(s) to destination")
                    }
                    (_, true, _, _) => {
                        println!("[DRY RUN]: Would update the destination file(s)");
                    }
                    (_, _, true, _) => {
                        println!(
                            "[DRY RUN]: Would prevent the creation of the destination file(s)"
                        );
                    }
                    (_, _, _, true) => {
                        println!(
                            "[DRY RUN]: Would overwrite the destination file(s) with source file(s)"
                        );
                    }
                    _ => eprintln!("[DRY RUN]: Would give the error because no changes detected"),
                }
            }
            (_, _, _, true) => {
                if !self.src_dest_dir_present() {
                    eprintln!("Err: missing source or destination directories");
                    return;
                }

                let src_created = self.src_file_created();
                let (modified_src_file, src_modified) = self.src_file_modified();

                let dest_created = self.dest_file_created();
                let (_, dest_modified) = self.dest_file_modified();

                match (src_created, src_modified, dest_created, dest_modified) {
                    (true, _, _, _) => {
                        self.log_for_source_creation();
                    }
                    (_, true, _, _) => {
                        self.log_for_source_modification(modified_src_file);
                    }
                    (_, _, true, _) => {
                        self.log_for_dest_creation();
                    }
                    (_, _, _, true) => {
                        self.log_for_dest_modification();
                    }
                    _ => eprintln!("Err: no changes detected"),
                }
            }
            _ => eprintln!("Err: other commands are not allowed"),
        }
    }

    pub fn sync_options(&mut self) {
        self.sync_output();
    }
}
