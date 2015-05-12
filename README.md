[![Build Status](https://travis-ci.org/dead10ck/filetype-rs.svg?branch=master)](https://travis-ci.org/dead10ck/filetype-rs)

[API Documentation](http://dead10ck.github.io/filetype-rs/filetype/)

**This is a pet project, and not tested very well (contributions welcome!).
Use at your own risk.**

This crate provides a basic extension to `std::fs::File`: it defines a method
which returns the file's type (on *nix systems).

```rust
extern crate filetype;

use std::fs;
use std::fs::File;
use filetype::{FileType, UnixFileType};

let f = File::open("foo").unwrap();
let ftype = f.file_type().unwrap();

match ftype {
    FileType::Regular => {},
    FileType::Directory => {},
    FileType::Symlink => {},
    FileType::NamedPipe => {},
    FileType::BlockDevice => {},
    FileType::CharacterDevice => {},
}
```
