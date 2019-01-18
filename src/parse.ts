

import * as fs from 'fs'

export interface Parse {
    code: number
    comment: number
    blank: number
    lines: number
}

export default (filePath: string, commentReg?: RegExp): Parse => {

    const content = fs.readFileSync(filePath, 'utf8')

    let blank = (content.match(/\n\s*\n/g) || []).length

    let comment = 0
    if(commentReg !== undefined) {
        comment = (content.match(commentReg) || []).length
    }

    return {
        code: 0,
        comment,
        blank,
        lines: content.split('\n').length
    }

}
