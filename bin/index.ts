#!/usr/bin/env node

import * as fs from 'fs'
import tree, { TreeOutput } from '../src/tree'
import typescript from '../parse/typescript'
import json from '../parse/json'
import other from '../parse/other'

const getFiles = (fileTree: TreeOutput[]): TreeOutput[] => {
    let files = []
    fileTree.forEach((item) => {
        if(item.type === 'file') {
            files.push(item)
        }else {
            files = files.concat(getFiles(item.children))
        }
    })
    return files
}

const result: any = {}

const files = getFiles(tree(process.cwd(), {
    filter: /node_modules|\.git/
}))

const mergeOption = (lang: string, options: any) => {
    if(result[lang] === undefined) {
        result[lang] = {
            Code: 0,
            Comment: 0,
            Blank: 0,
            Lines: 0,
            Files: 0
        }
    }
    result[lang].Code = result[lang].Code + options.Code
    result[lang].Comment = result[lang].Comment + options.Comment
    result[lang].Blank = result[lang].Blank + options.Blank
    result[lang].Lines = result[lang].Lines + options.Lines
    result[lang].Files = result[lang].Files + 1
}

files.forEach((file) => {
    switch(file.extension) {
        case '.ts': {
            mergeOption('TypeScript', typescript(fs.readFileSync(file.path, 'utf8')))
            break
        }
        case '.json': {
            mergeOption('JSON', json(fs.readFileSync(file.path, 'utf8')))
            break
        }
        default: {
            mergeOption('Other', other(fs.readFileSync(file.path, 'utf8')))
            break
        }
    }
})

console.log(result)


