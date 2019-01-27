

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

const tree = (dir: string, option?: Option): Tree[]  => {

    option = option || {}

    const files = fs.readdirSync(dir, 'utf8')
    const result: Tree[] = []

    for (let i = 0; i < files.length; i++) {

        const fileName = files[i]

        if (isReg(option.filter) && option.filter.test(fileName)) {
            continue
        }
        
        const fileStat = fs.statSync(`${dir}/${fileName}`)
        const isDirectory = fileStat.isDirectory()

        result.push({
            name: fileName,
            path: path.resolve(dir, fileName),
            type: isDirectory ? 'directory' : 'file',
            size: fileStat.size,
            extension: isDirectory ? null : path.extname(fileName),
            children: isDirectory ? tree(`${dir}/${fileName}`, option) : []
        })

    }
    
    return result

}



export default tree


