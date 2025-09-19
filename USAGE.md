# Usage

[Example](/examples/simple)

```toml
# Cargo.toml

# ..
[dependencies]
libbuildinfo = { git = "https://github.com/charliethomson/libbuildinfo" }

[build-dependencies]
libbuildinfo = { git = "https://github.com/charliethomson/libbuildinfo" }
# ..

```

```rs
// build.rs

fn main() {
    // ..
    libbuildinfo::emit().expect("Failed to inject build info");
    // ..
}

```

```rs
// main.rs

fn main() {
    // ..
    libbuildinfo::load_build_info!().expect("Failed to load build info")
    // ..
}

```
