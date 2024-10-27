use regress::Regex;
const WORKER_OR_SHARED_WORKER_RE: &str = r#"(?:\?|&)(worker|sharedworker)(?:&|$)"#;
const WORKER_FILE_RE: &str = r#"(?:\?|&)worker_file&type=(\w+)(?:&|$)"#;
const INLINE_RE: &str = r#"[?&]inline\b"#;

#[test]
fn test_regex() {
    let re = Regex::new(WORKER_OR_SHARED_WORKER_RE).unwrap();
    let test_str = "src/worker/test.worker.ts?worker";
    assert_eq!(re.find(test_str).is_some(), true);

    let re = Regex::new(WORKER_FILE_RE).unwrap();
    let test_str = "src/worker/test.worker.ts?worker_file&type=module";
    assert_eq!(re.find(test_str).is_some(), true);
    let pos = re.find(test_str).unwrap().group(1).unwrap();
    assert_eq!(&test_str[pos], "module");
}
