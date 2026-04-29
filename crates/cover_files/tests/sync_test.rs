use blake2::{Blake2s256, Digest};
use cover_files::sync::sync::SyncData;
use file_hashing::{get_hash_file, get_hash_folder};
use itertools::{EitherOrBoth::Both, EitherOrBoth::Left, EitherOrBoth::Right, Itertools};
use std::{
    collections::HashSet,
    env,
    fs::{File, create_dir_all, read_dir, read_to_string, remove_dir_all, remove_file, write},
    panic,
    path::PathBuf,
    thread, time,
};
use walkdir::WalkDir;

#[test]
fn test_src_dir_exists() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(home_dir.clone()).join("tmpsrc_dir_exists");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("dir_exists");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let mut exists = false;
    let src_listing = sync.list_src_dirs();
    let relative_dir = sync.prefixed_listing(src_listing.clone(), sync.source.clone());

    if relative_dir.len() > 0 {
        exists = true;
    }

    let dir_exists = sync.src_dir_exists();
    assert_eq!(dir_exists, exists);
    assert_eq!(exists, true);

    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
}

#[test]
fn test_dest_dir_exists() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let dest_parent_dir = PathBuf::from(home_dir.clone()).join("tmpdest_dir_exists");

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let dest_sub_dir = dest_parent_dir.join("dir_exists");

    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");

    let sync = SyncData {
        source: PathBuf::new(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let mut exists = false;
    let dest_listing = sync.list_dest_dirs();
    let relative_dir = sync.prefixed_listing(dest_listing.clone(), sync.destination.clone());

    if relative_dir.len() > 0 {
        exists = true;
    }

    let dir_exists = sync.dest_dir_exists();
    assert_eq!(dir_exists, exists);
    assert_eq!(exists, true);

    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_src_file_exists() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(home_dir.clone()).join("tmpsrc_file_exists");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("dir_exists");
    let src_file = src_parent_dir.join("file_one_exists.txt");
    let src_sub_file = src_sub_dir.join("file_two_exists.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");

    File::create(&src_file).expect("[ERROR]: failed to create a source file");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let mut exists = false;
    let src_listing = sync.list_src_files();
    let relative_file = sync.prefixed_listing(src_listing.clone(), sync.source.clone());

    if relative_file.len() > 0 {
        exists = true;
    }

    let file_exists = sync.src_file_exists();
    assert_eq!(file_exists, exists);
    assert_eq!(exists, true);

    remove_file(&src_file).ok();
    remove_file(&src_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
}

#[test]
fn test_dest_file_exists() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let dest_parent_dir = PathBuf::from(home_dir.clone()).join("tmpdest_file_exists");

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let dest_sub_dir = dest_parent_dir.join("dir_exists");
    let dest_file = dest_parent_dir.join("file_one_exists.txt");
    let dest_sub_file = dest_sub_dir.join("file_two_exists.txt");

    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");

    File::create(&dest_file).expect("[ERROR]: failed to create a destination file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");

    let sync = SyncData {
        source: PathBuf::new(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let mut exists = false;
    let dest_listing = sync.list_dest_files();
    let relative_file = sync.prefixed_listing(dest_listing.clone(), sync.destination.clone());

    if relative_file.len() > 0 {
        exists = true;
    }

    let file_exists = sync.dest_file_exists();
    assert_eq!(file_exists, exists);
    assert_eq!(exists, true);

    remove_file(&dest_file).ok();
    remove_file(&dest_sub_file).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_src_dest_dirname_matched() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(home_dir.clone()).join("tmpsrc_dirname_matched");
    let dest_parent_dir = PathBuf::from(home_dir.clone()).join("tmpdest_dirname_matched");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_dir = src_parent_dir.join("dir_name");
    let dest_dir = dest_parent_dir.join("dir_name");

    let src_sub_dir = src_dir.join("sub_dir_name");
    let dest_sub_dir = dest_dir.join("sub_dir_name");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_listing = sync.list_src_dirs();
    let dest_listing = sync.list_dest_dirs();

    let src_relative_dir = sync.prefixed_listing(src_listing.clone(), sync.source.clone());
    let dest_relative_dir = sync.prefixed_listing(dest_listing.clone(), sync.destination.clone());

    let difference: HashSet<_> = src_relative_dir.difference(&dest_relative_dir).collect();
    assert_eq!(difference.is_empty(), true);

    let check_diff = sync.src_dest_dirname_matched();
    assert_eq!(check_diff, true);

    remove_dir_all(&src_dir).ok();
    remove_dir_all(&dest_dir).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_dest_src_dirname_matched() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(home_dir.clone()).join("tmpsrc_dirname_matched");
    let dest_parent_dir = PathBuf::from(home_dir.clone()).join("tmpdest_dirname_matched");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_dir = src_parent_dir.join("dir_name");
    let dest_dir = dest_parent_dir.join("dir_name");

    let src_sub_dir = src_dir.join("sub_dir_name");
    let dest_sub_dir = dest_dir.join("sub_dir_name");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_listing = sync.list_src_dirs();
    let dest_listing = sync.list_dest_dirs();

    let src_relative_dir = sync.prefixed_listing(src_listing.clone(), sync.source.clone());
    let dest_relative_dir = sync.prefixed_listing(dest_listing.clone(), sync.destination.clone());

    let difference: HashSet<_> = dest_relative_dir.difference(&src_relative_dir).collect();
    assert_eq!(difference.is_empty(), true);

    let check_diff = sync.dest_src_dirname_matched();
    assert_eq!(check_diff, true);

    remove_dir_all(&src_dir).ok();
    remove_dir_all(&dest_dir).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_src_dest_filename_matched() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(home_dir.clone()).join("tmpsrc_filename_matched");
    let dest_parent_dir = PathBuf::from(home_dir.clone()).join("tmpdest_filename_matched");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_file = src_parent_dir.join("file_name.txt");
    let dest_file = dest_parent_dir.join("file_name.txt");

    let src_sub_dir = src_parent_dir.join("sub_dir_name");
    let dest_sub_dir = dest_parent_dir.join("sub_dir_name");

    let src_sub_file = src_sub_dir.join("file_name.txt");
    let dest_sub_file = dest_sub_dir.join("file_name.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");

    File::create(&src_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_file).expect("[ERROR]: failed to create a destination file");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_listing = sync.list_src_files();
    let dest_listing = sync.list_dest_files();

    let src_relative_file = sync.prefixed_listing(src_listing.clone(), sync.source.clone());
    let dest_relative_file = sync.prefixed_listing(dest_listing.clone(), sync.destination.clone());

    let difference: HashSet<_> = src_relative_file.difference(&dest_relative_file).collect();
    assert_eq!(difference.is_empty(), true);

    let check_diff = sync.src_dest_filename_matched();
    assert_eq!(check_diff, true);

    remove_file(&src_file).ok();
    remove_file(&dest_file).ok();
    remove_file(&src_sub_file).ok();
    remove_file(&dest_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_dest_src_filename_matched() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(home_dir.clone()).join("tmpsrc_filename_matched");
    let dest_parent_dir = PathBuf::from(home_dir.clone()).join("tmpdest_filename_matched");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_file = src_parent_dir.join("file_name.txt");
    let dest_file = dest_parent_dir.join("file_name.txt");

    let src_sub_dir = src_parent_dir.join("sub_dir_name");
    let dest_sub_dir = dest_parent_dir.join("sub_dir_name");

    let src_sub_file = src_sub_dir.join("file_name.txt");
    let dest_sub_file = dest_sub_dir.join("file_name.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");

    File::create(&src_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_file).expect("[ERROR]: failed to create a destination file");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_listing = sync.list_src_files();
    let dest_listing = sync.list_dest_files();

    let src_relative_file = sync.prefixed_listing(src_listing.clone(), sync.source.clone());
    let dest_relative_file = sync.prefixed_listing(dest_listing.clone(), sync.destination.clone());

    let difference: HashSet<_> = dest_relative_file.difference(&src_relative_file).collect();
    assert_eq!(difference.is_empty(), true);

    let check_diff = sync.src_dest_filename_matched();
    assert_eq!(check_diff, true);

    remove_file(&src_file).ok();
    remove_file(&dest_file).ok();
    remove_file(&src_sub_file).ok();
    remove_file(&dest_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_hashes_matched() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(home_dir.clone()).join("tmpsrc_hashes_matched");
    let dest_parent_dir = PathBuf::from(home_dir.clone()).join("tmpdest_hashes_matched");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_file = src_parent_dir.join("file_hash.txt");
    let dest_file = dest_parent_dir.join("file_hash.txt");

    let src_sub_dir = src_parent_dir.join("sub_dir_hash");
    let dest_sub_dir = dest_parent_dir.join("sub_dir_hash");

    let src_sub_file = src_sub_dir.join("sub_file_hash.txt");
    let dest_sub_file = dest_sub_dir.join("sub_file_hash.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");

    File::create(&src_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_file).expect("[ERROR]: failed to create a destination file");

    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_list = sync.list_src_files();
    let dest_list = sync.list_dest_files();

    let src_dir_list = sync.list_src_dirs();
    let dest_dir_list = sync.list_dest_dirs();

    let src_file_listing = sync.loop_listing(src_list.clone(), sync.source.clone());
    let dest_file_listing = sync.loop_listing(dest_list.clone(), sync.destination.clone());
    let src_dir_listing = sync.loop_listing(src_dir_list, sync.source.clone());
    let dest_dir_listing = sync.loop_listing(dest_dir_list, sync.destination.clone());

    let src_file_hash = sync
        .file_hash(src_file_listing.clone())
        .expect("[ERROR]: failed to get the hash");

    let dest_file_hash = sync
        .file_hash(dest_file_listing.clone())
        .expect("[ERROR]: failed to get the hash");

    let src_dir_hash = sync
        .dir_hash(src_dir_listing.clone())
        .expect("[ERROR]: failed to get the hash");

    let dest_dir_hash = sync
        .dir_hash(dest_dir_listing.clone())
        .expect("[ERROR]: failed to get the hash");

    let file_difference: HashSet<_> = src_file_hash.difference(&dest_file_hash).collect();
    let dir_difference: HashSet<_> = src_dir_hash.difference(&dest_dir_hash).collect();

    assert_eq!(file_difference.is_empty(), true);
    assert_eq!(dir_difference.is_empty(), true);

    let check_diff = sync.hashes_matched();
    assert_eq!(check_diff, true);

    assert_eq!(file_difference.is_empty(), check_diff);
    assert_eq!(dir_difference.is_empty(), check_diff);

    remove_file(&src_file).ok();
    remove_file(&dest_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_prefixed_listing() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(home_dir.clone()).join("tmpsrc_prefixed_listing");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    let src_file1 = src_parent_dir.join("file_prefixed1.txt");
    let src_file2 = src_parent_dir.join("file_prefixed2.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    File::create(&src_file1).expect("[ERROR]: failed to create a source file");
    File::create(&src_file2).expect("[ERROR]: failed to create a source file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let list_files = sync.list_src_files();
    let mut prefixed_list = HashSet::new();
    for entry in list_files.clone() {
        if entry == sync.source {
            continue;
        }

        let relative_file = entry
            .strip_prefix(&sync.source)
            .expect("[ERROR]: failed to prefix the file")
            .to_path_buf();

        prefixed_list.insert(relative_file);
    }

    let prefix_listing = sync.prefixed_listing(list_files, sync.source.clone());
    assert_eq!(prefixed_list.len(), prefix_listing.len());

    remove_file(&src_file1).ok();
    remove_file(&src_file2).ok();
    remove_dir_all(&src_parent_dir).ok();
}

#[test]
fn test_loop_listing() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(home_dir.clone()).join("tmpsrc_loop_listing");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    let src_file1 = src_parent_dir.join("file_loop1.txt");
    let src_file2 = src_parent_dir.join("file_loop2.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    File::create(&src_file1).expect("[ERROR]: failed to create a source file");
    File::create(&src_file2).expect("[ERROR]: failed to create a source file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let list_files = sync.list_src_files();
    let mut loop_list = Vec::new();
    for entry in list_files.clone() {
        if entry == sync.source {
            continue;
        }

        loop_list.push(entry);
    }

    let loop_listing = sync.loop_listing(list_files, sync.source.clone());
    assert_eq!(loop_listing.len(), loop_list.len());

    for pair in loop_list.iter().zip_longest(loop_listing.iter()) {
        match pair {
            Both(l, r) => {
                assert_eq!(l, r);
            }
            Left(_) | Right(_) => assert!(false),
        }
    }

    remove_file(&src_file1).ok();
    remove_file(&src_file2).ok();
    remove_dir_all(&src_parent_dir).ok();
}

#[test]
fn test_src_dir_parent_exists() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_parent_presence");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_parent_presence");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    assert_eq!(!sync.source.exists(), false);
    assert_eq!(!sync.destination.exists(), false);

    assert_eq!(!sync.source.is_dir(), false);
    assert_eq!(!sync.destination.is_dir(), false);

    assert!(
        sync.src_dest_dir_parent_exists(),
        "[ERROR]: source or destination directory not exists"
    );

    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_single_command_selected() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");

    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_command");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_command");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");

    let mut sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let source_contains = !sync.source.to_string_lossy().trim().is_empty();
    let destination_contains = !sync.destination.to_string_lossy().trim().is_empty();

    assert_eq!(!sync.source.is_dir(), false);
    assert_eq!(!sync.destination.is_dir(), false);

    assert_eq!(source_contains, true);
    assert_eq!(destination_contains, true);

    assert!(
        sync.single_command_selected(),
        "Expected one command, but multiple are reported"
    );

    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_has_duplicates() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_duplicates");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("sub_duplicate_dir");
    let src_file = src_parent_dir.join("src_file.txt");
    let src_sub_file = src_sub_dir.join("sub_duplicate_file.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");

    File::create(&src_file).expect("[ERROR]: failed to create a source file");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let mut dir_hash: HashSet<PathBuf> = HashSet::new();
    let mut file_hash: HashSet<PathBuf> = HashSet::new();

    let src_dirs = sync.list_src_dirs();
    let src_files = sync.list_src_files();
    let dir_list = sync.prefixed_listing(src_dirs, sync.source.clone());
    let file_list = sync.prefixed_listing(src_files, sync.source.clone());

    for dir in dir_list {
        let mut results = Vec::new();
        let dirs_list = vec![dir.clone(), dir.clone()];
        for d in dirs_list {
            let is_duplicate = !dir_hash.insert(d);
            results.push(is_duplicate);
        }

        assert_eq!(results[0], false);
        assert_eq!(results[1], true);
    }

    for file in file_list {
        let mut results = Vec::new();
        let files_list = vec![file.clone(), file.clone()];
        for f in files_list {
            let is_duplicate = !file_hash.insert(f);
            results.push(is_duplicate);
        }

        assert_eq!(results[0], false);
        assert_eq!(results[1], true);
    }

    let check_duplicates = sync.has_duplicates();
    assert_eq!(check_duplicates, false);

    remove_file(&src_file).ok();
    remove_file(&src_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
}

#[test]
fn test_validate_path() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_validate");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_validate");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_file = src_parent_dir.join("same_file.txt");
    let dest_file = dest_parent_dir.join("same_file.txt");

    let src_sub_dir = src_parent_dir.join("sub_dir_validate");
    let dest_sub_dir = dest_parent_dir.join("sub_dir_validate");

    let src_sub_file = src_sub_dir.join("sub_file_validate.txt");
    let dest_sub_file = dest_sub_dir.join("sub_file_validate.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a source directory");

    File::create(&src_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_file).expect("[ERROR]: failed to create a destination file");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let parent_exists = sync.src_dest_dir_parent_exists();
    let src_exists = sync.src_dir_exists() && sync.src_file_exists();
    let dest_exists = sync.dest_dir_exists() && sync.dest_file_exists();
    let src_dest_dirname_matched = sync.src_dest_dirname_matched();
    let src_dest_filename_matched = sync.src_dest_filename_matched();
    let dest_src_dirname_matched = sync.dest_src_dirname_matched();
    let dest_src_filename_matched = sync.dest_src_filename_matched();
    let path_matched = sync.path_matched();

    assert_eq!(parent_exists, true);
    assert_eq!(src_exists, true);
    assert_eq!(dest_exists, true);
    assert_eq!(src_dest_dirname_matched, true);
    assert_eq!(dest_src_dirname_matched, true);
    assert_eq!(src_dest_filename_matched, true);
    assert_eq!(dest_src_filename_matched, true);
    assert_eq!(path_matched, true);

    remove_file(&src_file).ok();
    remove_file(&dest_file).ok();
    remove_file(&src_sub_file).ok();
    remove_file(&dest_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_dir_contains_files() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_contains_files");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    let src_sub_file = src_parent_dir.join("sub_file.txt");
    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let dir_list = sync.list_src_dirs();
    if dir_list.len() > 0 {
        for dir in &dir_list.clone() {
            let read_dir = read_dir(dir).expect("[ERROR]: failed to read the directory");

            for entry in read_dir {
                let entry = entry.expect("[ERROR]: failed to get the entry");
                assert_eq!(entry.path().is_file(), true);
            }
        }
    }

    let dir_contains_files = sync.dir_contains_files(dir_list);
    assert_eq!(dir_contains_files, true);

    remove_file(&src_sub_file).ok();
    remove_dir_all(&src_parent_dir).ok();
}

#[test]
fn test_file_hash() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_validate");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    let src_file = src_parent_dir.join("same_file.txt");
    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    File::create(&src_file).expect("[ERROR]: failed to create a source file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let files = sync.list_src_files();
    let mut hashes = HashSet::new();

    if files.len() > 0 {
        for file in files.clone() {
            let mut blake_hash = Blake2s256::new();
            let hash =
                get_hash_file(file, &mut blake_hash).expect("[ERROR]: failed to get the file hash");
            hashes.insert(hash);
        }
    }
    let src_hash = sync
        .file_hash(files.clone())
        .expect("[ERROR]: failed to get the hash");

    assert_eq!(src_hash, hashes);

    remove_file(&src_file).ok();
    remove_dir_all(&src_parent_dir).ok();
}

#[test]
fn test_dir_hash() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_dir_hash");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    let src_file = src_parent_dir.join("src_file.txt");
    let src_sub_dir = src_parent_dir.join("src_sub_dir");
    let src_sub_file = src_sub_dir.join("src_sub_file.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let mut hashes = HashSet::new();
    let dirs = sync.list_src_dirs();
    let list_dirs = sync.loop_listing(dirs.clone(), sync.source.clone());
    let dir_contains_files = sync.dir_contains_files(list_dirs.clone());

    if dir_contains_files {
        for dir in list_dirs.clone() {
            let mut blake_hash = Blake2s256::new();
            let hash = get_hash_folder(&dir, &mut blake_hash, 12, |_| {})
                .expect("[ERROR]: failed to get the directory hash");
            hashes.insert(hash);
        }
    } else {
        panic!("[ERROR]: failed to find the files in a directory");
    }

    let src_hash = sync
        .dir_hash(list_dirs.clone())
        .expect("[ERROR]: failed to get the hash");

    assert_eq!(src_hash, hashes);

    remove_file(&src_file).ok();
    remove_file(&src_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
}

#[test]
fn test_list_src_dirs() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(home_dir).join("tmp_list_src_dirs");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }
    let src_sub_dir = src_parent_dir.join("test_dir");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let mut src_dirs_list = Vec::new();
    for entry in WalkDir::new(&sync.source) {
        let entry_path = entry
            .as_ref()
            .expect("[ERROR]: failed to get the path")
            .path()
            .to_path_buf();

        assert_eq!(entry_path.is_dir(), true);
        src_dirs_list.push(entry_path);
    }

    let dirs_list = sync.list_src_dirs();
    assert_eq!(src_dirs_list, dirs_list);

    let mut searched_dir: Vec<PathBuf> = Vec::new();
    for dir in &dirs_list {
        let dirname = dir
            .iter()
            .last()
            .expect("[ERROR]: failed to get the last name")
            .to_string_lossy()
            .to_string();

        searched_dir.push(PathBuf::from(dirname));
    }

    assert!(searched_dir.len() != 0);

    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
}

#[test]
fn test_list_src_files() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(home_dir).join("tmp_list_src_files");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let src_sub_file = src_sub_dir.join("test_src_file.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let mut src_files_list = Vec::new();
    for entry in WalkDir::new(&sync.source) {
        let entry = entry.expect("[ERROR]: failed to get the path");
        let entry_path = entry.path().to_path_buf();

        if entry_path.is_file() {
            assert_eq!(entry_path.is_file(), true);
            src_files_list.push(entry_path);
        }
    }

    let mut searched_file: Vec<PathBuf> = Vec::new();
    let files_list = sync.list_src_files();
    for file in files_list {
        let filename = file
            .file_name()
            .and_then(|f| f.to_str())
            .expect("[ERROR]: failed to get the filename");

        searched_file.push(PathBuf::from(filename));
    }

    assert!(searched_file.len() != 0);

    remove_file(&src_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
}

#[test]
fn test_list_dest_dirs() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let dest_parent_dir = PathBuf::from(home_dir).join("tmp_list_dest_dirs");

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }
    let dest_sub_dir = dest_parent_dir.join("test_dir");

    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");

    let sync = SyncData {
        source: PathBuf::new(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let mut dest_dirs_list = Vec::new();
    for entry in WalkDir::new(&sync.destination) {
        let entry_path = entry
            .as_ref()
            .expect("[ERROR]: failed to get the path")
            .path()
            .to_path_buf();

        assert_eq!(entry_path.is_dir(), true);
        dest_dirs_list.push(entry_path);
    }

    let dirs_list = sync.list_dest_dirs();
    assert_eq!(dest_dirs_list, dirs_list);

    let mut searched_dir: Vec<PathBuf> = Vec::new();
    for dir in &dirs_list {
        let dirname = dir
            .iter()
            .last()
            .expect("[ERROR]: failed to get the last name")
            .to_string_lossy()
            .to_string();

        searched_dir.push(PathBuf::from(dirname));
    }

    assert!(searched_dir.len() != 0);

    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_list_dest_files() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let dest_parent_dir = PathBuf::from(home_dir).join("tmp_list_dest_files");

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let dest_file = dest_parent_dir.join("test_dest_file.txt");
    let dest_sub_dir = dest_parent_dir.join("test_dir");
    let dest_sub_file = dest_sub_dir.join("test_dest_file.txt");

    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&dest_file).expect("[ERROR]: failed to create a destination file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");

    let sync = SyncData {
        source: PathBuf::new(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let mut dest_files_list = Vec::new();
    for entry in WalkDir::new(&sync.destination) {
        let entry = entry.expect("[ERROR]: failed to get the path");
        let entry_path = entry.path().to_path_buf();

        if entry_path.is_file() {
            assert_eq!(entry_path.is_file(), true);
            dest_files_list.push(entry_path);
        }
    }

    let mut searched_file: Vec<PathBuf> = Vec::new();
    let files_list = sync.list_dest_files();
    for file in files_list {
        let filename = file
            .file_name()
            .and_then(|f| f.to_str())
            .expect("[ERROR]: failed to get the filename");

        searched_file.push(PathBuf::from(filename));
    }

    assert!(searched_file.len() != 0);

    remove_file(&dest_file).ok();
    remove_file(&dest_sub_file).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_do_update_same_paths_different_content() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(home_dir.clone()).join("tmpsrc_update");
    let dest_parent_dir = PathBuf::from(home_dir).join("tmpdest_update");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_file = src_parent_dir.join("file_hash.txt");
    let dest_file = dest_parent_dir.join("file_hash.txt");

    let src_sub_dir = src_parent_dir.join("same_name_sub_dir");
    let dest_sub_dir = dest_parent_dir.join("same_name_sub_dir");

    let src_sub_file = src_sub_dir.join("file_hash.txt");
    let dest_sub_file = dest_sub_dir.join("file_hash.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");

    File::create(&src_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_file).expect("[ERROR]: failed to create a destination file");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");

    if sync.has_duplicates() {
        panic!("[ERROR]: duplicate files or directories are not allowed");
    }

    sync.both_files_exist()
        .expect("[ERROR]: failed to validate the existance");

    write(src_file.clone(), "matched content").expect("[ERROR]: failed to write to a file");
    write(dest_file.clone(), "not matched content").expect("[ERROR]: failed to write to a file");
    write(src_sub_file.clone(), "matched content").expect("[ERROR]: failed to write to a file");
    write(dest_sub_file.clone(), "not matched content")
        .expect("[ERROR]: failed to write to a file");

    assert_eq!(sync.both_files_exist(), Ok(()));
    assert_eq!(sync.path_matched(), true);
    assert_eq!(sync.hashes_matched(), false);
    assert_eq!(sync.do_update(), true);

    remove_file(&src_file).ok();
    remove_file(&dest_file).ok();
    remove_file(&src_sub_file).ok();
    remove_file(&dest_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_do_rename_different_paths_same_content() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(home_dir.clone()).join("tmpsrc_rename");
    let dest_parent_dir = PathBuf::from(home_dir).join("tmpdest_rename");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_file = src_parent_dir.join("file_hash1.txt");
    let dest_file = dest_parent_dir.join("file_hash.txt");

    let src_sub_dir = src_parent_dir.join("same_name_sub_dir1");
    let dest_sub_dir = dest_parent_dir.join("same_name_sub_dir");

    let src_sub_file = src_sub_dir.join("file_hash1.txt");
    let dest_sub_file = dest_sub_dir.join("file_hash.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");

    File::create(&src_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_file).expect("[ERROR]: failed to create a destination file");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");

    if sync.has_duplicates() {
        panic!("[ERROR]: duplicate files or directories are not allowed");
    }

    sync.both_files_exist()
        .expect("[ERROR]: failed to validate the existance");

    write(src_file.clone(), "matched content").expect("[ERROR]: failed to write to a file");
    write(dest_file.clone(), "matched content").expect("[ERROR]: failed to write to a file");
    write(src_sub_file.clone(), "matched content").expect("[ERROR]: failed to write to a file");
    write(dest_sub_file.clone(), "matched content").expect("[ERROR]: failed to write to a file");

    assert_eq!(sync.both_files_exist(), Ok(()));
    assert_eq!(sync.dir_matched(), false);
    assert_eq!(sync.file_matched(), false);
    assert_eq!(sync.hashes_matched(), true);
    assert_eq!(sync.do_rename(), true);

    remove_file(&src_file).ok();
    remove_file(&dest_file).ok();
    remove_file(&src_sub_file).ok();
    remove_file(&dest_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

// #[test]
// fn test_do_copy_extra_path() {
//     let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
//     let src_parent_dir = PathBuf::from(home_dir.clone()).join("tmpsrc_do_copy");
//     let dest_parent_dir = PathBuf::from(home_dir).join("tmpdest_do_copy");

//     if src_parent_dir.exists() {
//         remove_dir_all(&src_parent_dir).ok();
//     }

//     if dest_parent_dir.exists() {
//         remove_dir_all(&dest_parent_dir).ok();
//     }

//     let sync = SyncData {
//         source: src_parent_dir.clone(),
//         destination: dest_parent_dir.clone(),
//         changed_only: true,
//         delete: false,
//         verbose: false,
//         dry_run: false,
//     };

//     let src_file = src_parent_dir.join("file_hash.txt");
//     let src_sub_dir = src_parent_dir.join("same_name_sub_dir");
//     let src_sub_file = src_sub_dir.join("file_hash.txt");

//     create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
//     create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");

//     File::create(&src_file).expect("[ERROR]: failed to create a source file");
//     File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");

//     if sync.has_duplicates() {
//         panic!("[ERROR]: duplicate files or directories are not allowed");
//     }

//     write(src_file.clone(), "matched content").expect("[ERROR]: failed to write to a file");
//     write(src_sub_file.clone(), "matched content").expect("[ERROR]: failed to write to a file");

//     assert!(sync.both_files_exist().is_err());
//     assert_eq!(sync.src_dest_dirname_matched(), false);
//     assert_eq!(sync.src_dest_filename_matched(), false);
//     assert_eq!(sync.hashes_matched(), false);
//     assert_eq!(sync.do_rename(), false);
//     assert_eq!(sync.do_copy(), true);

//     remove_file(&src_file).ok();
//     remove_file(&src_sub_file).ok();
//     remove_dir_all(&src_sub_dir).ok();
//     remove_dir_all(&src_parent_dir).ok();
//     remove_dir_all(&dest_parent_dir).ok();
// }

#[test]
fn test_do_nothing_file_and_hash_matched() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(home_dir.clone()).join("tmpsrc_do_nothing");
    let dest_parent_dir = PathBuf::from(home_dir).join("tmpdest_do_nothing");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_file = src_parent_dir.join("file_hash.txt");
    let dest_file = dest_parent_dir.join("file_hash.txt");

    let src_sub_dir = src_parent_dir.join("same_name_sub_dir");
    let dest_sub_dir = dest_parent_dir.join("same_name_sub_dir");

    let src_sub_file = src_sub_dir.join("file_hash.txt");
    let dest_sub_file = dest_sub_dir.join("file_hash.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");

    File::create(&src_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_file).expect("[ERROR]: failed to create a destination file");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");

    if sync.has_duplicates() {
        panic!("[ERROR]: duplicate files or directories are not allowed");
    }

    sync.both_files_exist()
        .expect("[ERROR]: failed to validate the existance");

    write(src_file.clone(), "matched content").expect("[ERROR]: failed to write to a file");
    write(dest_file.clone(), "matched content").expect("[ERROR]: failed to write to a file");
    write(src_sub_file.clone(), "matched content").expect("[ERROR]: failed to write to a file");
    write(dest_sub_file.clone(), "matched content").expect("[ERROR]: failed to write to a file");

    assert_eq!(sync.both_files_exist(), Ok(()));
    assert_eq!(sync.path_matched(), true);
    assert_eq!(sync.hashes_matched(), true);
    assert_eq!(sync.do_nothing(), true);

    remove_file(&src_file).ok();
    remove_file(&dest_file).ok();
    remove_file(&src_sub_file).ok();
    remove_file(&dest_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

// the directory is matched but the file name is not matched, fix it
// #[test]
// fn test_update_same_path_different_hash() {
//     let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
//     let src_parent_dir = PathBuf::from(home_dir.clone()).join("tmpsrc_update");
//     let dest_parent_dir = PathBuf::from(home_dir).join("tmpdest_update");

//     if src_parent_dir.exists() {
//         remove_dir_all(&src_parent_dir).ok();
//     }

//     if dest_parent_dir.exists() {
//         remove_dir_all(&dest_parent_dir).ok();
//     }

//     let src_file = src_parent_dir.join("file_hash.txt");
//     let dest_file = dest_parent_dir.join("file_hash.txt");

//     let src_sub_dir = src_parent_dir.join("same_name_sub_dir");
//     let dest_sub_dir = dest_parent_dir.join("same_name_sub_dir");

//     let src_sub_file = src_sub_dir.join("file_hash.txt");
//     let dest_sub_file = dest_sub_dir.join("file_hash.txt");

//     create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
//     create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
//     create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a sub source directory");
//     create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a sub destination directory");

//     File::create(&src_file).expect("[ERROR]: failed to create a source file");
//     File::create(&dest_file).expect("[ERROR]: failed to create a destination file");
//     File::create(&src_sub_file).expect("[ERROR]: failed to create a sub source file");
//     File::create(&dest_sub_file).expect("[ERROR]: failed to create a sub destination file");

//     write(src_file.clone(), "matched content").expect("[ERROR]: failed to write to a file");
//     write(dest_file.clone(), "matched content").expect("[ERROR]: failed to write to a file");
//     write(src_sub_file.clone(), "matched content").expect("[ERROR]: failed to write to a file");
//     write(dest_sub_file.clone(), "matched content").expect("[ERROR]: failed to write to a file");

//     let sync = SyncData {
//         source: src_parent_dir.clone(),
//         destination: dest_parent_dir.clone(),
//         changed_only: true,
//         delete: false,
//         verbose: false,
//         dry_run: false,
//     };

//     // check file matching
//     let src_dest_exists = sync.src_dest_dir_exists() && sync.src_dest_file_exists();
//     let dest_src_name_matched = sync.dest_src_dir_exists() && sync.dest_src_file_exists();

//     let path_name_matched = src_dest_exists && dest_src_name_matched;
//     let empty_content_matched =
//         sync.empty_file_content_matched() && sync.empty_dir_content_matched();
//     let non_empty_content_matched =
//         sync.non_empty_file_content_matched() && sync.non_empty_dir_content_matched();

//     assert_eq!(path_name_matched, true);
//     assert_eq!(src_dest_exists, true);
//     assert_eq!(dest_src_name_matched, true);
//     assert!(empty_content_matched == true || non_empty_content_matched == true);

//     let matching = sync.update();
//     assert_eq!(matching, true);

//     remove_file(&src_file).ok();
//     remove_file(&dest_file).ok();
//     remove_file(&src_sub_file).ok();
//     remove_file(&dest_sub_file).ok();
//     remove_dir_all(&src_sub_dir).ok();
//     remove_dir_all(&dest_parent_dir).ok();
//     remove_dir_all(&src_parent_dir).ok();
//     remove_dir_all(&dest_parent_dir).ok();
// }

// ----------------------------------------------- Edge Case ------------------------------------------------

#[test]
fn test_src_empty() {
    let mut list_file: Vec<PathBuf> = Vec::new();
    let mut list_dir: Vec<PathBuf> = Vec::new();

    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(home_dir).join("tmpsrc_empty");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: PathBuf::new(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let entries = read_dir(&src_parent_dir).expect("[ERROR]: failed to read the source directory");
    let searched_files: Vec<PathBuf> = entries.filter_map(|f| f.ok()).map(|f| f.path()).collect();

    let src_files = sync.list_src_files();
    let src_dirs = sync.list_src_dirs();

    for file in src_files {
        list_file.push(file);
    }

    for entry in src_dirs {
        if entry == src_parent_dir {
            continue;
        }

        let entry_path = entry.iter().last();
        let dir = entry_path.expect("[ERROR]: failed to get the file");
        list_dir.push(PathBuf::from(dir.display().to_string()));
    }

    assert!(list_file.len() == 0 && list_dir.len() == 0 && searched_files.len() == 0);

    remove_dir_all(&src_parent_dir).ok();
}

#[test]
fn test_dest_empty() {
    let mut list_file: Vec<PathBuf> = Vec::new();
    let mut list_dir: Vec<PathBuf> = Vec::new();

    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let dest_parent_dir = PathBuf::from(home_dir).join("tmpdest_empty");

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");

    let sync = SyncData {
        source: PathBuf::new(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let entries = read_dir(&dest_parent_dir).expect("[ERROR]: failed to read the directory");
    let searched_files: Vec<PathBuf> = entries.filter_map(|f| f.ok()).map(|f| f.path()).collect();

    let dest_files = sync.list_dest_files();
    let dest_dirs = sync.list_dest_dirs();

    for file in dest_files {
        list_file.push(file);
    }

    for entry in dest_dirs {
        if entry == dest_parent_dir {
            continue;
        }

        let entry_path = entry.iter().last();
        let dir = entry_path.expect("[ERROR]: failed to get the file");
        list_dir.push(PathBuf::from(dir.display().to_string()));
    }

    assert!(list_file.len() == 0 && list_dir.len() == 0 && searched_files.len() == 0);

    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_dest_files_not_removed() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_files_not_removed");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_files_not_removed");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let dest_sub_dir = dest_parent_dir.join("test_dir");

    let src_sub_file = src_sub_dir.join("common.txt");
    let dest_sub_file = dest_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");
    write(&src_sub_file, "source content").expect("[ERROR]: failed to write the file");
    write(&dest_sub_file, "destination content").expect("[ERROR]: failed to write the file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_files = sync.list_src_files();
    let dest_files = sync.list_dest_files();

    let srcfile: Vec<PathBuf> = src_files
        .iter()
        .filter_map(|f| {
            f.file_name()
                .and_then(|name| name.to_str().map(PathBuf::from))
        })
        .collect();

    let destfile: Vec<PathBuf> = dest_files
        .iter()
        .filter_map(|f| {
            f.file_name()
                .and_then(|name| name.to_str().map(PathBuf::from))
        })
        .collect();

    assert_eq!(srcfile, destfile);
    assert!(srcfile.len() != 0 && destfile.len() != 0);

    let mut srcdirs: Vec<PathBuf> = Vec::new();
    let mut destdirs: Vec<PathBuf> = Vec::new();

    let src_dirs = sync.list_src_dirs();
    let dest_dirs = sync.list_dest_dirs();

    for src in src_dirs {
        if src == sync.source {
            continue;
        }
        let source = src
            .to_string_lossy()
            .to_string()
            .split("/")
            .last()
            .expect("[ERROR]: failed to get the last name")
            .to_string();
        srcdirs.push(PathBuf::from(source));
    }

    for dest in dest_dirs {
        if dest == sync.destination {
            continue;
        }
        let directory = dest
            .to_string_lossy()
            .to_string()
            .split("/")
            .last()
            .expect("[ERROR]: failed to get the last name")
            .to_string();
        destdirs.push(PathBuf::from(directory));
    }

    assert_eq!(srcdirs, destdirs);
    assert!(srcdirs.len() != 0 && destdirs.len() != 0);

    remove_file(&src_sub_file).ok();
    remove_file(&dest_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_dest_file_not_updated() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_file_not_updated");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_file_not_updated");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let dest_sub_dir = dest_parent_dir.join("test_dir");

    let src_sub_file = src_sub_dir.join("common.txt");
    let dest_sub_file = dest_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");
    write(&dest_sub_file, "destination content").expect("[ERROR]: failed to write the file");
    thread::sleep(time::Duration::from_secs(1));
    write(&src_sub_file, "source content").expect("[ERROR]: failed to write the file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_files = sync.list_src_files();
    let dest_files = sync.list_dest_files();

    let mut src_content = String::new();
    let mut dest_content = String::new();

    for src_file in src_files {
        for dest_file in &dest_files {
            src_content =
                read_to_string(&src_file).expect("[ERROR]: failed to read the source file");
            dest_content =
                read_to_string(&dest_file).expect("[ERROR]: failed to read the destination file");
        }
    }

    assert!(src_content != dest_content);

    remove_file(&src_sub_file).ok();
    remove_file(&dest_sub_file).ok();
    remove_dir_all(&src_sub_dir).ok();
    remove_dir_all(&dest_sub_dir).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_no_dest_file() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_no_dest_file");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_no_dest_file");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let dest_sub_dir = dest_parent_dir.join("test_dir");

    let src_sub_file = src_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&src_sub_file).expect("[ERROR]: failed to create a source file");
    thread::sleep(time::Duration::from_secs(1));
    write(&src_sub_file, "source content").expect("[ERROR]: failed to write the file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_files = sync.list_src_files();
    let dest_files = sync.list_dest_files();

    let mut src_content = String::new();
    for src_file in src_files {
        src_content = read_to_string(&src_file).expect("[ERROR]: failed to read the source file");
    }

    assert!(!src_content.is_empty() && dest_files.is_empty());

    remove_file(&src_sub_file).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}

#[test]
fn test_no_src_file() {
    let home_dir = env::var("HOME").expect("[ERROR]: failed to get the home directory");
    let src_parent_dir = PathBuf::from(&home_dir).join("tmpsrc_no_src_file");
    let dest_parent_dir = PathBuf::from(&home_dir).join("tmpdest_no_src_file");

    if src_parent_dir.exists() {
        remove_dir_all(&src_parent_dir).ok();
    }

    if dest_parent_dir.exists() {
        remove_dir_all(&dest_parent_dir).ok();
    }

    let src_sub_dir = src_parent_dir.join("test_dir");
    let dest_sub_dir = dest_parent_dir.join("test_dir");

    let dest_sub_file = dest_sub_dir.join("common.txt");

    create_dir_all(&src_parent_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_parent_dir).expect("[ERROR]: failed to create a destination directory");
    create_dir_all(&src_sub_dir).expect("[ERROR]: failed to create a source directory");
    create_dir_all(&dest_sub_dir).expect("[ERROR]: failed to create a destination directory");
    File::create(&dest_sub_file).expect("[ERROR]: failed to create a destination file");
    thread::sleep(time::Duration::from_secs(1));
    write(&dest_sub_file, "source content").expect("[ERROR]: failed to write the file");

    let sync = SyncData {
        source: src_parent_dir.clone(),
        destination: dest_parent_dir.clone(),
        changed_only: true,
        delete: false,
        verbose: false,
        dry_run: false,
    };

    let src_files = sync.list_src_files();
    let dest_files = sync.list_dest_files();

    let mut dest_content = String::new();
    for dest_file in dest_files {
        dest_content =
            read_to_string(&dest_file).expect("[ERROR]: failed to read the destination file");
    }

    assert!(!dest_content.is_empty() && src_files.is_empty());

    remove_file(&dest_sub_file).ok();
    remove_dir_all(&src_parent_dir).ok();
    remove_dir_all(&dest_parent_dir).ok();
}
