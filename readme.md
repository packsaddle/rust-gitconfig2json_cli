# gitconfig2json_cli

[![crates version][crates-image]][crates-url] [![Travis-CI Status][travis-image]][travis-url] [![Appveyor Status][appveyor-image]][appveyor-url] ![license][license-image]

> Parse `git config --list --null` for cli

See [gitconfig2json](https://github.com/packsaddle/rust-gitconfig2json) for the programmatic API.

## Example

```
$ git config --list --null | gitconfig2json | jq
{
  "branch": {
    "chore/foo=bar.baz": {
      "merge": "refs/heads/master",
      "remote": "upstream"
    },
    "master": {
      "merge": "refs/heads/master",
      "remote": "upstream"
    }
  },
  "color": {
    "ui": "auto"
  },
  "core": {
    "bare": "false",
    "editor": "vim",
    "excludesfile": "~/data/src/github.com/sanemat/dotfiles2016/gitignore-system",
    "filemode": "false",
    "logallrefupdates": "true",
    "repositoryformatversion": "0"
  },
  "remote": {
    "upstream": {
      "fetch": "+refs/heads/*:refs/remotes/upstream/*",
      "url": "git@github.com:packsaddle/rust-gitconfig2json_cli.git"
    }
  },
  "url": {
    "git@gist.github.com:": {
      "pushinsteadof": "https://gist.github.com//"
    },
    "git@github.com:": {
      "pushinsteadof": "https://github.com/"
    }
  }
}
```


## Install

Download from [Latest release](https://github.com/packsaddle/rust-gitconfig2json_cli/releases/latest) for your own environment.

or

```
$ cargo install gitconfig2json_cli
```

## changelog

[changelog](./changelog.md)

## License

MIT/Apache-2.0 © [Sanemat](http://sane.jp)

[travis-url]: https://travis-ci.org/packsaddle/rust-gitconfig2json_cli
[travis-image]: https://img.shields.io/travis/packsaddle/rust-gitconfig2json_cli/master.svg?style=flat-square&label=travis
[appveyor-url]: https://ci.appveyor.com/project/sanemat/rust-gitconfig2json-cli/branch/master
[appveyor-image]: https://img.shields.io/appveyor/ci/sanemat/rust-gitconfig2json-cli/master.svg?style=flat-square&label=appveyor
[crates-url]: https://crates.io/crates/gitconfig2json_cli
[crates-image]: https://img.shields.io/crates/v/gitconfig2json_cli.svg?style=flat-square
[license-image]: https://img.shields.io/crates/l/gitconfig2json_cli.svg?style=flat-square
