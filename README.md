

# rots

[![Build Status](https://img.shields.io/circleci/project/github/wyhaya/rots/master.svg?style=flat-square)](https://circleci.com/gh/wyhaya/rots) [![codecov](https://img.shields.io/codecov/c/github/wyhaya/rots.svg?style=flat-square)](https://codecov.io/github/wyhaya/rots) [![Download](https://img.shields.io/npm/dt/rots.svg?style=flat-square)](https://www.npmjs.com/package/rots) [![Version](https://img.shields.io/npm/v/rots.svg?style=flat-square)](https://www.npmjs.com/package/rots) [![License](https://img.shields.io/npm/l/rots.svg?style=flat-square)](./LICENSE)

`rots` is a command line tool that is used to quickly calculate the number of lines of various language codes in a project

![preview](https://user-images.githubusercontent.com/23690145/51724031-757fbe00-2096-11e9-960c-0172f2307802.png)

## Features

* Quickly calculate data
* Support multiple languages
* Beautiful user interface

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

```bash
# If you want to ignore files
rots --ignore node_modules dist
```

```bash
# If you want to count a language separately
rots --ext .ts .js
```

```bash
# If you want to output a table with three primary colors
rots --color
```

## Supported Languages

<table>
    <tr>
        <td><code>.css</code></td>
        <td><code>.scss</code></td>
        <td><code>.sass</code></td>
        <td><code>.html</code></td>
        <td><code>.js</code></td>
        <td><code>.jsx</code></td>
    </tr>
    <tr>
        <td><code>.json</code></td>
        <td><code>.php</code></td>
        <td><code>.rs</code></td>
        <td><code>.go</code></td>
        <td><code>.ts</code></td>
        <td><code>.tsx</code></td>
    </tr>
     </tr>
        <td><code>.md</code></td>
        <td><code>.py</code></td>
        <td><code>.sh</code></td>
        <td></td>
        <td></td>
        <td></td>
    </tr>
</table>
          

## License

[MIT](./LICENSE) LICENSE

