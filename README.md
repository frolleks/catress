# catress

A simple command line tool to read files.

## Usage

To read a file simply run:

```sh
catress file.txt
```

And, to read multiple files simply run:

```sh
catress file1.txt file2.txt
```

## Benchmark

Tested on:
- Intel Core i5-11400F (6C/12T)
- 16GB RAM

| command                  | time        |
| ------------------------ | ----------- |
| `cat test/test.json`     | 24.226s     |
| `catress test/test.json` | **21.684s** |

catress won the benchmark by 3 seconds.