#!/usr/bin/env node

import * as fs from 'fs'
import * as path from 'path'
const basePath = process.cwd()

const contnet = fs.readFileSync(path.join(__dirname, './index.ts'), 'utf8')


