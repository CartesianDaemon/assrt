use itertools::{Itertools, EitherOrBoth::*};

#[cfg(test)]
mod rsst_tests {
    use assrt::rsst;
    use super::*;

    #[test]
    fn test_rsst_passes() {
        let reasonable_integer = 2;
        rsst!(reasonable_integer < 10);
    }

    #[test]
    fn test_rsst_fails() {
        let two: i32 = 2;
        let reasonable_integer = 2000;
        expect_panic(|| rsst!(reasonable_integer < 10), vec!(
            "failed: reasonable_integer < 10",
            "  reasonable_integer: 2000"
        ));

        expect_panic(|| rsst!( two.pow(2+3+5)+1 == 1023), vec!(
            "Assertion failed: two.pow(2+3+5)+1 == 1023",
            "  two.pow(2 + 3 + 5) + 1: 1025",
            "  two.pow(2 + 3 + 5): 1024",
            "  two: 2",
            "  2 + 3 + 5: 10",
            )
        );
    }
}

#[cfg(test)]
mod csst_tests {
    use assrt::csst;
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
        let two:i32 = 2;
        let reasonable_integer = 2000;
        expect_panic(|| csst!(reasonable_integer < 10), vec!("needed 'reasonable_integer' < '10' but was '2000'"));
        // Mostly to check syntax works ok for comparison with test_rsst_fails().
        expect_panic(|| csst!( two.pow(2+3+5)+1 == 1023), vec!("needed 'two"));
    }
}

fn result_to_err_string<R: std::fmt::Debug>(ret: &std::thread::Result<R>) -> &str {
    assert!(ret.is_err());
    let err = ret.as_ref().unwrap_err();
    if let Some(err_string) = err.downcast_ref::<String>() {
        err_string
    } else if let Some(err_string) = err.downcast_ref::<&str>() {
        err_string
    } else {
        panic!("Can't convert Err from panic into string");
    }
}

fn expect_panic(f: impl FnOnce() + std::panic::UnwindSafe, expected_lines: Vec<&str>) {
    let ret = std::panic::catch_unwind(f);
    let err_string = result_to_err_string(&ret);
    for exp_lin in expected_lines.iter().zip_longest(err_string.lines()) {
        match exp_lin {
            Both(expected, err_line) => assert!(err_line.contains(expected), "'{err_line}' does not contain '{expected}'"),
            Left(expected) => panic!("Expected another line containing {expected}"),
            Right(err_line) => panic!("Unexpected line in panic output: '{err_line}'"),
        }
    }
}
