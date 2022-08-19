# ohq2quarto

Save an [Observable HQ](https://observablehq.com) Notebook to a [Quarto](https://quarto.org/) project.

Given an ObsHQ reference (full URL or `@user/slug`) and an output directory, this utility will make a Quarto project directory, build a `qmd` file and download all `FileAttachment`s.

Presently, this is a quick hack over lunch right after doing R and Golang work, so it needs some cleanup. Code context switching isn't fun.

```shell
cargo build # build it after cloning

cargo run -- --ohq-ref @hrbrmstr/just-one-more-thing --output-dir ./examples # run it after cloning

cargo install --git https://github.com/hrbrmstr/ohq2quarto # install it (~/.cargo/bin/ohq2quarto)
```

```shell
cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/ohq2quarto --help`
ohq2quarto 0.1.0
boB Rudis (@hrbrmstr)
Given an Observable Notebook reference, create a Quarto project with all FileAttachments

USAGE:
    ohq2quarto [OPTIONS] --ohq-ref <OHQ_REF> --output-dir <OUTPUT_DIR>

OPTIONS:
        --echo <echo>                turn cell echo or off in the Quarto document (default is to not
                                     echo) [default: false]
        --filename <FILENAME>        optional filename for the main Quarto document (will be taken
                                     from the slug in `ohq_ref`; e.g. "just-the-facts" from the
                                     example param)
    -h, --help                       Print help information
        --ohq-ref <OHQ_REF>          an Observable notebook short reference
                                     ("@hrbrmstr/just-the-facts") or a full URL
        --output-dir <OUTPUT_DIR>    directory to place Quarto project and files (will be created if
                                     it does not exist)
    -V, --version                    Print version information
```

(More docs / pre-built binaries / `cargo install …` / etc. forthcoming.)