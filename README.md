![RAD logo](./logo.png)

# Rust Archive.org Downloader
> A toolkit for downloading and processing archive.org data

This project uses streaming to download and process archive.org data. It is easy
on the processor and memory, so you can run through enormous amounts of data
relatively quickly.

## Getting started

You can find the tools from `src/bin/` directory. Start by running:

```shell
cargo build --release
```

This will build the binary to `target/release/rad`. You can then run the
programs from there:

```shell
./target/release/rad
```

This will print out the help for the program.

## Features

This project is a personal toolkit that probably requires some Rust knowledge to
run. The best parts:
* It works
* It's fast (a few hours to parse all pastebin data)
* Parallel processing (did I say it's fast?)
* No error handling – catastrophic failures on network hiccups (good for people
  who like to live dangerously)
* Hopefully gets me some edge finding bug bounties

## Note about fair use of archive.org

Please check out the blog post from archive.org titled ["Let us serve you, but
don’t bring us down"][archive-org-blog-post].

TL;DR don't use multiple VPS instances, it will DDOS archive.org.

[archive-org-blog-post]: https://blog.archive.org/2023/05/29/let-us-serve-you-but-dont-bring-us-down/

## Contributing

If you'd like to contribute, please fork the repository and use a feature
branch. Pull requests are warmly welcome.

## Licensing

The code in this project is licensed under MIT license.

## Logo

Logo was made using [DeepFloyd IF](https://huggingface.co/spaces/DeepFloyd/IF), with the prompt:

```
Logo for a project called "RAD", pixel art, isometric, vibrant, archive, download, diskette, 8-bit retro 90-s design
```