

# lok

[![Build Status](https://img.shields.io/travis/wyhaya/lok.svg?style=flat-square)](https://travis-ci.org/wyhaya/lok)
[![Crates.io](https://img.shields.io/crates/l/lok.svg?style=flat-square)](https://github.com/wyhaya/lok/blob/master/LICENSE)

`lok` is a command line tool, that is used to quickly calculate the number of lines of various language codes in a project

![preview](https://user-images.githubusercontent.com/23690145/51882818-3c5b8c80-23bb-11e9-8da6-5e7b19a7f536.png)


## Features

* Quickly calculate data
* Support multiple languages
* Beautiful user interface

## Install

```bash
cargo install lok
```

## Use

Go to your project in the terminal and type lok on the command line

```bash
cd your-project
lok
```

```bash
# If you want to ignore files
lok --ignore node_modules dist
```

```bash
# If you want to use regular expressions
lok --ignore 'node_\w+|dist'
```

```bash
# If you want to calculate some languages
lok --ext ts js
```


## Contributing

If you want to add statistics for other languages, please refer to [./src/main.rs](./src/main.rs)


Example:

```typescript
const config = {
    '.js': [
        'JavaScript', /^\s*\/\//, [/\/\*/, /\*\//]
    ]
    // ...
}
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
        <td><code>.swift</code></td>
        <td></td>
        <td></td>
    </tr>
</table>

## License

[MIT](./LICENSE) LICENSE

