# diff-rs

A more beautiful and readable diff output.
[![Build Status](https://travis-ci.org/B3RR10/diff-rs.svg?branch=develop)](https://travis-ci.org/B3RR10/diff-rs)

##### Screenshot
![Screenshot][screenshot]

[screenshot]: screenshot.png "Screenshot"

## Getting Started

Install it in different ways:

**actually not availible**
```
cargo install diff-rs
```

or directly over github:

```
git clone https://github.com/B3RR10/diff-rs
cd diff-rs
make install
```

## Usage

**Git diff**

```
$ git diff | diff-rs
```

or add it to your `.gitconfig`:

```
git config --global core.pager Path/to/diff-rs
```

## Contributing

Feel free to open a pull request or only a issue to contribute to this project.

## Authors

* **Miguel Berrio** - *Initial work* - [@B3RR10](https://github.com/B3RR10)
* **Dimitrij Vogt** - *Initial work* - [@dvogt23](https://github.com/dvogt23)

See also the list of [contributors](https://github.com/B3RR10/diff-rs/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
