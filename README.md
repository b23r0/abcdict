# abcdict [![Build Status](https://img.shields.io/github/workflow/status/b23r0/abcdict/Rust)](https://github.com/b23r0/abcdict/actions/workflows/rust.yml) [![ChatOnDiscord](https://img.shields.io/badge/chat-on%20discord-blue)](https://discord.gg/ZKtYMvDFN4) [![Crate](https://img.shields.io/crates/v/abcdict)](https://crates.io/crates/abcdict)
A better customization password dictionary generator.

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

| Control Character        | Example | Description        | Arguments |
|----------------|----------|-------------|-------------|
| p         | [p0-2-1-12]     | range of numbers with padded       |  {pad}-{length}-{begin}-{end}  |
| c | [cA-z]     | range of ASCII characters   | {begin}-{end} |
| s         | [sjack-tom]     | collection of strings       | {string1}-{string2}-...-{stringN} |
| n          | [n2012-2021]     | range of numbers   | {begin}-{end} |
| x         | [x10]     | factorial previous unit       | {factorial layers} |

# Examples

```
$> ./abcdict 2021[p0-2-1-12]

202101
202102
202103
202104
202105
202106
202107
202108
202109
202110
202111
202112

```

```
$> ./abcdict [sjack-tom-danny]like[skathy-nancy]

jacklikekathy
jacklikenancy
tomlikekathy
tomlikenancy
dannylikekathy
dannylikenancy

```

```
$> ./abcdict [sadmin-root-manager-master][c*-/]

admin*
admin+
admin,
admin-
admin.
admin/
root*
root+
root,
root-
root.
root/
manager*
manager+
manager,
manager-
manager.
manager/
master*
master+
master,
master-
master.
master/

```

```
$>./abcdict ab[sc][x10]de

abcde
abccde
abcccde
abccccde
abcccccde
abccccccde
abcccccccde
abccccccccde
abcccccccccde
abccccccccccde

```
