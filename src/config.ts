

interface Config {
    [extension: string]: [string, RegExp?]
}

const config: Config = {
    '.css': ['CSS'],
    '.scss': ['CSS'],
    '.sass': ['CSS'],
    '.html': ['HTML'],
    '.js': ['JavaScript', /\s*\/\/.*/g],
    '.jsx': ['JavaScript JSX'],
    '.ts': ['TypeScript', /\s*\/\/.*/g],
    '.tsx': ['TypeScript JSX'],
    '.json': ['JSON'],
    '.md': ['MarkDown'],
    '.php': ['PHP'],
    '.vue': ['Vue'],
    '.yml': ['YML']
}

export default config

