# Beltex Combinations

I wrote this code as I was playing [Beltex](https://store.steampowered.com/app/2051420/Beltex/) in order to find the right combinations to get to a number.

## Usage

```shell
cargo build --release
./target/release/beltex [--base=<base>] <target>
```

For more information :
```shell
./target/release/beltex --help
```

## Problem

The problem was to get to a number from a limited set of a numbers and some operations.

The available operations are addition, substraction and multiplication.

For example, if your base set of a number is `[1, 2, 3, 4, 5, 6]`, if you want to get to `26` in the least amount of steps, you can do :
- 1 + 5 * 5
- 2 + 4 * 6
- 5 * 6 - 4
- 6 + 4 * 5

## Algorithm

The algorithm I made up works in iterations.

Each iteration tries every new combinations available of numbers and store their result along with their complexity (i.e. the number of steps to get to it) into two HashMap.

So if you want to get to a number, you can just iterate until a combination has been found for the number.

## Acknowledgement

Big thanks to @abelianvariety for the code review.
