

import * as path from 'path'
import tree from '../src/tree'


test('tree -> generate', () => {

    const map = tree(path.resolve(__dirname, '../src'))
    expect(typeof map).toBe('object')

})


test('tree -> tree object', async () => {

    const map = await tree(path.resolve(__dirname, '../src'))
    expect(map.length).toBe(6)

    map.forEach((item) => {
        expect(item.type).toBe('file')
        expect(item.extension).toBe('.ts')
        expect(item.children).toEqual([])
        expect(typeof item.size).toBe('number')
    })

})


test('filter', async () => {

    const map = await tree(process.cwd(), {
        filter: /git|node_modules/
    })
    const length = map.filter(item => {
        return /git/.test(item.name)
    }).length
   

    expect(length).toBe(0)

})


