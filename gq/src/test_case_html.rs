use std::{fs::OpenOptions, io::Cursor};

use serde::{Deserialize, Serialize};

use crate::language::transpile_program;

#[derive(Deserialize, Serialize)]
struct SubTestCase {
    input: String,
    output: String,
}

#[derive(Deserialize)]
struct TestCase {
    name: String,
    code: String,
    test_cases: Vec<SubTestCase>,
}

#[derive(Serialize)]
struct TestCaseOutput {
    name: String,
    code: String,
    transpiled_code: String,
    test_cases: Vec<SubTestCase>,
}

pub fn gen_test_case_html() -> std::io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .open("test_cases/test_cases.yaml")?;

    let test_cases: Vec<TestCase> = serde_yml::from_reader(file).unwrap();

    let test_results: Vec<_> = test_cases
        .into_iter()
        .map(|test_case| {
            let mut output = Cursor::new(vec![]);
            transpile_program(&mut test_case.code.chars().peekable(), &mut output).unwrap();

            let result = std::str::from_utf8(output.get_ref().as_slice()).unwrap();
            TestCaseOutput {
                name: test_case.name,
                code: test_case.code,
                transpiled_code: result.to_owned(),
                test_cases: test_case.test_cases,
            }
        })
        .collect();

    let output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("test_cases/result.json")?;
    serde_json::to_writer(output_file, &test_results).unwrap();

    Ok(())
}
