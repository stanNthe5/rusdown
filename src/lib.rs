use pulldown_cmark::{html, Options, Parser};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn markdown_to_html(
    markdown_input: &str,
    enable_tables: bool,
    enable_footnotes: bool,
    enable_strikethrough: bool,
    enable_tasklists: bool,
    enable_smart_punctuation: bool,
    enable_heading_attributes: bool,
    enable_math: bool,
    enable_subscript: bool,
    enable_superscript: bool,
) -> String {
    let mut parser_options = Options::empty();
    if enable_tables {
        parser_options.insert(Options::ENABLE_TABLES);
    }
    if enable_footnotes {
        parser_options.insert(Options::ENABLE_FOOTNOTES);
    }
    if enable_strikethrough {
        parser_options.insert(Options::ENABLE_STRIKETHROUGH);
    }
    if enable_tasklists {
        parser_options.insert(Options::ENABLE_TASKLISTS);
    }
    if enable_smart_punctuation {
        parser_options.insert(Options::ENABLE_SMART_PUNCTUATION);
    }
    if enable_heading_attributes {
        parser_options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    }
    if enable_math {
        parser_options.insert(Options::ENABLE_MATH);
    }
    if enable_subscript {
        parser_options.insert(Options::ENABLE_SUBSCRIPT);
    }
    if enable_superscript {
        parser_options.insert(Options::ENABLE_SUPERSCRIPT);
    }

    let parser = Parser::new_ext(markdown_input, parser_options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
