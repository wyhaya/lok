

import { Parse } from './parse'
import outputColor from './color'

interface Result extends Parse {
    files: number
    language?: string
}

const max = 14

function fill(content: string | number, isPadEnd?: boolean): string {
    return isPadEnd === true ?
        content.toString().padEnd(max, ' ')
        :
        content.toString().padStart(max, ' ')
}

function fillRow(left: string, right: string): string {
    return left + '─'.padEnd(max * 6 + 2, '─') + right
}


export default (data: { [key: string]: Result }): void => {

    const result: Result[] = []

    for (let x in data) {
        data[x].language = x
        result.push(data[x])
    }

    result.sort((x, y) => {
        if(x.language === 'Other') {
            return 1
        }
        return x.language.charAt(0) > y.language.charAt(0) ? 1 : -1
    })

    outputColor(fillRow('┌', '┐'))
    outputColor(
        '│ ' + 
        fill('Language', true) + 
        fill('Code') + 
        fill('Comment') + 
        fill('Blank') + 
        fill('Lines') + 
        fill('Files') + 
        ' │'
    )
    outputColor(fillRow('├', '┤'))

    result.forEach((item) => {
        outputColor(
            '│ ' +
            fill(item.language, true) +
            fill(item.code) +
            fill(item.comment) +
            fill(item.blank) +
            fill(item.lines) +
            fill(item.files) +
            ' │'
        )
    })

    const total = result.reduce((prev, next) => {
        next = { ...next }
        next.code += prev.code
        next.comment += prev.comment
        next.blank += prev.blank
        next.lines += prev.lines
        next.files += prev.files
        return next
    })

    outputColor(fillRow('├', '┤'))
    outputColor(
        '│ ' + 
        fill('Total', true) + 
        fill(total.code) + 
        fill(total.comment) + 
        fill(total.blank) + 
        fill(total.lines) + 
        fill(total.files) + 
        ' │'
    )
    outputColor(fillRow('└', '┘'))

}


