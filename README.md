# ohq2quarto

Save an [Observable HQ](https://observablehq.com) Notebook to a [Quarto](https://quarto.org/) project.

Given an Observable Notebook reference (full URL or `@user/slug`) and an output directory, this utility will make a Quarto project directory, build `qmd` & `_quarto.yml` files and download all `FileAttachment`s.

## TODO

- [ ] [Handle Collections](https://github.com/hrbrmstr/ohq2quarto/issues/2)
- [ ] [Make this a lib+bin](https://github.com/hrbrmstr/ohq2quarto/issues/3)
- [ ] [Publish on crates.io](https://github.com/hrbrmstr/ohq2quarto/issues/4)

## Getting `ohq2quarto`

The [releases](https://github.com/hrbrmstr/ohq2quarto/releases) section has pre-built binaries for Windows and macOS (which is also signed universal binary).

Linux or just DIY folks can:

```shell
$ cargo install --git https://github.com/hrbrmstr/ohq2quarto # install it (~/.cargo/bin/ohq2quarto)
```

### Building/Using

```shell
$ cargo build # build it after cloning
```

````shell
$ cargo run -- --ohq-ref @hrbrmstr/just-one-more-thing --output-dir ./examples --verbose # run it after cloning
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/ohq2quarto --ohq-ref '@hrbrmstr/just-one-more-thing' --output-dir ./examples --verbose`
      Title: Just One More Thing
       Slug: just-one-more-thing
  Author(s): boB Rudis
  Copyright: Copyright 2022 boB Rudis
    License: "mit"
 Observable: https://observablehq.com/@hrbrmstr/just-one-more-thing

$ tree examples
├── _quarto.yml
├── columbo_data.csv
└── just-one-more-thing.qmd

$ head -16 examples/just-one-more-thing.qmd
---
title: 'Just One More Thing'
author: 'boB Rudis'
format: html
echo: false
observable: 'https://observablehq.com/@hrbrmstr/just-one-more-thing'
---

```{ojs}
md`# Just One More Thing`
```

```{ojs}
md`This week, Chris Holmes tweeted something super dangerous:`
```
````

```shell
$ cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/ohq2quarto --help`
ohq2quarto 0.1.0
boB Rudis (@hrbrmstr)
Given an Observable Notebook reference, create a Quarto project with all FileAttachments

USAGE:
    ohq2quarto [OPTIONS] --ohq-ref <OHQ_REF> --output-dir <OUTPUT_DIR>

OPTIONS:
        --echo                       turn cell echo on in the Quarto document (default is to not
                                     echo)
        --filename <FILENAME>        optional filename for the main Quarto document (will be taken
                                     from the slug in `ohq_ref`; e.g. "just-the-facts" from the
                                     example param)
    -h, --help                       Print help information
        --ohq-ref <OHQ_REF>          an Observable notebook short reference
                                     ("@hrbrmstr/just-the-facts") or a full URL
        --output-dir <OUTPUT_DIR>    directory to place Quarto project and files (will be created if
                                     it does not exist)
    -V, --version                    Print version information
        --verbose                    Print Notebook metadata during processing
```
