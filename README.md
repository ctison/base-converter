# Base converter

Convert a number from any base to any other base from your terminal!

## Install

You can find binaries in [Releases](https://github.com/ctison/base/releases).
```
curl -Lo /usr/local/bin/base https://github.com/ctison/base/releases/latest/download/base-linux-amd64
chmod 500 /usr/local/bin/base
```

## Usage

```sh
$ base NUMBER FROM_BASE TO_BASE
```

## Examples

Convert 51966 from base 10 to base 16

```sh
$ base 51966 0123456789 0123456789ABCDEF
CAFE
```

Convert 42 from base 10 to base 2

```sh
$ base 42 0123456789 🦾🔥
🔥🦾🔥🦾🔥🦾
```
