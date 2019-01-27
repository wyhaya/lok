

import parse from '../src/parse'
import config from '../src/config'

test('psrse -> JavaScript', () => {

    const JavaScript = parse('test/__code/main.js', config['.js'][1], config['.js'][2])

    expect(JavaScript.lines).toBe(12)
    expect(JavaScript.blank).toBe(6)
    expect(JavaScript.comment).toBe(4)
    expect(JavaScript.code).toBe(2)

})

test('psrse -> TypeScript', () => {

    const TypeScript = parse('test/__code/main.ts', config['.ts'][1], config['.ts'][2])

    expect(TypeScript.lines).toBe(12)
    expect(TypeScript.blank).toBe(6)
    expect(TypeScript.comment).toBe(4)
    expect(TypeScript.code).toBe(2)

})


// ...


