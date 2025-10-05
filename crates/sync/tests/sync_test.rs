use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use sync::changed_only::{else_file_changed_only, file_changed_only};
use sync::utils::create_temp_file;
use tempfile::tempdir;

#[test]
fn test_changed_only_copies_missing_file() {
    // temp dirs
    let tmp_dir = tempdir().unwrap();
    let source = tmp_dir.path().join("source_test");
    let destination = tmp_dir.path().join("destination_test");

    // clean if exists
    if source.exists() {
        fs::remove_dir_all(&source).unwrap();
    }
    if destination.exists() {
        fs::remove_dir_all(&destination).unwrap();
    }
    fs::create_dir(&source).unwrap();
    fs::create_dir(&destination).unwrap();

    // create a file in source
    let src_file_path = source.join("hello.txt");
    let mut src_file = File::create(&src_file_path).unwrap();
    writeln!(src_file, "Hello world!").unwrap();

    // fake "last modified" timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // build input vectors
    let src_files = vec![(
        src_file_path.clone(),
        timestamp,
        "pretty_time".to_string(),
        (12.0, "B"),
    )];

    let source_files = vec![(src_file_path.clone(), timestamp)];
    let destination_files: Vec<(PathBuf, i64)> = vec![];

    // call function
    file_changed_only(
        &src_files,
        &source_files,
        &destination_files,
        &source,
        &destination,
    );

    // assert file got copied
    let copied_file = destination.join("hello.txt");
    assert!(copied_file.exists(), "File was not copied to destination");

    // cleanup
    fs::remove_dir_all(&source).unwrap();
    fs::remove_dir_all(&destination).unwrap();
}

#[test]
fn test_else_file_changed_only_copies() {
    let src_dir = tempdir().unwrap();
    let dest_dir = tempdir().unwrap();

    let src_path = src_dir.path().to_path_buf();
    let dest_path = dest_dir.path().to_path_buf();

    let src_file = create_temp_file(&src_path, "file1.txt", "hello");
    let dest_file = create_temp_file(&dest_path, "old_file.txt", "outdated");

    let source_files = vec![(src_file.clone(), 0)];
    let destination_files = vec![(dest_file.clone(), 0)];

    else_file_changed_only(&source_files, &destination_files, &src_path, &dest_path);

    let new_dest_file = dest_path.join("file1.txt");
    assert!(new_dest_file.exists(), "Expected copied file to exist");
    assert!(!dest_file.exists(), "Expected old file to be deleted");

    let copied_content = fs::read_to_string(new_dest_file).unwrap();
    assert_eq!(copied_content, "hello");
}
