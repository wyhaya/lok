

import * as fs from 'fs'

export interface Parse {
    code: number
    comment: number
    blank: number
    lines: number
}


async function readFile(filePath: string): Promise<string> {
    return new Promise((success) => {
        fs.readFile(filePath, 'utf8', (err, data) => {
            success(data)
        })
    })
}


export default (filePath: string, commentReg?: RegExp): Promise<Parse> => new Promise(async (success) => {

    const content = await readFile(filePath)

    let blank = (content.match(/(^[\t|\s]*\n)/gm) || []).length

    let comment = 0
    if (commentReg) {
        comment = (content.match(commentReg) || []).length
    }

    success({
        code: 0,
        comment,
        blank,
        lines: content.split('\n').length
    })

})

