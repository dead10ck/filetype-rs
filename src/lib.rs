extern crate libc;
extern crate nix;

use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::convert::From;
use nix::sys::stat::fstat;
use libc::consts::os::posix88;

/// Every standard Unix file type.
pub enum FileType {
    Regular,
    Directory,
    Symlink,
    NamedPipe,
    BlockDevice,
    CharacterDevice,
}

pub enum Error {
    NixError(nix::Error),
    UnknownFileType,
}

impl From<nix::Error> for Error {
    fn from(nix_err : nix::Error) -> Self {
        Error::NixError(nix_err)
    }
}

/// Returns the file type of `f`.
pub fn file_type(f : &File) -> Result<FileType, Error> {
    let fd = f.as_raw_fd();
    let fstat = try!(fstat(fd));
    let file_mask = fstat.st_mode & posix88::S_IFMT;
    get_file_type(file_mask)
}

fn get_file_type(file_mask : u16) -> Result<FileType, Error> {
    match file_mask {
        posix88::S_IFREG => Ok(FileType::Regular),
        posix88::S_IFDIR => Ok(FileType::Directory),
        posix88::S_IFLNK => Ok(FileType::Symlink),
        posix88::S_IFIFO => Ok(FileType::NamedPipe),
        posix88::S_IFBLK => Ok(FileType::BlockDevice),
        posix88::S_IFCHR => Ok(FileType::CharacterDevice),
        _ => Err(Error::UnknownFileType),
    }
}
