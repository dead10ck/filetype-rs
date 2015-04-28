#[cfg(test)] use super::FileType;
#[cfg(test)] use super::UnixFileType;
#[cfg(test)] use std::fs;
#[cfg(test)] use std::fs::File;

#[test]
fn regular_file() {
    let fname = "foo";
    let f = File::create(fname).unwrap();
    assert_eq!(f.file_type().unwrap(), FileType::Regular);
    assert!(fs::remove_file(fname).is_ok());
}

#[test]
fn diretory() {
    let dirname = "testdir";
    assert!(fs::create_dir(dirname).is_ok());
    let f = File::open(dirname).unwrap();
    assert_eq!(f.file_type().unwrap(), FileType::Directory);
    assert!(fs::remove_dir(dirname).is_ok());
}
