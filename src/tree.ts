

import * as path from 'path'
import * as fs from 'fs'

interface Option {
    ignore?: RegExp,
    ext?: string[]
}

export interface Tree {
    name: string
    path: string,
    type: 'directory' | 'file'
    extension: string | null
    children: Tree[]
}


const tree = (dir: string, option?: Option): Tree[]  => {

    option = option || {}

    const files = fs.readdirSync(dir, 'utf8')
    const result: Tree[] = []

    for (let i = 0; i < files.length; i++) {

        const fileName = files[i]

        if (option.ignore && option.ignore.test(fileName)) {
            continue
        }

        const ext = path.extname(fileName)
        const fileStat = fs.statSync(`${dir}/${fileName}`)
        const isDirectory = fileStat.isDirectory()

        if(option.ext && !isDirectory && !option.ext.includes(ext)) {
            continue
        }

        result.push({
            name: fileName,
            path: path.resolve(dir, fileName),
            type: isDirectory ? 'directory' : 'file',
            extension: isDirectory ? null : ext,
            children: isDirectory ? tree(`${dir}/${fileName}`, option) : []
        })

    }
    
    return result

}



export default tree


