# rsst!() "Rich assert" macro

The rsst!() macro functions like assert!() but prints the value of expressions used in the
assert condition.

For example:

```
    // Source code
    rsst!( two.pow(2+3+5)+1 == 1023 );
```

```
$ cargo run
...
Assertion failed: two.pow(2+3+5)+1 == 1023
  two.pow(2 + 3 + 5) + 1: 1025
  two.pow(2 + 3 + 5): 1024
  two: 2
  2 + 3 + 5: 10
```

See examples/ directory for specific examples of rsst!() being used.

# csst!() "Comparison assert" macro

csst!() macro functions like rsst! but expects an assert condition using a comparison
operator (==, !=, <, <=, >, >=) and if the assert fails displays the value of the
expression on the LHS of the comparison.

See examples/ for an example of csst!() being used.

# Benefits

Seeing what value the variables and expressions used in an assert makes seeing the cause
of a failure quicker, compared to adding and removing println debugging or starting up a
debugger.

Manually writing a message for each assert displaying the values adds friction to writing
asserts. And programmers can guess wrong about which values are most useful.

Automatically displaying all of the values is usually most useful. And making asserts
easier to write means that programmers are less likely to leave them out when they would
have been useful.

# Notes

I started using rust very much only recently. This is an experiment to see what I could
do with rust's macro system, on functionality I've wanted in many languages.

It is directly inspired by the C++ test framework https://github.com/catchorg/Catch2
which provides a similar functionality in C++. I am used to C asserts being inconvenient.
I hoped other languages would be better, but the best I know of is asserts like in python
which require you to write `assert_eq(lhs, rhs)` which is less natural than `assert(lhs == rhs)`,
and is fiddly every time you need to change the condition. Programmers test things a lot,
it should be as convenient as possible.

This is an early draft. There are a lot of obvious holes, including:
* No way to specify an assert message.
* Doesn't display the values of an assert expressions with side-effects correctly.
* rsst! displays approximately "values of variables, function calls, and expressions" but
  this needs to be refined to display the values of subexpressions that are meaningful in
  most common circumstances.

# Implementation

rsst! uses a proc_macro and syn to walk a syntax tree for an expression and construct
arguments to an assert! that displays the values of subexpressions that look meaningful.

I like that this is possible. I don't know if this is good or bad to do. This was an
experiment, I don't know which existing crates would already provide this functionality.

csst! was also implemented as a proc_macro, but it turned out that it could have been
written as a macro_rules.
