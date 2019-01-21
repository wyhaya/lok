

interface Config {
    [extension: string]: [string, RegExp?]
}

const config: Config = {
    '.css':  ['CSS'],
    '.scss': ['CSS'],
    '.sass': ['CSS'],
    '.html': ['HTML'],
    '.js':   ['JavaScript', /\/\*.+?\*\/|\/\/.*(?=[\n\r])/g],
    '.jsx':  ['JavaScript JSX'],
    '.ts':   ['TypeScript', /\/\*.+?\*\/|\/\/.*(?=[\n\r])/g],
    '.tsx':  ['TypeScript JSX'],
    '.json': ['JSON'],
    '.md':   ['MarkDown'],
    '.php':  ['PHP'],
    '.rs':   ['Rust'],
    '.go':   ['Go'],
    '.py':   ['Python'],
    '.sh':   ['Shell'],
    '.vue':  ['Vue'],
    '.yml':  ['YML']
}

export default config

