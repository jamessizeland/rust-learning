# Variables

## let

Variables, by default are immutable and declared with the 'let' keyword. When a variable is immutable, once a value is bound to a name, you can’t change that value.

Mutable variables are declared with the 'mut' keyword.

There are multiple trade-offs to consider in addition to the prevention of bugs. For example, in cases where you’re using large data structures, mutating an instance in place may be faster than copying and returning newly allocated instances. With smaller data structures, creating new instances and writing in a more functional programming style may be easier to think through, so lower performance might be a worthwhile penalty for gaining that clarity.

## const

Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

First, you aren’t allowed to use 'mut' with constants. Constants aren’t just immutable by default **they’re always immutable**. You declare constants using the 'const' keyword instead of the 'let' keyword, and the **type of the value must be annotated**.

Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.

The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.
