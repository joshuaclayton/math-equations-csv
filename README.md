# Math Equation CSV Generator

Small CLI to generate a comma-delimited CSV of math equations.

Useful for online flashcard tools like [Quizlet](https://quizlet.com).

## Usage

```sh
$ math --operation division --operation multiplication --maximum 10                                      ruby 2.7.1p83

...snip
6x8,48
54÷9,6
6x9,54
60÷10,6
6x10,60
7x0,0
7÷1,7
7x1,7
14÷2,7
7x2,14
21÷3,7
7x3,21
28÷4,7
7x4,28
35÷5,7
7x5,35
42÷6,7
7x6,42
49÷7,7
7x7,49
56÷8,7
7x8,56
63÷9,7
...snip
90÷9,10
10x9,90
100÷10,10
10x10,100
```

See `--help` for full usage.

## Download

Downloads of the CLI are available on the [releases] page.

[releases]: https://github.com/joshuaclayton/math-equations-csv/releases

To install, download the math-VERSION gzipped directory, then move the
generated `math` binary to somewhere in your `$PATH`.

## License

Copyright 2020 Josh Clayton. See the [LICENSE](LICENSE).
