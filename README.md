# ReiX

ReiX is a simple and lightweight hex editor written in Rust.

---

## ReiX_cli

ReiX_cli is the command line version of ReiX, providing simple hex binary viewing functionality.

### Usage

```shell
ReiX_cli [options] <file> [start:end | pos]
```

### Options

- `-h`: Show help message.
- `-n`: Don't display line numbers.
- `-c`: Don't display ASCII characters.
- `-i`: Display hex bytes in one line (will set -n and -c to true).
- `-v`: Show version information.

### Arguments

- `<file>`: The path to the file to be viewed.
- `[start:end]`: The range of bytes to be displayed, end is exclusive.
  > Both `start` and `end` can be empty or negative.
    > - If `start` is empty, it will be set to 0.
    > - If `end` is empty, it will be set to the end of the file.
    > - If `start` or `end` is negative, it will be counted from the end of the file.
- `[pos]`: The position of the byte to be displayed.