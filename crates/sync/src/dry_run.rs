use std::{path::PathBuf, process::exit};

use crate::utils::take_strip;

pub fn dry_run_flag(
    source_files: &Vec<(PathBuf, i64)>,
    destination_files: &Vec<(PathBuf, i64)>,
    source: &PathBuf,
    destination: &PathBuf,
) {
    let mut files_to_be_copied = Vec::new();
    let mut files_to_be_updated = Vec::new();
    let mut files_to_be_removed = Vec::new();

    for (src_path, src_modified) in source_files {
        let src_real_path = take_strip(src_path, source);

        let mut found = false;
        let mut needs_update = false;

        for (dest_path, dest_modified) in destination_files {
            let dest_real_path = take_strip(dest_path, destination);

            if src_real_path == dest_real_path {
                found = true;
                if src_modified > dest_modified {
                    needs_update = true;
                }
                break;
            }
        }

        if !found {
            files_to_be_copied.push(src_real_path.to_path_buf().into_os_string());
        } else if needs_update {
            files_to_be_updated.push(src_real_path.to_path_buf().into_os_string());
        }
    }

    for (dest_path, _) in destination_files {
        let dest_real_path = take_strip(dest_path, destination);

        let mut found = false;
        for (src_path, _) in source_files {
            let src_real_path = take_strip(src_path, source);
            if src_real_path == dest_real_path {
                found = true;
                break;
            }
        }

        if !found {
            files_to_be_removed.push(dest_real_path.to_owned());
        }
    }

    if !files_to_be_copied.is_empty() {
        println!(
            "Missing files {:?} will move to {:?} destination",
            files_to_be_copied, destination
        );
    }

    if !files_to_be_updated.is_empty() {
        println!(
            "Outdated files {:?} will be updated in {:?} destination",
            files_to_be_updated, destination
        );
    }

    if !files_to_be_removed.is_empty() {
        println!(
            "Extra files {:?} will be deleted from {:?} destination",
            files_to_be_removed, destination
        );
    }

    if source_files.is_empty() {
        exit(0)
    }

    if files_to_be_copied.is_empty()
        && files_to_be_updated.is_empty()
        && files_to_be_removed.is_empty()
    {
        println!("No changes needed. Files are up-to-date.");
    }
}
