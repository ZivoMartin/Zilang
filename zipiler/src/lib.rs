mod zipiler;


#[cfg(test)]
mod tests {
    use super::zipiler::compile_txt;

    static PATHS: [&str; 5] = [
        "testing/main.zi", 
        "testing/global_tests/operations.zi",
        "testing/global_tests/class_definition.zi",
        "testing/global_tests/class_method_call.zi",
        "testing/global_tests/print.zi"
        ];

    #[test]
    fn ctest_ompile() {
        for p in PATHS {
            match compile_txt(String::new(), p, false) {
                Ok(()) => println!("PASSED: {p}"),
                Err(e) => panic!("During the test of {p}: {e}")
            }
        }
    }
}