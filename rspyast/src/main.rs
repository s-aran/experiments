#[macro_use]
extern crate env_logger;
use rustpython_parser::{ast, Parse};

fn main() {
    let py = r#"
import unittest;

@class_decorator
class TestClass(unittest.TestCase):
    """hogeをpiyoしたfugaをmogeるテスト"""

    @unittest.skip
    def test_case1(self):
        """test for do qux to baz that bar did by foo"""

        a = 1
        b = 2
        c = a + b
        self.assertEqual(3, c)
"#;

    env_logger::init();

    let result = match ast::Suite::parse(py, "test_case.py").map_err(|e| e.to_string()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    };

    for e in result.iter() {
        println!("{:?}", e);
    }


    println!("");

    // let ra = result.get(0).unwrap();
    // println!("{:?}", ra.names.grt(0).unwrap());

    // println!("{:?}", result);
    // println!("{}", result);
}
