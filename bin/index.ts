#!/usr/bin/env node

import tree, { Tree } from '../src/tree'
import parse, { Parse } from '../src/parse'
import argv from '../src/argv'
import color from '../src/color'
import table from '../src/table'
import config from '../src/config'


const argExt = argv('--ext')
const argIgnore = argv('--ignore')
const argColor = argv('--color')


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


const forEachFile = (files: Tree[]) =>  {

    for (let i = 0; i < files.length; i++) {

        const file = files[i]
       
        if (file.type === 'directory') {
            forEachFile(file.children)
            continue
        }
        
        const [langName, singleLine, multiLine] = config[file.extension]
        merge(langName, parse(file.path, singleLine, multiLine))

    }

}

// Which files are ignored
const ignore = Array.isArray(argIgnore) ? 
    new RegExp(argIgnore.join('|')) 
    :
    undefined

// Total available file extensions
const ext = Array.isArray(argExt) ?
    argExt.filter((ext) => Object.keys(config).includes(ext))
    :
    Object.keys(config)


forEachFile(tree(process.cwd(), {
    ignore, 
    ext
}))


argColor ?
    color(table(result))
    :
    console.log(table(result))

