# Variables

## let

Variables, by default are immutable and declared with the 'let' keyword. When a variable is immutable, once a value is bound to a name, you can’t change that value.

Mutable variables are declared with the 'mut' keyword.

There are multiple trade-offs to consider in addition to the prevention of bugs. For example, in cases where you’re using large data structures, mutating an instance in place may be faster than copying and returning newly allocated instances. With smaller data structures, creating new instances and writing in a more functional programming style may be easier to think through, so lower performance might be a worthwhile penalty for gaining that clarity.

## const

[constant evaluation reference](https://doc.rust-lang.org/reference/const_eval.html)

Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

First, you aren’t allowed to use 'mut' with constants. Constants aren’t just immutable by default **they’re always immutable**. You declare constants using the 'const' keyword instead of the 'let' keyword, and the **type of the value must be annotated**.

Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.

The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

## Shadowing

Repeat the 'let' keyword to reassign a variables value.

Shadowing is different from marking a variable as 'mut', because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the 'let' keyword. By using 'let', we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.

## Integers

Length | Signed | Unsigned |
-------|--------|----------|
8-bit | i8 | u8
16-bit | i16 | u16
32-bit | i32 | u32
64-bit | i64 | u64
128-bit | i128 | u128
arch | isize | usize

Number literals | Example |
----------------|---------|
Decimal | 98_222
Hex | 0xff
Octal | 0o77
Binary | 0b1111_0000
Byte (u8 only) | b'A'

### Overflow

 Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping.

- Wrap in all modes with the wrapping_* methods, such as wrapping_add
- Return the None value if there is overflow with the checked_* methods
- Return the value and a boolean indicating whether there was overflow with the overflowing_* methods
- Saturate at the value’s minimum or maximum values with saturating_* methods

## Floats

The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.

## Booleans

booleans are 1 byte.

## Char

This is Rust's most primitive alphabetic type.  We specify char literals with single quotes, as opposed to string literals, which use double quotes. Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.

<https://doc.rust-lang.org/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings>

## Compound Types

Rust has two primitive compound types: tuples and arrays.

### Tuple

A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.

The tuple without any values, (), is a special type that has only one value, also written (). The type is called the unit type and the value is called the unit value. **Expressions implicitly return the unit value if they don’t return any other value.**
