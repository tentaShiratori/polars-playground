use polars::prelude::*;
use termimad;

fn show(df: &DataFrame) {
    println!("{:?}", df);
}


fn main() {
    let df = CsvReader::from_path("sample.csv").unwrap().finish().unwrap();
    show(&df);
    termimad::print_text(r#"
# title
* a list item
* another item
"#);
    termimad::print_text("## title\n* a list item\n* another item");
}
