# diff.rs

A more beautiful and readable diff output for ...

## Screenshot

![diff.rs](http://via.placeholder.com/800x600)

## Getting Started

Install it in different ways:

```
cargo install diff.rs
```

or directly over github:

```
git clone https://github.com/miguelberrio91/diff.rs
cd diff.rs
make
sudo make install
```

## Usage

**Compare two files**
```
───┬─────────────────────────────────────────────────────────────────
   │ M Cargo.toml @5ac01d12897f
───┼─────────────────────────────────────────────────────────────────
 1 │ #!/usr/bin/env bash
 2 │
 3 │-echo "Test"
 3 │+echo "Test is going on"
───┴─────────────────────────────────────────────────────────────────
```

**Git diff**
```
$ git diff | diff-rs -c
```

```
───┬─────────────────────────────────────────────────────────────────
   │ M Cargo.toml @5ac01d12897f
───┼─────────────────────────────────────────────────────────────────
 1 │ #!/usr/bin/env bash
 2 │
 3 │-echo "Test"
 3 │+echo "Test is going on"
───┴─────────────────────────────────────────────────────────────────
```

## Contributing

Feel free to open a pull request or only a issue to contribute to this project.

## Authors

* **Miguel Berrio** - *Initial work* - [gh/miguelberrio91](https://github.com/miguelberrio91)
* **Dimitrij Vogt** - *Initial work* - [gh/dvogt23](https://github.com/dvogt23)

See also the list of [contributors](https://github.com/miguelberrio91/diff.rs/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
