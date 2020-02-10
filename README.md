# kat

Interactive tab-opener and thing-doer for the kitty terminal emulator.

Note: not ready for use by anyone but me.

## Install

```
cargo install --git https://github.com/dontlaugh/kat
```

I'm not going to put this on crates.io until it's in a better state.

## Overview

Kitty supports [remote control](https://sw.kovidgoyal.net/kitty/remote-control.html).
With this enabled, we can create a DSL to send commands to kitty. We can open
tabs, run programs, and do anything your shell could do.

You must enable remote control, and run `kat` from within a kitty instance.

Built on top of the [Molt Tcl interpreter](https://github.com/wduquette/molt).

## Configuration

Create a Tcl file at the expected location. The file will be evaluated when you 
run `kat`.

```
mkdir -p $HOME/.config/kat
touch $HOME/.config/kat/kat.tcl
```

You can override that location with `KAT_CONFIG`.
