

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


async function readdir(dir: string): Promise<string[]> {
    return new Promise((success) => {
        fs.readdir(dir, 'utf8', (err, files) => {
            success(files)
        })
    })
}

async function stat(filePath: string): Promise<fs.Stats> {
    return new Promise((success) => {
        fs.stat(filePath, (err, stats) => {
            success(stats)
        })
    })
}

const isReg = (reg) => {
    return typeof reg === 'object' && reg.constructor == RegExp
}

const tree = (dir: string, option?: Option): Promise<Tree[]> => new Promise(async (success) => {

    option = option || {}

    const files = await readdir(dir)
    const result: Tree[] = []

    for (let i = 0; i < files.length; i++) {

        const fileName = files[i]
        const fileStat = await stat(`${dir}/${fileName}`)
        const isDirectory = fileStat.isDirectory()

        if (isReg(option.filter) && option.filter.test(fileName)) {
            continue
        }

        result.push({
            name: fileName,
            path: path.resolve(dir, fileName),
            type: isDirectory ? 'directory' : 'file',
            size: fileStat.size,
            extension: isDirectory ? null : path.extname(fileName),
            children: isDirectory ? await tree(`${dir}/${fileName}`, option) : []
        })

    }

    success(result)
})



export default tree


