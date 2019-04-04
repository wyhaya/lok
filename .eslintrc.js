

module.exports = {
    env: {
        node: true,
        commonjs: true,
        es6: true,
        jest: true
    },
    parser: '@typescript-eslint/parser',
    extends: [
        'eslint:recommended',
        'plugin:@typescript-eslint/recommended'
    ],
    parserOptions: {
        sourceType: 'module',
        ecmaVersion: '2018'
    },
    rules: {
        'indent': [
            'error',
            4
        ],
        'quotes': [
            'error',
            'single'
        ],
        'linebreak-style': [
            'error',
            'unix'
        ],
        'semi': [
            'error',
            'never'
        ],
        'comma-spacing': [
            'error', {
                before: false,
                after: true
            }
        ],
        'func-call-spacing': [
            'error'
        ],
        'implicit-arrow-linebreak': [
            'error'
        ],
        'computed-property-spacing': [
            'error'
        ],
        'space-infix-ops': [
            'error', {
                int32Hint: false
            }
        ],
        'key-spacing': [
            'error', {
                afterColon: true
            }
        ],
        'new-parens': [
            'error'
        ],
        'eqeqeq': [
            'error'
        ],
        'arrow-spacing': [
            'error'
        ],
        'generator-star-spacing': [
            'error', {
                before: false,
                after: true
            }
        ],
        'yield-star-spacing': [
            'error', {
                before: false,
                after: true
            }
        ],
        'array-bracket-spacing': [
            'error'
        ],
        'brace-style': [
            'error'
        ],
        'comma-dangle': [
            'error'
        ],
        'no-console': [
            'error', {
                allow: [
                    'log',
                    'error'
                ]
            }
        ],
        'for-direction': [
            'error'
        ],
        'no-debugger': [
            'error'
        ],
        'no-eval': [
            'error'
        ],
        'no-useless-computed-key': [
            'error'
        ],
        'no-var': [
            'error'
        ],
        'object-shorthand': [
            'error'
        ],
        'prefer-arrow-callback': [
            'error'
        ],
        '@typescript-eslint/no-var-requires': [
            '0'
        ],
        '@typescript-eslint/member-delimiter-style': [
            'error', {
                multiline: {
                    delimiter: 'none'
                }
            }
        ],
        '@typescript-eslint/no-explicit-any': [
            '0'
        ],
        '@typescript-eslint/no-unused-vars': [
            'error'
        ],
        '@typescript-eslint/explicit-member-accessibility': [
            '0'
        ],
        '@typescript-eslint/explicit-function-return-type': [
            '0'
        ]
    }
}


