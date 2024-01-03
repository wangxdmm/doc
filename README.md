# 🌈 doc

## Why use it ?

When I want to read docs of some frameworks, I always do this:

1. Open Chrome and find the bookmark, click it, google it, and click the link.
2. I repeat `1 step` day and a day, it so wastes time.
3. I want a command-line tool to combine all the docs I will read.
4. Some frameworks have their playground to test features directly, I always use google to find them -v-.

So, `doc` is a command-line tool that can help you open `document`, `playground`, `repositories` easily.

## Install

[Download](https://github.com/wangxdmm/doc/releases)

## Usage

We must have an config file locate in your home folder named `~/.doc.toml`

```shell
# Lin: Some(/home/alice)
# Win: Some(C:\Users\Alice)
# Mac: Some(/Users/Alice)
```

### Init

You can use `doc init` to create an initial config file.

```shell
# init by remote url
doc init -u https://raw.githubusercontent.com/wangxdmm/doc_config/main/my.toml
# patch your local config by remote 
doc init -m -u https://raw.githubusercontent.com/wangxdmm/doc_config/main/my.toml
# force update your local config
doc init -u https://raw.githubusercontent.com/wangxdmm/doc_config/main/my.toml -f

```

### Example

```toml
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
