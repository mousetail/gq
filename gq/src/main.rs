use language::get_builtin_tokens;
use test_case_html::gen_test_case_html;

mod language;
mod test_case_html;

pub fn main() {
    get_builtin_tokens();
    gen_test_case_html().unwrap();
}
