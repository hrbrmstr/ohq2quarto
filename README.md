# ohq2quarto

Save an [Observable HQ](https://observablehq.com) Notebook to a [Quarto](https://quarto.org/) project.

Given an ObsHQ reference (full URL or `@user/slug`) and an output directory, this utility will make a Quarto project directory, build a `qmd` file and download all `FileAttachment`s.

Presently, this is a quick hack over lunch right after doing R and Golang work, so it needs some cleanup. Code context switching isn't fun.

```shell
cargo build
cargo run -- --ohq-ref @hrbrmstr/just-one-more-thing --output-dir /tmp/jomt
```

(More docs / pre-built binaries / `cargo install …` / etc. forthcoming.)