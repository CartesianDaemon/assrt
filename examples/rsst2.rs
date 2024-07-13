use assrt::rsst;

/*
 * Demonstrate example use of csst!() assertions.
 */
fn main() {
    /* Panics with output:
     *
     * Assertion failed: two.pow(2+3+5)+1 == 1023",
     *   two.pow(2 + 3 + 5) + 1: 1025",
     *   two.pow(2 + 3 + 5): 1024",
     *   two: 2",
     *   2 + 3 + 5: 10",
     */
    let two:i32 = 2;
    rsst!( two.pow(2+3+5)+1 == 1023 );
}
