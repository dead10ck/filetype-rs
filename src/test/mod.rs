use super::FileType;
use super::UnixFileType;
use std::fs;
use std::fs::File;
use std::result::Result;

#[test]
fn regular_file() {
    let fname = "foo";
    let f = File::create(fname).unwrap();
    assert_eq!(f.file_type().unwrap(), FileType::Regular);
    fs::remove_file(fname);
}
