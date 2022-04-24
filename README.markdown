# RGET :rocket: :fire:

***Hypersonic downloads for your terminal.*** :rocket: :fire:

![GitHub CI](https://github.com/iamtheblackunicorn/rget/actions/workflows/rust.yml/badge.svg)

## ABOUT :books:

I've used the popular Unix tool `wget` in the past a lot. Now, since I'd heard that Rust supported asynchronous programming, I wanted to experiment with that. ***RGET*** is the culmination of those efforts. And the best part? ***RGET*** very graciously does all the heavy lifting for you. You just feed it a URL and it does the rest for you.

## USAGE :hammer:

Using ***RGET*** is very simple. It supports three commands:

- Download a file from the interwebs.

```bash
$ rget -d https://yoururl.com/yourfile.txt
# OR
$ rget --download https://yoururl.com/yourfile.txt
```

- Display version information:

```bash
$ rget -v
# OR
$ rget --version
```

- Get a usage guide:

```bash
$ rget -h
# OR
$ rget --help
```

## INSTALLATION :inbox_tray:

I'm assuming you have Rust and Git installed and available from the command line. If you do, run this command to install ***RGET***:

```bash
$ cargo install --git https://github.com/iamtheblackunicorn/rget
```

## BUILDING

### Tools

You will need the following tools installed and available:

- Rust
- Git

### Steps

- 1.) Get the source code:
```bash
$ git clone https://github.com/iamtheblackunicorn/rget.git
```
- 2.) Change directory:
```bash
$ cd rget
```
- 3.) Build the source code:
```bash
$ cargo build --release
```

This will produce an executable on the path `rget/target/release/rget`.

## CHANGELOG :black_nib:

### Version 1.0.0

- Initial release.
- Upload to GitHub.

## NOTE :scroll:

- *RGET :rocket: :fire:* by Alexander Abraham :black_heart: a.k.a. *"The Black Unicorn" :unicorn:*
- Licensed under the MIT license.## INSTALLATION
