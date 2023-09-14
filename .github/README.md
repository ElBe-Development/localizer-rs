<h1 align="center">
    localizer-rs
</h1>
<h3 align="center">
    Localizer helps localize (translate) your rust applications using json files.
</h3>
<p align="center">
    <img src="https://img.shields.io/crates/v/qstash-rs">
    <img src="https://www.codefactor.io/repository/github/ElBe-Development/localizer-rs/badge">
    <img src="https://github.com/ElBe-Development/localizer-rs/actions/workflows/megalinter.yml/badge.svg?branch=main&event=push">
    <img src="https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit">
</p>

<img src="https://github.com/ElBe-Development/localizer-rs/blob/main/.github/example.png?raw=true" width="500px"/>

## About this project

Localizer is a tool to translate text using json files.

## Installing

Run the following command to add the package to your dependencies:

```bash

$ cargo add localizer-rs
...

```

### Git

To clone the repository locally using git run `git clone https://github.com/ElBe-Development/localizer-rs.git`.

## Usage

To use localizer-rs, you need a directory (eg. `translations`) with your translations files (eg. `en.json`). You then need to follow these steps:

1. Import the localizer-rs crate:

   ```rust

   use localizer_rs;

   ```

2. Create a new config object:

   ```rust

   let config = localizer_rs::Config::new("DIRECTORY NAME", "LANGUAGE NAME");

   ```

3. Translate your text:

   ```rust

   config.t("key", vec!["placeholder", "value"]);

   ```

## Example

With the following `en.json` file.

```json

{
    "error": "{{color.red}}{{bold}}Error:{{end}} Something went wrong: {{details}}."
}

```

And the following rust code.

```rust

use localizer_rs;

fn main() {
    let config: localizer_rs::Config = localizer_rs::Config::new("translations", "en");

    println!("{:}", config.t("error", vec![("details", "Path not found")]));
}

```

You will get the following output:

```bash

Error: Something went wrong: Path not found.

```

Where `Error:` is red and bold.

## Contact

To contact us, get help or just chat with others, you can visit [our discord server](https://discord.gg/JVyyDukQqV).
