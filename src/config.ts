

interface Config {
    [extension: string]: [string, RegExp?, [RegExp, RegExp]?]
}

// Language name
// Single line comment verification
// Verification of multi-line comments

const config: Config = {
    '.css': [
        'CSS', void 0, [/\/\*/, /\*\//]
    ],
    '.scss': [
        'CSS', /^\s*\/\//, [/\/\*/, /\*\//]
    ],
    '.sass': [
        'CSS', /^\s*\/\//, [/\/\*/, /\*\//]
    ],
    '.html': [
        'HTML', void 0, [/<!--/, /-->/]
    ],
    '.js': [
        'JavaScript', /^\s*\/\//, [/\/\*/, /\*\//]
    ],
    '.jsx': [
        'JavaScript JSX', /^\s*\/\//, [/{?\/\*/, /\*\/}?/]
    ],
    '.ts': [
        'TypeScript', /^\s*\/\//, [/\/\*/, /\*\//]
    ],
    '.tsx': [
        'TypeScript JSX', /^\s*\/\//, [/{?\/\*/, /\*\/}?/]
    ],
    '.json': [
        'JSON'
    ],
    '.md': [
        'MarkDown'
    ],
    '.php': [
        'PHP', /^\s*\/\/|#/, [/\/\*/, /\*\//]
    ],
    '.rs': [
        'Rust', /^\s*\/\//, [/\/\*/, /\*\//]
    ],
    '.go': [
        'Go', /^\s*\/\//, [/\/\*/, /\*\//]
    ],
    '.py': [
        'Python', /^\s*#/
    ],
    '.sh': [
        'Shell', /^\s*#/
    ],
    '.yml': [
        'YML', /^\s*#/
    ],
    '.swift': [
        'Swift', /^\s*\/\//, [/\/\*/, /\*\//]
    ],

    // undone ...
    '.c': [
        'C'
    ],
    '.coffee': [
        'CoffeeScript'
    ],
    '.dart': [
        'Dart'
    ],
    '.docker': [
        'Docker'
    ],
    '.java': [
        'Java'
    ],
    '.less': [
        'Less'
    ],
    '.lua': [
        'Lua'
    ],
    '.m': [
        'ObjectiveC'
    ],
    '.aspx': [
        'AspNet'
    ],
    '.makefile': [
        'Makefile'
    ],
    '.sc': [
        'Scala'
    ],
    '.sql': [
        'Sql'
    ],
    '.styl': [
        'Stylus'
    ],
    '.vim': [
        'VimScript'
    ],
    '.xml': [
        'XML'
    ]
}

export default config

