![dfang](doc/img/dfang_logo.png)

## Intro

For when you need to quickly make IOCs (email, urls, ip addresses) unclickable and safe to send, just send them through `dfang`. If you receive something that's been defanged and you need to give it some teeth again, just run it back through `rfang`.

## Install

With Homebrew, on macOS or Linux:

```shell
brew tap PatrickTulskie/tap
brew trust PatrickTulskie/tap
brew install dfang
```

One formula carries both binaries. Homebrew 6 won't load a third-party tap until
it's trusted, which is what the `brew trust` line is for.

With cargo:

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

# Defang a log as it grows
tail -f app.log | dfang --line-buffered
```

Piped input is handled line by line as it arrives. Output is flushed in blocks,
which is fastest for big files; `--line-buffered` flushes after every line for
when the input trickles in or the next thing in the pipeline is interactive.

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
