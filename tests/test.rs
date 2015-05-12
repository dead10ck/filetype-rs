extern crate libc;
extern crate nix;
extern crate filetype;

use filetype::FileType;
use filetype::UnixFileType;
use std::path::Path;
use std::fs;
use std::fs::{File, OpenOptions};
use libc::consts::os::posix88;
// use std::os::unix::io::FromRawFd;
// use nix::unistd;
use nix::sys::stat;
use nix::sys::stat::{SFlag, Mode};

#[test]
fn regular_file_file() {
    let fname = "foo";
    let f = File::create(fname).unwrap();
    assert_eq!(f.file_type().unwrap(), FileType::Regular);
    assert!(fs::remove_file(fname).is_ok());
}

#[test]
fn regular_file_path() {
    let path = Path::new("foopath");
    let _ = File::create(path).unwrap();
    assert_eq!(path.file_type().unwrap(), FileType::Regular);
    assert!(fs::remove_file(path).is_ok());
}

#[test]
fn diretory_file() {
    let dirname = "testdir";
    assert!(fs::create_dir(dirname).is_ok());
    let f = File::open(dirname).unwrap();
    assert_eq!(f.file_type().unwrap(), FileType::Directory);
    assert!(fs::remove_dir(dirname).is_ok());
}

#[test]
fn diretory_path() {
    let dir_path = Path::new("testdirpath");
    assert!(fs::create_dir(dir_path).is_ok());
    assert_eq!(dir_path.file_type().unwrap(), FileType::Directory);
    assert!(fs::remove_dir(dir_path).is_ok());
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
fn symlink_path() {
    let fname = "symbase";
    let link_path = Path::new("symlink");
    let _ = File::create(fname).unwrap();

    // make the link
    assert!(fs::soft_link(fname, link_path).is_ok());
    assert!(File::open(link_path).is_ok());

    // test its type
    assert_eq!(link_path.file_type().unwrap(), FileType::Symlink);
    assert!(fs::remove_file(link_path).is_ok());
    assert!(fs::remove_file(fname).is_ok());
}

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
// there doesn't appear to be an equivalent block device on MacOS that is
// user-accessible. There is /dev/disk*, but these require root privileges.
// Additionally, mknod also requires root privileges to create block devices.
// Consequently, there doesn't appear to be a way to test block devices on
// MacOS without root privileges.
#[cfg(target_os = "linux")]
fn block() {
    let fname = "/dev/sda";
    let f = match File::open(fname) {
        Ok(f) => f,
        Err(ref e) => {
            // skip the test if opening the block device fails
            println!("error opening {}: {}", fname, e);
            return;
        }
    };
    assert_eq!(f.file_type().unwrap(), FileType::BlockDevice);
}

#[test]
fn character() {
    let fname = "/dev/random";
    let f = File::open(fname).unwrap();
    assert_eq!(f.file_type().unwrap(), FileType::CharacterDevice);
}
