#!/usr/bin/env node

import tree, { Tree } from '../src/tree'
import parse, { Parse } from '../src/parse'
import color from '../src/color'
import table from '../src/table'
import config from '../src/config'

const argv = process.argv.slice(2, process.argv.length)

const getArg =  (query: string): boolean | string[] => {

    const index = argv.findIndex((arg) => {
        return arg === query
    })

    if(index === -1) {
        return false
    }

    let result: string[] = []

    for(let i = index + 1; i < argv.length; i++) {
        if(argv[i].indexOf('--') === -1) {
            result.push(argv[i])
        }else {
            break
        }
    }

    return result.length ? result : true
    
}

const argExt = getArg('--ext')
const argIgnore = getArg('--ignore')
const argColor = getArg('--color')

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

