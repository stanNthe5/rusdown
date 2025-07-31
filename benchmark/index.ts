import { readFileSync } from 'node:fs';
import rustdown from 'rusdown'
import markdownit from 'markdown-it'


const rd = new rustdown();
const md = markdownit()

let f = readFileSync('./demo.md')
let str = f.toString()

function test() {
    let h: string = ""
    // markdown-it
    console.time('markdown-it')
    for (let i = 0; i <= 1000; i++) {
        h = md.render(str)
    }
    console.timeEnd('markdown-it')
    // rusdown
    console.time('rusdown')
    for (let i = 0; i <= 1000; i++) {
        h = rd.render(str)
    }
    console.timeEnd('rusdown')

}

for (let i = 0; i <= 5; i++) {
    test()
}