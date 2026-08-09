![dfang](doc/img/dfang_logo.png)

## Intro

For when you need to quickly make IOCs (email, urls, ip addresses) unclickable and safe to send, just send them through `dfang`. If you receive something that's been defanged and you need to give it some teeth again, just run it back through `rfang`.

## Install

```shell
cargo install dfang
cargo install rfang
```

## Usage

```shell
dfang something@somewhere.com
rfang something[@]somewhere[.]com
```

...or pipe in from another application

```shell
# Extract and refang the defanged URLs in a file
grep -i hxxp iocs.txt | rfang

# Take your clipboard, defang it, and copy it again
pbpaste | dfang | pbcopy
```

## Use as a library

Both crates ship a library alongside the binary, so the string processing can be called
directly from Rust instead of shelling out. Neither has any dependencies and neither does
any I/O.

```shell
cargo add dfang
cargo add rfang
```

```rust
use dfang::defang;
use rfang::refang;

assert_eq!(defang("http://example.com"), "hxxp[://]example[.]com");
assert_eq!(refang("hxxp[://]example[.]com"), "http://example.com");
```
