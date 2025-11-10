use super::sync::SyncData;
use std::{
    fs::remove_file,
    fs::{copy, read_to_string},
    path::{Path, PathBuf},
};

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
