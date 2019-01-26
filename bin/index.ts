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


const configExts = Object.keys(config)
const ext = Array.isArray(argExt) ?
    argExt.filter((ext) => configExts.includes(ext))
    :
    configExts

const forEachFile = (files: Tree[]) => new Promise(async (success) => {

    for (let i = 0; i < files.length; i++) {

        const file = files[i]
        if (file.type === 'directory') {
            await forEachFile(file.children)
            continue
        }

        const conf = config[file.extension]
        ext.includes(file.extension) && merge(conf[0], await parse(file.path, conf[1]))

    }

    success()

})


!(async () => {

    await forEachFile(await tree(process.cwd(), {
        filter: Array.isArray(argIgnore) ? new RegExp(argIgnore.join('|')) : undefined
    }))

    argColor ?
        color(table(result))
        :
        console.log(table(result))

})()


