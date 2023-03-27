# topclean

> Free up disk space with one command

`topclean` is intended to be used when you are running low on disk space and you are sure that deleting temporary files, caches, files marked for deletion (e.g. recycle bin) etc. is safe.

## Motivation and Inspiration

`topclean` has been inspired by these projects:
- [BleachBit](https://github.com/bleachbit/bleachbit)
- [topgrade](https://github.com/r-darwish/topgrade)

The goal is to provide disk cleaning functionality like BleachBit, but in a zero/low config way like topgrade offers for software upgrades. Another similarity to the latter is a focus on development tools.

Also, I wanted to try out Rust.

## Install and Run

### Windows

You can install topclean using scoop as below, or the same way as for other OS.

```ps1
# Add custom bucket
scoop bucket add topclean https://github.com/mertd/topclean
# Install
scoop install topclean
# Run
topclean
```

### Generic

Use this for all other OS.

Check out the repository and execute `cargo run` within the directory.

## Licence

[MIT](LICENCE)