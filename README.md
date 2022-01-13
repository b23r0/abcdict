# abcdict
A better customization dictionary generator implementation by Rust.

# Features

* Cli
* Faster
* Customize Rules

# Build & Installation

`$> cargo build --release`

`$> cargo install abcdict`

# Uasge

## Example

`$> ./abcdict jack[n2012-2013][ca-c][s@-#]`

```
jack2012a@
jack2012a#
jack2012b@
jack2012b#
jack2012c@
jack2012c#
jack2013a@
jack2013a#
jack2013b@
jack2013b#
jack2013c@
jack2013c#
```

## Control Block

| Control Character        | Example | Description        |
|----------------|----------|-------------|
| p         | [p0-2-1-12]     | range of numbers with padded       |
| c | [cA-z]     | range of ASCII characters   |
| s         | [sjack-tom]     | collection of strings       |
| n          | [n2012-2021]     | range of numbers   |
| x         | [x10]     | factorial previous unit       |