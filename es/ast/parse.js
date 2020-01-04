const acorn = require('acorn')
const espurify = require('espurify')
const code = process.argv[2]
const ast = acorn.parse(code)
console.log(JSON.stringify(ast, null, 2))