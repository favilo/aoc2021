# Advent of Code 2021

To run all days:

```sh
cargo run --release
```

Timings generated by:

The `cargo-criterion` crate is useful to get nice benchmarks.

```sh
cargo criterion
```

Though not required, this just doesn't have as nice output, and will deprecate plots soon:

```sh
cargo bench
```

## Timings

```
day01/get_input         time:   [30.165 us 30.207 us 30.255 us]
day01/part1             time:   [711.06 ns 712.68 ns 714.55 ns]
day01/part2             time:   [2.3112 us 2.3178 us 2.3255 us]

day02/get_input         time:   [42.592 us 42.651 us 42.718 us]
day02/part1             time:   [545.94 ns 548.03 ns 550.17 ns]
day02/part2             time:   [729.97 ns 731.44 ns 733.11 ns]

day03/get_input         time:   [23.079 us 23.122 us 23.171 us]
day03/part1             time:   [737.94 ns 739.46 ns 741.72 ns]
day03/part2             time:   [8.1633 us 8.1777 us 8.1946 us]

day04/get_input         time:   [74.034 us 74.188 us 74.374 us]
day04/part1             time:   [19.244 us 19.329 us 19.427 us]
day04/part2             time:   [90.903 us 91.201 us 91.569 us]

day05/get_input         time:   [875.77 us 879.41 us 883.73 us]
day05/part1             time:   [152.17 us 152.95 us 153.97 us]
day05/part2             time:   [206.94 us 216.63 us 227.91 us]

day06/get_input         time:   [1.8628 us 1.8722 us 1.8821 us]
day06/part1             time:   [6.4909 ns 6.5098 ns 6.5312 ns]
day06/part2             time:   [30.811 ns 30.918 ns 31.045 ns]

day07/get_input         time:   [4.3000 us 4.3099 us 4.3220 us]
day07/part1             time:   [114.43 ns 114.95 ns 115.53 ns]
day07/part2             time:   [673.07 ns 676.99 ns 680.80 ns]

day08/get_input         time:   [71.718 us 71.898 us 72.098 us]
day08/part1             time:   [518.46 ns 522.44 ns 528.77 ns]
day08/part2             time:   [65.264 us 65.405 us 65.563 us]

day09/get_input         time:   [150.64 us 150.76 us 150.87 us]
day09/part1             time:   [73.486 us 73.519 us 73.554 us]
day09/part2             time:   [311.25 us 315.14 us 319.01 us]

day10/get_input         time:   [313.38 us 313.80 us 314.33 us]
day10/part1             time:   [74.825 ns 74.911 ns 75.012 ns]
day10/part2             time:   [1.2887 us 1.2906 us 1.2923 us]

day11/get_input         time:   [1.5303 us 1.5320 us 1.5338 us]
day11/part1             time:   [657.77 us 658.31 us 658.91 us]
day11/part2             time:   [1.9361 ms 1.9375 ms 1.9391 ms]

day12/get_input         time:   [3.1992 us 3.2063 us 3.2196 us]
day12/part1             time:   [746.59 us 754.44 us 762.19 us]
day12/part2             time:   [16.694 ms 16.841 ms 16.988 ms]

day13/get_input         time:   [53.814 us 53.845 us 53.882 us]
day13/part1             time:   [1.4433 ms 1.4449 ms 1.4467 ms]
day13/part2             time:   [2.4746 ms 2.4772 ms 2.4797 ms]

day14/get_input         time:   [5.9087 us 5.9148 us 5.9221 us]
day14/part1             time:   [14.963 us 14.976 us 14.991 us]
day14/part2             time:   [59.822 us 59.862 us 59.911 us]

day15/get_input         time:   [141.84 us 141.97 us 142.13 us]
day15/part1             time:   [1.2577 ms 1.2588 ms 1.2601 ms]
day15/part2             time:   [67.768 ms 67.835 ms 67.913 ms]

day16/get_input         time:   [59.267 us 59.344 us 59.421 us]
day16/part1             time:   [314.11 ns 314.28 ns 314.48 ns]
day16/part2             time:   [726.71 ns 728.15 ns 729.45 ns]

day17/get_input         time:   [205.54 ns 205.75 ns 206.00 ns]
day17/part1             time:   [5.2076 ms 5.2104 ms 5.2139 ms]
day17/part2             time:   [5.3259 ms 5.3324 ms 5.3415 ms]

```

## Failed experiments

### Day 6

I tried to be more clever than the optimizer, but failed. If you use add
`--features day06_ring` you can see the awful timings I got when attempting to
use a ring buffer to update in place. It _works_, but it ended up being about 3000% slower

```
day06/get_input         time:   [4.2848 us 4.3215 us 4.3654 us]
                        change: [-3.9853% -2.3302% -0.6116%] (p = 0.01 < 0.05)
                        Change within noise threshold.
day06/part1             time:   [262.28 ns 266.21 ns 270.56 ns]
                        change: [+3033.7% +3134.4% +3236.0%] (p = 0.00 < 0.05)
                        Performance has regressed.
day06/part2             time:   [836.93 ns 852.07 ns 869.67 ns]
                        change: [+1851.4% +1892.5% +1933.2%] (p = 0.00 < 0.05)
                        Performance has regressed.

```
