tempify-rs
==========

auto-remove temp file or dir when drop

usage
-----

1. add dependency into Cargo.toml

```toml
[dependencies]
tempify = { git = "https://github.com/0x75960/tempify-rs", branch = "master" }
```

2. import and use in your code

### case file (or dir) has not exists yet

```rust
extern crate tempify;

use tempify::Temp;

fn main() {
	let temp = Temp::new().unwrap();
	let f = File::create(temp.path.as_str()).unwrap();
	// ... something use temp file ...
	// file or directory that in temp.path will be removed when "temp"  has dropped.
}
```

### case file (or dir) has already exists

```rust
extern crate tempify;

use tempify::Temp;

fn main() {
	let temp = Temp::as_temp("/path/to/file_or_dir");
	// ...
	// file or directory that in "/path/to/file_or_dir" will be removed when "temp" has dropped.
}
```

test
----

```sh
$ cargo test
```
