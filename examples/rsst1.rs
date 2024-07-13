use assrt::rsst;

/*
 * Demonstrate example use of rsst!() assertions.
 */
fn main() {
    /* Passes */
    let reasonable_integer = 2;
    rsst!(reasonable_integer < 10);

    /* Panics with output:
     *
     * Assertion failed: reasonable_integer < 10
     *   reasonable_integer: 2000
     */
    let reasonable_integer = 2000;
    rsst!(reasonable_integer < 10);
}
