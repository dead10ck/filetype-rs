#[cfg(test)] use super::FileType;
#[cfg(test)] use super::UnixFileType;
#[cfg(test)] use std::path::Path;
#[cfg(test)] use std::fs;
#[cfg(test)] use std::fs::{File, OpenOptions};
#[cfg(test)] use libc::consts::os::posix88;
//#[cfg(test)] use std::os::unix::io::FromRawFd;
//#[cfg(test)] use nix::unistd;
#[cfg(test)] use nix::sys::stat;
#[cfg(test)] use nix::sys::stat::{SFlag, Mode};

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

#[test]
fn pipe() {
    let fname = "foopipe";
    let file_flag = SFlag::from_bits(posix88::S_IFIFO).unwrap();
    let perms = Mode::from_bits(0o666).unwrap();
    let dev = 0;

    // create a named pipe
    if let Err(e) = stat::mknod(Path::new(fname), file_flag, perms, dev) {
        println!("Error creating pipe: {:?}", e);
    }

    let f_result = OpenOptions::new().read(true).write(true).open(fname);

    if let Err(ref e) = f_result {
        println!("Error opening pipe: {}", e);
        assert!(false);
    }

    let f = f_result.unwrap();

    assert_eq!(f.file_type().unwrap(), FileType::NamedPipe);

    // delete the block device
    assert!(fs::remove_file(fname).is_ok());
}

#[test]
fn block() {
    let fname = "/dev/sda";
    let f = File::open(fname).unwrap();
    assert_eq!(f.file_type().unwrap(), FileType::BlockDevice);
}

#[test]
fn character() {
    let fname = "/dev/random";
    let f = File::open(fname).unwrap();
    assert_eq!(f.file_type().unwrap(), FileType::CharacterDevice);
}
