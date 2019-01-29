

import table from '../src/table'

const data = {
    'TypeScript': {
        code: 100,
        comment: 100,
        blank: 100,
        lines: 100,
        files: 100
    },
    'JavaScript': {
        code: 100,
        comment: 100,
        blank: 100,
        lines: 100,
        files: 100
    }
}

test('table -> length', () => {

    table(data).split('\n').forEach((line) => {
        expect(line.length).toBe(88)
    })

})

test('table -> content', () => {

    const result = table(data).split('\n')
    result.shift()
    result.shift()
    result.shift()
    result.pop()
    result.pop()
    result.pop()
    result.forEach((line) => {
        line.match(/\d+/g).forEach((n) => {
            expect(parseInt(n)).toBe(100)
        })
    })

})

test('table -> total', () => {

    const result = table(data).split('\n')
    result[6].match(/\d+/g).forEach((n) => {
        expect(parseInt(n)).toBe(200)
    })

})


