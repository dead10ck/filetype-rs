extern crate libc;
extern crate nix;

use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::convert::From;
use nix::sys::stat;
use nix::sys::stat::fstat;
use libc::consts::os::posix88;

pub type FileTypeResult = Result<FileType, Error>;

/// Can return a Unix file type.
pub trait UnixFileType {
    fn file_type(&self) -> FileTypeResult;
}

/// Every standard Unix file type (except for Sockets, since this
/// is not provided by `libc::consts::os::posix88`)
pub enum FileType {
    Regular,
    Directory,
    Symlink,
    NamedPipe,
    BlockDevice,
    CharacterDevice,
}

/// An error which occurred during file type determination.
/// Either an error returned by `nix::sys::stat::fstat`, or
/// the file type is uknown. If unknown, the file mask is
/// returned; the file mask is the `st_mode` returned by
/// `stat` bitwise-ANDed with `libc::consts::os::posix88::S_IFMT`.
/// See the man pages for `fstat` for more information.
pub enum Error {
    NixError(nix::Error),
    UnknownFileType(u32),
}

impl From<nix::Error> for Error {
    fn from(nix_err : nix::Error) -> Self {
        Error::NixError(nix_err)
    }
}

/// Returns the file type of `f`.
impl UnixFileType for File {
    fn file_type(&self) -> FileTypeResult {
        let fd = self.as_raw_fd();
        let fstat = try!(stat::fstat(fd));
        let file_mask = fstat.st_mode & posix88::S_IFMT;
        get_file_type(file_mask)
    }
}

fn get_file_type(file_mask : u32) -> FileTypeResult {
    match file_mask {
        posix88::S_IFREG => Ok(FileType::Regular),
        posix88::S_IFDIR => Ok(FileType::Directory),
        posix88::S_IFLNK => Ok(FileType::Symlink),
        posix88::S_IFIFO => Ok(FileType::NamedPipe),
        posix88::S_IFBLK => Ok(FileType::BlockDevice),
        posix88::S_IFCHR => Ok(FileType::CharacterDevice),
        _ => Err(Error::UnknownFileType(file_mask)),
    }
}
