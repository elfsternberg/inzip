# InZip: Just a learning experience

Inzip is a simple Rust crate that wraps the Zip crate in a simple API layer so
that client code can open a zip file and list the content, list contents
recursively, assert the presence of a path, and retrieve the contents of a file
if it exists.  The InZip layer has two constructors:

- `new` - Takes anything that supports Seek + Read
- `from_file` - Takes a filename and passes the Seek + Read File object to
  `new()`
  
The *idea* is that it would be possible to use `include_bytes!` in the an
executable and ship an entire website in a single binary.

## LICENSE

Inzip is Copyright [Elf M. Sternberg](https://elfsternberg.com) (c) 2019, and
licensed with the Mozilla Public License vers. 2.0. A copy of the license file
is included in the root folder.

