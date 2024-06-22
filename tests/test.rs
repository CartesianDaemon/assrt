use itertools::{Itertools, EitherOrBoth::*};

use assrt::csst;

fn expect_panic(f: impl FnOnce() + std::panic::UnwindSafe, expected_lines: Vec<&str>) {
    let ret = std::panic::catch_unwind(f);
    assert!(ret.is_err());
    let err = ret.unwrap_err();
    let err_string = err.downcast_ref::<String>().expect("Need to coerce Err to String");
    println!("\nAssertion message: <<<{}>>>\n", err_string);
    for exp_lin in expected_lines.iter().zip_longest(err_string.lines()) {
        match exp_lin {
            // Use @ ?
            Both(expected, err_line) => assert!(err_line.contains(expected)),
            Left(expected) => panic!("Expected another line containing {expected}"),
            Right(err_line) => panic!("Unexpected line in panic output: '{err_line}'"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csst_malformd() {
        // These fail to compile. TODO: Need to use trybuild or doctests to test that.
        // csst!(let);
        // csst!(foo);
        // csst!(8);
        // csst!(1 + 2);
    }

    #[test]
    fn test_csst_passes() {
        let reasonable_integer = 2;
        csst!(reasonable_integer < 10);
    }

    #[test]
    fn test_csst_fails() {
        let reasonable_integer = 2000;
        expect_panic(|| csst!(reasonable_integer < 10), vec!("Needed 'reasonable_integer' < '10' but was '2000'"));
    }
}
