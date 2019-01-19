

import chalk from 'chalk'

const options = {
    // Seed of the rainbow, use the same for the same pattern
    seed: Math.round(Math.random() * 1000),
    // Spread of the rainbow
    spread: 8.0,
    // Frequency of the rainbow colors
    freq: 0.3
}

const rainbow = (freq: number, i: number) => {
    const red = Math.round(Math.sin(freq * i + 0) * 127 + 128)
    const green = Math.round(Math.sin(freq * i + 2 * Math.PI / 3) * 127 + 128)
    const blue = Math.round(Math.sin(freq * i + 4 * Math.PI / 3) * 127 + 128)
    return {
        red,
        green,
        blue
    }
}

const printlnPlain = (line: string) => {
    for (let i = 0; i < line.length; i++) {
        const colors = rainbow(options.freq, options.seed + i / options.spread)
        process.stdout.write(chalk.rgb(colors.red, colors.green, colors.blue)(line[i]))
    }
    process.stdout.write('\n')
}

export default (content: string): void => {
    const lines = content.split('\n')
    lines.forEach((line) => {
        options.seed += 1
        printlnPlain(line)
    })
}


