# 🌈 doc

## Why use it ?

When I want to read docs of some frameworks, I always do this:

1. Open Chrome and find the bookmark, click it, google it, and click the link.
2. I repeat `1` day and a day, it so wastes time.
3. I want a command-line tool to combine all the docs I will read.
4. Some frameworks have their playground to test features directly, I always use google to find them -v-.

So, `doc` is a command-line tool that can help you open `document`, `playground`, `repositories` easily.

## Install

See Release.

## Usage

We must have an config file locate in your home folder named `~/.doc.toml`

```shell
# Lin: Some(/home/alice)
# Win: Some(C:\Users\Alice)
# Mac: Some(/Users/Alice)
```

You can use `doc init` to create an initial config file.

```
[map]

[map.rs]
full = "rust"
url = "https://doc.rust-lang.org"
search = "https://doc.rust-lang.org/std/index.html?search={}"
play = "https://play.rust-lang.org/"
rep = "https://github.com/rust-lang/rust"

```
- `doc rs` will open `https://doc.rust-lang.org`
- `doc rs -p` will open `https://play.rust-lang.org/`
- `doc rs String -s` will open `https://doc.rust-lang.org/std/index.html?search=String`
- `doc ls` will list all docs in your config file
- `doc rs -r` will open `https://github.com/rust-lang/rust`
