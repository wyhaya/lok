

const max = 12

export default (data: any) => {

    const result = []

    for(let x in data) {
        data[x].language = x
        result.push(data[x])
    }

    const { code, comment, blank, lines, files } = result.reduce((prev, next) => {
        next = {...next}
        next.code += prev.code
        next.comment += prev.comment
        next.blank += prev.blank
        next.lines += prev.lines
        next.files += prev.files
        return next
    })

    console.log(`┌${'─'.padEnd(max * 6 + 2, '─')}┐`)
    console.log(`│ ${'Language'.padEnd(max, ' ')}${'Code'.padStart(max, ' ')}${'Comment'.padStart(max, ' ')}${'Blank'.padStart(max, ' ')}${'Lines'.padStart(max, ' ')}${'Files'.padStart(max, ' ')} │`)
    console.log(`├${'─'.padEnd(max * 6 + 2, '─')}┤`)
    result.forEach((item) => {
        const language = item.language.padEnd(max, ' ')
        const code = item.code.toString().padStart(max, ' ')
        const comment = item.comment.toString().padStart(max, ' ')
        const blank = item.blank.toString().padStart(max, ' ')
        const lines = item.lines.toString().padStart(max, ' ')
        const files = item.files.toString().padStart(max, ' ')
        console.log(`│ ${language}${code}${comment}${blank}${lines}${files} │`)
    })
    console.log(`├${'─'.padEnd(max * 6 + 2, '─')}┤`)
    console.log(`│ ${'Total'.padEnd(max, ' ')}${code.toString().padStart(max, ' ')}${comment.toString().padStart(max, ' ')}${blank.toString().padStart(max, ' ')}${lines.toString().padStart(max, ' ')}${files.toString().padStart(max, ' ')} │`)
    console.log(`└${'─'.padEnd(max * 6 + 2, '─')}┘`)
}


