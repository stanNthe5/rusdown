## Rusdown
Rusdown is a fast Markdown parser built with Rust's `pulldown_cmark` library and compiled to WebAssembly (WASM), offering high performance and easy integration with JavaScript environments. It is twice as fast as the `markdown-it`.

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

