use std::{fs::remove_file, path::PathBuf};

use crate::utils::take_strip;

pub fn delete_flag(
    source_files: &Vec<(PathBuf, i64)>,
    destination_files: &Vec<(PathBuf, i64)>,
    source: &PathBuf,
    destination: &PathBuf,
) {
    let mut deleted_any = false;

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
            remove_file(dest_path).expect("Err: failed to remove the file(s)");
            deleted_any = false;
        }
    }

    if !deleted_any {
        println!("Non-matching destination files are deleted successfully");
    }
}
