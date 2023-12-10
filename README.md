# Base converter

Convert a number from any base to any other base from your terminal!

## Install

Pre-built binaries are available in [Releases](https://github.com/ctison/base-converter/releases).

NPM package (wasm) available at https://www.npmjs.com/package/@ctison/base-converter.

## CLI Usage

```sh
$ base-converter <NUMBER> <FROM_BASE> <TO_BASE>
```

```sh
# Convert 51966 from base 10 to base 16
$ base-converter 51966 0123456789 0123456789ABCDEF
CAFE
```

```sh
# Convert 42 from base 10 to base 2
$ base-converter 42 0123456789 🦀🚀
🚀🦀🚀🦀🚀🦀
```

## NPM usage

```sh
$ npm add @ctison/base-converter
```

```ts
/**
 * Checks if a base is valid by throwing an error if not.
 * @param {string} base
 */
export function checkBase(base: string): void
/**
 * Convert a number from any base to an decimal number.
 * @param {string} nbr
 * @param {string} fromBase
 * @returns {number}
 */
export function baseToDecimal(nbr: string, fromBase: string): number
/**
 * Convert an decimal number to any base.
 * @param {number} nbr
 * @param {string} toBase
 * @returns {string}
 */
export function decimal_to_base(nbr: number, toBase: string): string
/**
 * Convert a number from any base to any base.
 * @param {string} nbr
 * @param {string} fromBase
 * @param {string} toBase
 * @returns {string}
 */
export function baseToBase(
  nbr: string,
  fromBase: string,
  toBase: string
): string
```
