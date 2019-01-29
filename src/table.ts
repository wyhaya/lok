

import { Parse } from './parse'

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


export default (data: { [key: string]: Result }): string => {

    const result: Result[] = []
    const content = []

    for (let x in data) {
        data[x].language = x
        result.push(data[x])
    }

    result.sort((x, y) => {
        return x.language.charAt(0) > y.language.charAt(0) ? 1 : -1
    })

    // Header
    content.push(fillRow('┌', '┐'))
    content.push(
        '│ ' +
        fill('Language', true) +
        fill('Code') +
        fill('Comment') +
        fill('Blank') +
        fill('Lines') +
        fill('Files') +
        ' │'
    )
    content.push(fillRow('├', '┤'))

    // Content
    result.forEach((item) => {
        content.push(
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

    const total = result.length > 0 ? result.reduce((prev, next) => {
        next = { ...next }
        next.code += prev.code
        next.comment += prev.comment
        next.blank += prev.blank
        next.lines += prev.lines
        next.files += prev.files
        return next
    }) : {
            code: 0,
            comment: 0,
            blank: 0,
            lines: 0,
            files: 0
        }

    // Footer
    content.push(fillRow('├', '┤'))
    content.push(
        '│ ' +
        fill('Total', true) +
        fill(total.code) +
        fill(total.comment) +
        fill(total.blank) +
        fill(total.lines) +
        fill(total.files) +
        ' │'
    )
    content.push(fillRow('└', '┘'))

    return content.join('\n')

}


