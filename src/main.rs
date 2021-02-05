extern crate glob;
use glob::glob;
use pulldown_cmark::{html, Options, Parser};
use std::fs;

fn compile_markdown_to_markdown(content: String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);    
    return html_output.to_string()
}

fn main() {
    for entry in glob("./pages/**/*.md").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let markdown_file_path = path.display();
                let content = fs::read_to_string(markdown_file_path.to_string())
                    .expect("Something went wrong reading the file");
                println!("{}",compile_markdown_to_markdown(content));
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
