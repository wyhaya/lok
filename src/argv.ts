


const argv = process.argv.slice(2, process.argv.length)

export default (query: string): boolean | string[] => {

    const index = argv.findIndex((arg) => {
        return arg === query
    })

    if(index === -1) {
        return false
    }

    let result: string[] = []

    for(let i = index + 1; i < argv.length; i++) {
        if(argv[i].indexOf('--') === -1) {
            result.push(argv[i])
        }else {
            break
        }
    }

    return result.length ? result : true
    
}


