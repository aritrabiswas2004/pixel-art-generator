# Application CLI

> [!WARNING]
> This documentation page is incomplete and may not be up to date as you read it.

You can run the CLI as of version `0.1.0` only via development configuration involving
`cargo`.

Run the program with

```shell
cargo run -- path/to/image.jpg
```

to directly process any image.

### Options

- `-v` / `--verbose` : For more verbose output of the processing pipeline
- `-V` / `--version` : Prints version
- `-d` / `--downscale` : Manually set the downscaling height in pixels (default is 128)
- `-i` / `--imagepath` : Taken directly as the option with no preceding flag but can be added as well.
Argument is the path to the image file.
