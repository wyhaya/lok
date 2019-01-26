

# rots

[![Build Status](https://img.shields.io/circleci/project/github/wyhaya/rots/master.svg?style=flat-square)](https://circleci.com/gh/wyhaya/rots) [![codecov](https://img.shields.io/codecov/c/github/wyhaya/rots.svg?style=flat-square)](https://codecov.io/github/wyhaya/rots) [![Download](https://img.shields.io/npm/dt/rots.svg?style=flat-square)](https://www.npmjs.com/package/rots) [![Version](https://img.shields.io/npm/v/rots.svg?style=flat-square)](https://www.npmjs.com/package/rots) [![License](https://img.shields.io/npm/l/rots.svg?style=flat-square)](./LICENSE)

`rots` is a command line tool that is used to quickly calculate the number of lines of various language codes in a project

![preview](https://user-images.githubusercontent.com/23690145/51724031-757fbe00-2096-11e9-960c-0172f2307802.png)

## Features

* Fast calculation
* Support multiple languages
* Very beautiful user interface

## Install

```shell
yarn global add rots
```

## Use

Go to your project in the terminal and type rots on the command line

```bash
cd your-project
rots
```

If you want to ignore files

```bash
rots --ignore node_modules dist
```

If you want to count a language separately

```bash
rots --ext .ts .js
```

If you want to output a table with three primary colors

```bash
rots --color
```

## License

[MIT](./LICENSE) LICENSE

