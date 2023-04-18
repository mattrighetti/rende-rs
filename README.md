# rende-rs

**This is a work-in-progress**

I use this tool to generate metatag images for my website. In particular I make use of the `sample.svg` file that you can find in the examples folder.
`rende-rs` uses Liquid to generate the svg xml code and then it outputs the svg as png.

### How to use
```sh
$ cargo run -- <input.svg> <output.png> <key> <value> ...
```

For example, running
```
$ cargo run -- examples/sample.svg examples/out.png \
    "website" "mattrighetti.com" \
    "date" "Mar, 16" \
    "title" "asciidoc,\nliquid and jekyll" \
    "description" "A couple of days ago I was giving a little update\nto my website and I needed a way to create and inject\ncustom HTML into some of my posts"
```

will generate

![img](https://github.com/mattrighetti/rende-rs/blob/master/examples/out.png)