use super::sync::SyncData;
use std::{collections::BTreeSet, path::PathBuf};

pub fn filter_src_dir(src_dirs: &Vec<PathBuf>, source: &PathBuf) -> Vec<PathBuf> {
    let mut list_src_dirs = Vec::new();

    for dirs in src_dirs {
        if dirs == source {
            continue;
        }

        let subdir = dirs
            .strip_prefix(source)
            .expect("[ERROR]: failed to get the directory");
        list_src_dirs.push(PathBuf::from(subdir));
    }
    list_src_dirs
}

pub fn filter_dest_dir(dest_dirs: &Vec<PathBuf>, destination: &PathBuf) -> Vec<PathBuf> {
    let mut list_dest_dirs = Vec::new();

    for dirs in dest_dirs {
        if dirs == destination {
            continue;
        }

        let subdir = dirs
            .strip_prefix(destination)
            .expect("[ERROR]: failed to get the directory");
        list_dest_dirs.push(PathBuf::from(subdir));
    }
    list_dest_dirs
}

pub fn filter_src_file(src_files: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut list_src_files = Vec::new();

    for file in src_files {
        let sub_file = file
            .file_name()
            .expect("[ERROR]: failed to get the filename");

        list_src_files.push(PathBuf::from(sub_file));
    }
    list_src_files
}

pub fn filter_dest_file(dest_files: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut list_dest_files = Vec::new();

    for file in dest_files {
        let sub_file = file
            .file_name()
            .expect("[ERROR]: failed to get the filename");

        list_dest_files.push(PathBuf::from(sub_file));
    }
    list_dest_files
}

fn print_format(files: &Vec<PathBuf>) {
    if files.len() == 0 {
        println!("Empty");
    } else if files.len() == 1 {
        println!("{}", files[0].display());
    } else {
        println!("{:?}", files);
    }
}

fn format_path(path: Vec<&PathBuf>) -> String {
    if path.is_empty() {
        "0".to_string()
            .strip_prefix("")
            .expect("[ERROR]: failed to remove the comma")
            .to_string()
    } else {
        format!(
            "[{}]",
            path.iter()
                .map(|f| f.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

fn list_data(data: [Vec<PathBuf>; 4], from: [PathBuf; 2]) -> [Vec<PathBuf>; 4] {
    let list_src_dirs = filter_src_dir(&data[0], &from[0]);
    let list_dest_dirs = filter_dest_dir(&data[1], &from[1]);
    let list_src_files = filter_src_file(&data[2]);
    let list_dest_files = filter_dest_file(&data[3]);

    println!("\n[---------LOGS OF ACTION---------]");

    print!("[SOURCE DIRECTORIES]: ");
    print_format(&list_src_dirs);

    print!("[DESTINATION DIRECTORIES]: ");
    print_format(&list_dest_dirs);

    print!("[SOURCE FILES]: ");
    print_format(&list_src_files);

    print!("[DESTINATION FILES]: ");
    print_format(&list_dest_files);

    let result: [Vec<PathBuf>; 4] = [
        list_src_dirs,
        list_dest_dirs,
        list_src_files,
        list_dest_files,
    ];
    result
}

impl SyncData {
    pub fn src_creation_log(&self) {
        let src_dirs = self.list_src_dirs();
        let dest_dirs = self.list_dest_dirs();
        let src_files = self.list_src_files();
        let dest_files = self.list_dest_files();

        let data: [Vec<PathBuf>; 4] = [src_dirs, dest_dirs, src_files, dest_files];
        let from: [PathBuf; 2] = [self.source.clone(), self.destination.clone()];
        let list_result = list_data(data, from);

        let src_dir_set: BTreeSet<_> = list_result[0].iter().collect();
        let dest_dir_set: BTreeSet<_> = list_result[1].iter().collect();
        let src_file_set: BTreeSet<_> = list_result[2].iter().collect();
        let dest_file_set: BTreeSet<_> = list_result[3].iter().collect();
        let mut directory: Vec<&PathBuf> = Vec::new();
        let mut file: Vec<&PathBuf> = Vec::new();
        let mut done = false;

        for name in src_dir_set.union(&dest_dir_set) {
            match (src_dir_set.get(name), dest_dir_set.get(name)) {
                (Some(src), None) => {
                    directory.push(src);
                    done = true;
                }
                _ => {}
            }
        }

        for name in src_file_set.union(&dest_file_set) {
            match (src_file_set.get(name), dest_file_set.get(name)) {
                (Some(src), None) => {
                    file.push(src);
                    done = true;
                }
                _ => {}
            }
        }

        if done {
            println!(
                "[DIFFERENCE]: {} directories & {} files",
                format_path(directory.clone()),
                format_path(file.clone())
            );
            println!("[STATUS]: Not matched");
            self.copy_src_to_dest();
            println!(
                "[COPIED]: {} & {} -> {}",
                format_path(directory),
                format_path(file),
                &self.destination.display()
            );
        }
    }

    pub fn src_modification_log(&self, filenames: Vec<PathBuf>) {
        let src_dirs = self.list_src_dirs();
        let dest_dirs = self.list_dest_dirs();
        let src_files = self.list_src_files();
        let dest_files = self.list_dest_files();

        let data: [Vec<PathBuf>; 4] = [src_dirs, dest_dirs, src_files, dest_files];
        let from: [PathBuf; 2] = [self.source.clone(), self.destination.clone()];
        let _ = list_data(data, from);

        print!("[MODIFIED FILES]: ");
        print_format(&filenames);

        println!("[STATUS]: Not matched");

        self.update_dest_file(filenames.clone());

        print!("[UPDATED FILES]: ");
        print_format(&filenames);
    }

    pub fn dest_creation_log(&self) {
        let src_dirs = self.list_src_dirs();
        let dest_dirs = self.list_dest_dirs();
        let src_files = self.list_src_files();
        let dest_files = self.list_dest_files();

        let data: [Vec<PathBuf>; 4] = [src_dirs, dest_dirs, src_files, dest_files];
        let from: [PathBuf; 2] = [self.source.clone(), self.destination.clone()];
        let list_result = list_data(data, from);

        let src_dir_set: BTreeSet<_> = list_result[0].iter().collect();
        let dest_dir_set: BTreeSet<_> = list_result[1].iter().collect();
        let src_file_set: BTreeSet<_> = list_result[2].iter().collect();
        let dest_file_set: BTreeSet<_> = list_result[3].iter().collect();
        let mut directory: Vec<&PathBuf> = Vec::new();
        let mut file: Vec<&PathBuf> = Vec::new();
        let mut done = false;

        for name in src_dir_set.union(&dest_dir_set) {
            match (src_dir_set.get(name), dest_dir_set.get(name)) {
                (None, Some(dest)) => {
                    directory.push(dest);
                    done = true;
                }
                _ => {}
            }
        }

        for name in src_file_set.union(&dest_file_set) {
            match (src_file_set.get(name), dest_file_set.get(name)) {
                (None, Some(dest)) => {
                    file.push(dest);
                    done = true;
                }
                _ => {}
            }
        }

        if done {
            println!(
                "[DIFFERENCE]: {} directories & {} files",
                format_path(directory.clone()),
                format_path(file.clone())
            );
            println!("[STATUS]: Not matched");
            self.remove_dest_file();
            println!(
                "[REMOVED]: {} directories & {} files from {}",
                format_path(directory),
                format_path(file),
                &self.destination.display()
            );
        }
    }

    pub fn dest_modification_log(&self, filenames: Vec<PathBuf>) {
        let src_dirs = self.list_src_dirs();
        let dest_dirs = self.list_dest_dirs();
        let src_files = self.list_src_files();
        let dest_files = self.list_dest_files();

        let data: [Vec<PathBuf>; 4] = [src_dirs, dest_dirs, src_files, dest_files];
        let from: [PathBuf; 2] = [self.source.clone(), self.destination.clone()];
        let _ = list_data(data, from);

        print!("[MODIFIED FILES]: ");
        print_format(&filenames);

        println!("[STATUS]: Not matched");

        self.update_dest_file(filenames.clone());

        print!("[REMOVED FILE CONTENT]: ");
        print_format(&filenames);
    }
}
