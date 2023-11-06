# merge-cfg

Merge or cover config based on priorities.

## Usage

You can get your configuration from config file in any way at first. Then derive `MergeCfg` to merge your configuration from command line arguments.

```rust
use merge_cfg::MergeCfg;

#[derive(Debug, MergeCfg)]
struct Config {
    id: u64,
    name: String,
    scores: Vec<i32>,
}

fn main() {
    let mut cfg = Config {
        id: 1,
        name: "abc".to_string(),
        scores: vec![-1],
    };
    cfg.merge_cfg();
    println!("{:?}", cfg);
}
```

Command line arguments format: `{field}={value}`, field name should be the same with what you defined in your structure:

```shell
$ cargo run
Config { id: 1, name: "abc", scores: [-1] }

$ cargo run id=2
Config { id: 2, name: "abc", scores: [-1] }

$ cargo run name=xyz
Config { id: 1, name: "xyz", scores: [-1] }

$ cargo run scores=100 scores=-1000
Config { id: 1, name: "abc", scores: [100, -1000] }

$ cargo run id=2 name=xyz scores=100
Config { id: 2, name: "xyz", scores: [100] }
```

## Roadmap

- [ ] Support environment variables
  - [ ] Merge priority
  - [ ] Merge option (choose which to merge)
- [ ] More command line argument formats
  - [x] Simple equal format: `{field}={value}`
  - [ ] [getopt](https://en.wikipedia.org/wiki/Getopt) format: `-h --{field} {value}`
- [ ] Various use cases
  - [ ] Non-panic function
  - [ ] Immutable function
- [ ] Field Alias
- [ ] Support complex field type
- [ ] Document
- [ ] Readme
- [ ] ...
