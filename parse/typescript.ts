

interface Output {
    Code: number
    Comment: number
    Blank: number
    Lines: number
}

export default (content: string): Output => {

    return {
        Code: 0,
        Comment: 0,
        Blank: 0,
        Lines: content.split('\n').length
    }

}
