

import { Parse } from './parse'
import outputColor from './color'

const max = 14

interface Result extends Parse {
    files: number
    language?: string
}

export default (data: {[key: string]: Result}): void => {

    const result: Result[] = []

    for(let x in data) {
        data[x].language = x
        result.push(data[x])
    }

    result.sort((x, y) => x.language.charAt(0) > y.language.charAt(0) ? 1 : -1)

    const header = [
        'Language'.padEnd(max, ' '),
        'Code'.padStart(max, ' '),
        'Comment'.padStart(max, ' '),
        'Blank'.padStart(max, ' '),
        'Lines'.padStart(max, ' '),
        'Files'.padStart(max, ' ')
    ]

    outputColor(`┌${'─'.padEnd(max * 6 + 2, '─')}┐`)
    outputColor(`│ ${header[0]}${header[1]}${header[2]}${header[3]}${header[4]}${header[5]} │`)
    outputColor(`├${'─'.padEnd(max * 6 + 2, '─')}┤`)

    result.forEach((item) => {
        outputColor(
            '│ ' +
            item.language.padEnd(max, ' ') +
            item.code.toString().padStart(max, ' ') +
            item.comment.toString().padStart(max, ' ') +
            item.blank.toString().padStart(max, ' ') +
            item.lines.toString().padStart(max, ' ') +
            item.files.toString().padStart(max, ' ') +
            ' │'
        )
    })
    
    const total = result.reduce((prev, next) => {
        next = {...next}
        next.code += prev.code
        next.comment += prev.comment
        next.blank += prev.blank
        next.lines += prev.lines
        next.files += prev.files
        return next
    })

    const folter = [
        'Total'.padEnd(max, ' '),
        total.code.toString().padStart(max, ' '),
        total.comment.toString().padStart(max, ' '),
        total.blank.toString().padStart(max, ' '),
        total.lines.toString().padStart(max, ' '),
        total.files.toString().padStart(max, ' ')
    ]

    outputColor(`├${'─'.padEnd(max * 6 + 2, '─')}┤`)
    outputColor(`│ ${folter[0]}${folter[1]}${folter[2]}${folter[3]}${folter[4]}${folter[5]} │`)
    outputColor(`└${'─'.padEnd(max * 6 + 2, '─')}┘`)

}


