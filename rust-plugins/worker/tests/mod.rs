use std::env::args;

use regress::{Match, Regex};
const WORKER_OR_SHARED_WORKER_RE: &str = r#"(?:\?|&)(worker|sharedworker)(?:&|$)"#;
const WORKER_IMPORT_META_URL_RE: &str = r#"\bnew\s+(?:Worker|SharedWorker)\s*\(\s*(new\s+URL\s*\(\s*('[^']+'|"[^"]+"|`[^`]+`)\s*,\s*import\.meta\.url\s*\))"#;
#[test]
fn test_regex() {
  let re = Regex::new(WORKER_OR_SHARED_WORKER_RE).unwrap();
  let test_str = "src/worker/test.worker.ts?worker";
  assert_eq!(re.find(test_str).is_some(), true);

  //   let re = Regex::new(WORKER_IMPORT_META_URL_RE).unwrap();
  let test_str = r#"
  const s = new Worker(new URL("worker1.js", import.meta.url));
  const m = new Worker(new URL('worker2.js', import.meta.url))"#;
  //   for c in re.find(&test_str).unwrap().groups() {
  //     println!("{:?}", &test_str[c.unwrap()]);
  //     // 我需要递归这个 test_str 后续的字符
  //   }

  fn match_global(regex_str: &str, text: &str) -> Vec<Match> {
    let re = Regex::new(regex_str).unwrap();
    let mut matchs: Vec<Match> = Vec::new();
    let mut start = 0;
    loop {
      let m = re.find_from(text, start).next();
      match m {
        Some(m) => {
          matchs.push(m.clone());
          start = m.range().end;
          if start >= text.len() {
            break;
          }
        }
        None => break,
      }
    }
    matchs
  }

  let matches = match_global(WORKER_IMPORT_META_URL_RE, &test_str);
  matches.iter().for_each(|m| {
    let args = &m.captures[0].clone().unwrap();
    let worker_url = &m.captures[1].clone().unwrap();
    println!("args:{}",&test_str[args.start..args.end]);
    println!("worker_url:{}",&test_str[worker_url.start..worker_url.end])
  });
}
