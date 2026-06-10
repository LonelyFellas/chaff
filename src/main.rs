use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};

fn main() {
    let css = ".used { color: red } .unused { color: blue }";

    let sheet = StyleSheet::parse(css, ParserOptions::default()).unwrap();
    let out = sheet.to_css(PrinterOptions::default()).unwrap();

    println!("{}", out.code);
}
