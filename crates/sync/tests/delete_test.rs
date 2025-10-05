use sync::{delete::delete_flag, utils::create_temp_file};
use tempfile::tempdir;

#[test]
fn test_delete_flag_removes_non_matching_files() {
    // Setup temporary source and destination directories
    let src_dir = tempdir().unwrap();
    let dest_dir = tempdir().unwrap();

    let src_path = src_dir.path().to_path_buf();
    let dest_path = dest_dir.path().to_path_buf();

    // Source has file1.txt
    let src_file = create_temp_file(&src_path, "file1.txt", "keep me");

    // Destination has file1.txt and extra.txt
    let dest_file1 = create_temp_file(&dest_path, "file1.txt", "keep me");
    let dest_file2 = create_temp_file(&dest_path, "extra.txt", "delete me");

    // Prepare file lists
    let source_files = vec![(src_file.clone(), 0)];
    let destination_files = vec![(dest_file1.clone(), 0), (dest_file2.clone(), 0)];

    // Run the function under test
    delete_flag(&source_files, &destination_files, &src_path, &dest_path);

    // Verify: file1.txt should remain, extra.txt should be deleted
    assert!(dest_file1.exists(), "file1.txt should not be deleted");
    assert!(!dest_file2.exists(), "extra.txt should have been deleted");
}
