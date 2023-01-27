# dfang

Defang IOCs (ips, emails, urls, etc)

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
