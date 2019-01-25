

import * as path from 'path'
import * as fs from 'fs'

interface Option {
    filter?: RegExp
}

export interface Tree {
    name: string
    path: string,
    type: 'directory' | 'file'
    size: number
    extension: string | null
    children: Tree[]
}

const isReg = (reg) => {
    return typeof reg === 'object' && reg.constructor == RegExp
}

function tree(dir: string, option?: Option): Tree[] {

    option = option || {}

    const files = fs.readdirSync(dir, 'utf8')

    return files.map((fileName): any => {

        const fileStat = fs.statSync(`${dir}/${fileName}`)
        const isDirectory = fileStat.isDirectory()

        if (option.filter && isReg(option.filter)) {
            if (option.filter.test(fileName) === true) {
                return false
            }
        }

        return {
            name: fileName,
            path: path.resolve(dir, fileName),
            type: isDirectory ? 'directory' : 'file',
            size: fileStat.size,
            extension: isDirectory ? null : path.extname(fileName),
            children: isDirectory ? tree(`${dir}/${fileName}`, option) : []
        }

    }).filter(Boolean)

}

export default tree


