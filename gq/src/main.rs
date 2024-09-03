use test_case_html::gen_test_case_html;

mod language;
mod test_case_html;

pub fn main() {
    gen_test_case_html().unwrap();
}
