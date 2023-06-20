# Playground for clap and figment

Given the following function

```rust
fn main() {
    let config: Cli = Figment::new()
        .merge(Serialized::defaults(Cli::parse()))
        .merge(Toml::file("config.toml"))
        .merge(Env::raw())
        .extract()
        .expect("Failed to load config");

    println!("{config:#?}");
}
```

`config.toml`

```toml
name = "Bastian"
flag = true

[command.Test]
number = 42

```

I expect the ENV variable to override both the CLI and the config file values:

```bash
$ NUMBER=2 ./target/debug/clap-figment test
```

Outcome
```bash
$ NUMBER=2 ./target/debug/clap-figment test
Cli {
    name: Some(
        "Bastian",
    ),
    flag: Some(
        true,
    ),
    command: Some(
        Test {
            number: 42,
        },
    ),
}
```
