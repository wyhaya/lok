

import * as child from 'child_process'
import * as path from 'path'

const base = path.resolve('./node_modules/.bin/ts-node') + ' ' + path.resolve('./bin/index.ts')

const config: {
    encoding: 'utf8'
    cwd: string
} =  {
    encoding: 'utf8',
    cwd: path.resolve(process.cwd(), './test')
}

test('bin -> test', () => {
    child.execSync(base, config)
    child.execSync(`${base} --color`, config)
    expect(0).toBe(0)
})

test('bin -> ignore', () => {
    const stdout = child.execSync(`${base} --ignore __code`, config)
    expect(stdout.split('\n').length).toBe(8)
})


test('bin -> ext', () => {
    const stdout = child.execSync(`${base} --ext .css`, config)
    expect(stdout.split('\n').length).toBe(8)
})

