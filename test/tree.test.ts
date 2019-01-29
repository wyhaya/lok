

import tree from '../src/tree'

test('tree -> generate', () => {

    const map = tree('./src')
    expect(Array.isArray(map)).toBeTruthy()

})

test('tree -> tree object', () => {

    const map = tree('./src')
    expect(map.length).toBe(6)

    map.forEach((item) => {
        expect(item.type).toBe('file')
        expect(item.extension).toBe('.ts')
        expect(item.children).toEqual([])
        expect(typeof item.name).toBe('string')
        expect(typeof item.path).toBe('string')
    })

})

test('tree -> ignore', () => {

    const map = tree(process.cwd(), {
        ignore: /git|node_modules/
    })
    const length = map.filter(item => {
        return /git|node_modules/.test(item.name)
    }).length
    expect(length).toBe(0)

})

test('tree -> ext', () => {

    const js = tree('./src', {
        ext: ['.js']
    }).length
    expect(js).toBe(0)

    const ts = tree('./src', {
        ext: ['.ts']
    }).map((file) => {
        expect(file.extension).toBe('.ts')
        return file
    })
    expect(ts.length).toBe(6)

})

