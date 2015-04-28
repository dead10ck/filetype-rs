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

/* There doesn't appear to be a way to open a file without the underlying
 * system resolving the symlink, so it's not clear if it's even possible to open
 * a file in Rust which would have the `S_IFLNK` file mask.
#[test]
fn symlink() {
    let fname = "foo";
    let link_name = "foolink";
    let f = File::create(fname).unwrap();

    // make the link
    assert!(fs::soft_link(fname, link_name).is_ok());

    // test its type
    let link_file = File::open(link_name).unwrap();
    assert_eq!(link_file.file_type().unwrap(), FileType::Symlink);

    assert!(fs::remove_file(link_name).is_ok());

    if let Err(e) = fs::remove_file(fname) {
        println!("Error removing file: {}", e);
    }
}
*/
