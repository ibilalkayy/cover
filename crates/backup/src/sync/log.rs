use super::sync::SyncData;
use std::{fs::read_to_string, path::PathBuf};

impl SyncData {
    pub fn log_for_source_creation(&self) {
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
            println!("[STATUS]: Not matched");
            print!("Copied: ");
            self.copy_src_to_destination();
            list_src_files
                .iter()
                .filter_map(|f| f.file_name()?.to_str())
                .for_each(|name| print!("{} ", name));
            println!("-> {}", self.destination.to_string_lossy().to_string());
        } else {
            println!("Not empty");
            println!("[STATUS]: Not matched");
            print!("Copied: ");
            self.copy_src_to_destination();
            list_src_files
                .iter()
                .filter_map(|f| f.file_name()?.to_str())
                .for_each(|name| print!("{} ", name));
            println!("-> {}", self.destination.to_string_lossy().to_string());
        }
    }

    pub fn log_for_source_modification(&self, filename: PathBuf) {
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

    pub fn log_for_dest_creation(&self) {
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
                        "[MSG]: {} file will stay as it is",
                        src_files.file_name().unwrap().to_string_lossy().to_string()
                    );
                }

                if src_files.file_name().unwrap() != dest_files.file_name().unwrap() {
                    println!("[STATUS]: not matched");
                    println!(
                        "[ACTION]: {} file will be removed",
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
        println!("[MSG]: Source and destination files are matched");
    }

    pub fn log_for_dest_modification(&self) {
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
                    read_to_string(&src_file).expect("[ERROR]: failed to read the file");
                let dest_file_content =
                    read_to_string(&dest_file).expect("[ERROR]: failed to read the file");

                print!("Source file content: \n{}", src_file_content);
                print!("Destination file content: \n{}", dest_file_content);

                if src_file_content != dest_file_content {
                    println!("[STATUS]: content not matched");
                    let _ = self.overwrite_with_src(modified_dest_file.clone());
                    println!("[SUCCESS]: overwrite complete");
                } else {
                    println!("[MSG]: no overwrite needed because all files matched");
                }
            }
        }
    }
}
