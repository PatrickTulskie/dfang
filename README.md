![dfang](doc/img/dfang_logo.png)

## Intro

For when you need to quickly make IOCs (email, urls, ip addresses) unclickable and safe to send, just send them through `dfang`. If you receive something that's been defanged and you need to give it some teeth again, just run it back through `rfang`.

## Install

```
cargo install dfang
cargo install rfang
```

## Usage

```
dfang something@somewhere.com
rfang something[@]somewhere[.]com
```

...or pipe in from another application

```
// Extract defanged URLs from a file
grep hxxp iocs.txt | rfang

// Take your clipboard, defang it, and copy it again
pbpaste | dfang | pbcopy
```
