

import * as fs from 'fs'

export interface Parse {
    code: number
    comment: number
    blank: number
    lines: number
}


export default (filePath: string, singleLine?: RegExp, multiLine?: [RegExp, RegExp]): Parse => {

    const content = fs.readFileSync(filePath, 'utf8').split('\n')

    let blank = 0
    let comment = 0
    let code = 0
    let isCommment = false

    content.forEach((l) => {

        if (/^\s*$/.test(l)) {
            blank += 1
            return
        }

        if(multiLine) {
            if (multiLine[0].test(l)) {
                isCommment = true
            }
           
            if (multiLine[1].test(l)) {
                isCommment = false
                comment += 1
                return
            }

            if (isCommment) {
                comment += 1
                return
            }
        }

        if (singleLine && singleLine.test(l)) {
            comment += 1
            return
        }

        code += 1

    })

    return {
        code,
        comment,
        blank,
        lines: content.length
    }

}

