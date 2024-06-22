use assrt::csst;

/*
 * Demonstrate example use of csst!() assertions.
 *
 * The program is equivalent to:
 *
 *     fn main() {
 *         // Passing assert
 *         let reasonable_integer = 2;
 *         assert!(reasonable_integer < 10, "Assertion failed. Needed 'reasonable integer < 10' but 'reasonable_integer' was '{}'", reasonable_integer);
 *
 *         // Failing assert
 *         let reasonable_integer = 2000;
 *         assert!(reasonable_integer < 10, "Assertion failed. Needed 'reasonable integer < 10' but 'reasonable_integer' was '{}'", reasonable_integer)
 *     }
 *
 * The program fails at the second assert with output:
 *
 *     Assertion failed: Expected 'reasonable_integer' < '10' but instead was '2000'.
 */
fn main() {
    // Passing assert
    let reasonable_integer = 2;
    csst!(reasonable_integer < 10);

    // Failing assert
    let reasonable_integer = 2000;
    csst!(reasonable_integer < 10);
}
