

interface Config {
    [extension: string]: [string, RegExp?]
}

const config: Config = {
    '.css': ['CSS'],
    '.scss': ['CSS'],
    '.sass': ['CSS'],
    '.html': ['HTML'],
    '.js': ['JavaScript', /(?:^|\n|\r)\/\/.*(?:\r|\n|$)/g],
    '.jsx': ['JavaScript JSX'],
    '.ts': ['TypeScript', /(?:^|\n|\r)\/\/.*(?:\r|\n|$)/g],  // /(?:^|\n|\r)\s*\/\*[\s\S]*?\*\/\s*(?:\r|\n|$)/g
    '.tsx': ['TypeScript JSX'],
    '.json': ['JSON'],
    '.md': ['MarkDown'],
    '.php': ['PHP'],
    '.vue': ['Vue'],
    '.yml': ['YML']
}

export default config

