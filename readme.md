A simple crate to collect information about the build environment. Info about the current cargo pkg version, git info, OS info and rustc version

# Usage

```toml
[dependencies]
info = { git = "https://github.com/RuairidhWilliamson/info" }

[build-dependencies]
info = { git = "https://github.com/RuairidhWilliamson/info" }
```

You must setup a build script with the following:

```rust no_run
fn main() {
   info::build_script();
}
```

# Example

```rust
use info::{Info, raw_info};

let info = Info::new(raw_info!());
println!("{info}");

// Or use a static LazyLock to fetch once and reuse
let info_str = lazy_info_str!();
println!("{info_str}");
```
