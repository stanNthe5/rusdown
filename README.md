## Rusdown
Rusdown is a fast Markdown parser built with Rust's `pulldown_cmark` library and compiled to WebAssembly (WASM), offering high performance and easy integration with JavaScript environments. It is **3x** faster than `markdown-it`.

## Install
```
npm i rusdown
```

## Usage
```js
import rusdown from 'rusdown'
const str = `
# Hello World
`
const rd = new rusdown();
console.log(rd.render(str))
```

## Benchmark
(benchmark/index.ts)

```
markdown-it: 66.376ms
rusdown: 26.136ms

markdown-it: 37.424ms
rusdown: 12.951ms

markdown-it: 34.82ms
rusdown: 11.748ms

markdown-it: 34.384ms
rusdown: 10.858ms

markdown-it: 33.947ms
rusdown: 10.623ms

markdown-it: 33.929ms
rusdown: 10.993ms

```

## Options
```
interface MarkdownOptions {
    enable_tables?: boolean;
    enable_footnotes?: boolean;
    enable_strikethrough?: boolean;
    enable_tasklists?: boolean;
    enable_smart_punctuation?: boolean;
    enable_heading_attributes?: boolean;
    enable_math?: boolean;
    enable_subscript?: boolean;
    enable_superscript?: boolean;
}
```
Example:
```js
const rd = new rusdown({enable_tables:true});
```

