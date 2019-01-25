#!/usr/bin/env node

import tree, { Tree } from '../src/tree'
import parse, { Parse } from '../src/parse'
import output from '../src/output'
import config from '../src/config'


interface Result extends Parse {
    files: number
}

const result: {
    [key: string]: Result
} = { }


const merge = (lang: string, { code, comment, blank, lines }: Parse) => {
    if(result[lang] === undefined) {
        result[lang] = {
            code: 0,
            comment: 0,
            blank: 0,
            lines: 0,
            files: 0
        }
    }
    result[lang].code += code
    result[lang].comment += comment
    result[lang].blank += blank
    result[lang].lines += lines
    result[lang].files += 1
}


const forEachFile = (files: Tree[]): void => {
    files.forEach((file) => {
        if(file.type === 'file') {
            const conf = config[file.extension]
            conf && merge(conf[0], parse(file.path, conf[1]))
        }else {
            forEachFile(file.children)
        }
    })
}


forEachFile(tree(process.cwd(), {
    filter: /node_modules|\.git/
}))


output(result)


