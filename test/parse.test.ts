

import parse, { Parse } from '../src/parse'
import config from '../src/config'
import * as path from 'path'

const getParse = (filePath: string): Parse => {
    const ext = path.extname(filePath)
    return parse(filePath, config[ext][1], config[ext][2])
}

test('psrse -> JavaScript', () => {
    const JavaScript = getParse('test/__code/main.js')
    expect(JavaScript.lines).toBe(12)
    expect(JavaScript.blank).toBe(6)
    expect(JavaScript.comment).toBe(4)
    expect(JavaScript.code).toBe(2)
})

test('psrse -> TypeScript', () => {
    const TypeScript = getParse('test/__code/main.ts')
    expect(TypeScript.lines).toBe(12)
    expect(TypeScript.blank).toBe(6)
    expect(TypeScript.comment).toBe(4)
    expect(TypeScript.code).toBe(2)
})


// ...


