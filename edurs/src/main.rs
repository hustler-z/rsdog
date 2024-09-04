/* --------------------------------------------------------------
 * Rust Quick Intro - 2024
 *
 * Programming Paradigms:
 *
 * (a) Functional Programming
 * (b) Expression-oriented Programming
 * (c) Pattern-oriented Programming
 *
 * Features:
 *
 * (a) Safeness
 * (b) Ownership
 * (c) Lifetimes
 * (d) Fearless Concurrency
 * (e) Zero-Cost Abstraction
 * --------------------------------------------------------------
 */

#![allow(dead_code)]

// --------------------------------------------------------------

//! Declare Immutable Variables
//! let varname: type = value;
//! let varname = value;
//! let varname: type;
//! let varname;

use std::io;

/* To use HashMap, Must "use" it. */
use std::collections::HashMap;

/* --------------------------------------------------------------
 *
 *        ownership                        borrowing
 *              \                           /
 *               +-------------------------+
 *               | Memory Safeness in Rust |
 *               +-------------------------+
 *                            |
 *                        lifetimes
 *
 * Ownership
 *
 * Rules: (a) Each value in Rust has an owner.
 *        (b) There can only be one owner at a time.
 *        (c) When the owner goes out of scope, the value will be
 *            dropped.
 *
 * Ownership can be borrowed instead of moved. '&' Reference
 * (a) At any given time, can have either one mutable
 *     reference or any number of immutable references.
 *     - Think of data race cases.
 *
 * Lifetimes - prevent dangling references (occur when a
 *             reference outlives a borrowed value, at
 *             the time, the reference points to invalid
 *             memory).
 *
 * A Lifetime spans the lifetime of a value.
 *
 * A lifetime annotation only labels a lifetime. It does not
 * modify a lifetime in any manner.
 *
 * fn test_lifetime() {
 *    let ref_0; -------------------------------+
 *    {                                         |
 *        let num_0 = 2; ----+                  |
 *        ref_0 = &num_0;    | 'a for num_0     | 'b for ref_0
 *    } ---------------------+                  |
 *    println!("{}", ref_0);                    |
 * } -------------------------------------------+
 *
 * Lifetime Elision
 *
 * a) When there is a single input lifetime, it is elided to the
 *    output lifetime.
 *    i.e., “one in, one out” elision rule
 *
 *    fn f(x: &T) -> (&T, &T) =>
 *    fn f<'a>(x: &'a T) -> (&'a T, &'a T)
 *
 *
 * b) when there are no references in the outputs from a function;
 *    in this case, each of the input references automatically
 *    gets its own lifetime, different from any of the other
 *    input parameters.
 *
 *    fn f(x: &T, y: &T, z: &U) -> u32 =>
 *    fn<'a, 'b, 'c>(x: &'a T, y: &'b T, z: &'c T) -> u32
 *
 * c) methods that use a reference to self (either &self or &mut
 *    self); in this case, the compiler assumes that any output
 *    references take the lifetime of self, as this turns out to
 *    be (by far) the most common situation.
 *
 *    fn f(&self, x: T, y: &T) -> &U =>
 *    fn f<'a, 'b, 'c>(&'a self, x: &'b T, y: &'c T) -> &'a U
 * --------------------------------------------------------------
 *
 * Lifetime sharing
 *
 * --------------------------------------------------------------
 *
 * If there are no input lifetimes, but the output return value
 * includes a reference:
 *
 * A static lifetime, 'static, spans the entire application.
 *
 * fn f() -> &'static T
 *
 * The Rust compiler guarantees that a static item always has
 * the same address for the entire duration of the program and
 * never moves.
 *
 * Subtyping Lifetime creates an extensible relationship
 * between two lifetimes.
 * 'subtype: 'basetype
 *
 * --------------------------------------------------------------
 *
 * Anonymous Lifetime '_ can be used in places where a specific
 * lifetime label is not needed.
 *
 * The anonymous lifetime '_ allows you to mark an elided
 * lifetime as being present, without having to fully restore all
 * of the lifetime names.
 *
 * --------------------------------------------------------------
 *
 * Rust has two types of pointers: safe pointers and raw pointers.
 *
 * References a.k.a borrowing
 *
 * References, &T, are primitives and the preferred pointer type
 * in Rust. References borrow values, means ownership remains
 * with the original binding.
 *
 * Raw pointers
 * 1) Are allowed to ignore the borrowing rules by having both
 *    immutable and mutable pointers or multiple mutable pointers
 *    to the same location
 * 2) Aren’t guaranteed to point to valid memory
 * 3) Are allowed to be null
 * 4) Don’t implement any automatic cleanup
 *
 * (a) *const T  -  immutable raw pointer
 * (b) *mut T    -  mutable raw pointer
 *
 * Note the asterisk * not dereference operator. here in the
 * context of raw pointers, immutable means that the pointer
 * can't be directly assigned to after being dereferenced.
 *
 * The lifetime of the owner must outlive the borrower (i.e.,
 * the reference).
 *
 *
 * Deref and DerefMut Traits - Dereferencing Operators * and .
 *
 * Deref
 *
 * DerefMut - Dereference Mutably
 *
 * deref coercion ()
 *
 * --------------------------------------------------------------
 */
#[allow(unused)]
struct Data<'a> {
    val_0: &'a u32,
}

/* Tuple Struct     - A combination of a struct and a tuple.
 * Unit-like Struct - A struct with no fields.
 */
#[derive(Debug)]
struct Person(String, u8, String);

/* Note that: &self => self: &Self
 */
#[allow(unused)]
impl Data<'_> {
    fn do_op<'b>(&self, ref_0: &'b u32) -> &'b u32 {
        ref_0
    }
}

/* --------------------------------------------------------------
 * An attribute is metadata applied to some module, crate or
 * item.
 *
 * #[outer_attribute]  - apply to the item immediately following
 *                       it.
 * #![inner_attribute] - apply to enclosing item (typically a
 *                       module or a crate).
 *
 * #[cfg(...)]         - attribute position
 *
 * e.g. #[cfg(target_os = "linux")]
 * --------------------------------------------------------------
 */
#[allow(unused)]
fn test_lifetime_label<'a>(ref_0: &'a u32, ref_1: &'a u32) -> &'a u32 {
    ref_0
}

#[allow(unused)] /* attribute - unused function / variable */
fn interact_console() {
    let mut passwd = String::new();

    println!("Enter the password:");

    io::stdin().read_line(&mut passwd).unwrap_or_default();

    if passwd != "" {
        println!("\nPassword: {}", passwd.trim_end());
    } else {
        println!("\nEmpty password!!");
    }
}

/* --------------------------------------------------------------
 * Methods (方法)
 *
 * Associate functions with the new type using impl
 *
 * self - specify the "receiver" - the object the method acts on
 *
 * Traits (特征)
 *
 * a) Define a set of related functions that some underlying item
 *    make publicly available.
 * b) Describe collections of related functionality that all
 *    apply to the same underlying item.
 *
 * To implement traits: impl Trait for Type {..}
 *
 * Supertraits
 *
 * A trait can require the types implementing it also implement
 * other traits.
 *
 * In Rust, can define a trait as being a superset of another
 * trait.
 * --------------------------------------------------------------
 */
trait State {
    fn info(&self) -> String;
    fn put(&self) -> u32;
}

trait Dread {
    fn scare(&self) -> u8;
}

trait About: State + Dread {
    fn subset(&self) -> &'static str;
}

struct Status {
    stat: String,
    code: u32,
    dots: u8,
}

impl State for Status {
    fn info(&self) -> String {
        format!("Status: {}-{}", self.stat, self.code)
    }

    fn put(&self) -> u32 {
        self.code + 1
    }
}

impl Dread for Status {
    fn scare(&self) -> u8 {
        self.dots
    }
}

impl About for Status {
    fn subset(&self) -> &'static str {
        "about subset"
    }
}

/* Derivation is implemented with macros
 * - derive macros add useful functionality
 */
#[derive(Debug)]
struct Number(i32);

#[derive(Debug)]
struct Mulout(i32);

/* --------------------------------------------------------------
 * Associated types (a.k.a. output types) are placeholder types
 * which are supplied by the trait implementation.
 * --------------------------------------------------------------
 */
trait MathTech {
    type Output;
    fn mathmul(&self, other: &Self) -> Self::Output;
}

impl MathTech for Number {
    type Output = Mulout;

    fn mathmul(&self, other: &Self) -> Self::Output {
        Mulout(self.0 * other.0)
    }
}

use std::mem;

fn test_struct() {
    let status = Status {
        stat: String::from("broken"),
        code: 32,
        dots: 7,
    };

    println!("{} {} bytes: {} {} {}", status.info(),
            mem::size_of_val(&status), status.put(),
            status.scare(), status.subset());

    println!("Associated Type: {}",
            Number(21).mathmul(&Number(25)).0);
}

// --------------------------------------------------------------

/* Deref Coercion */
fn iter_str(s: &str) {
    for c in s.chars() {
        print!("{} ", c);
    }

    println!();
}

fn check_ops(x: &u32) {
    match x { // implicit dereference
        2 => println!("step 2"),
        5 => println!("step 5"),
        /* default，_ as wildcard pattern (忽略模式)
         * which matches any value.
         */
        _ => println!("step n"),
    }
}

/* Match Guard causes the arm to match only if the condition
 * is true.
 */
fn check_ops_guard(pair: &(u32, u32)) {
    print!("About {:?}: ", pair);

    match pair {
        (x, y) if x == y => println!("twins!!"),
        (x, y) if x % 2 == 0 && y % 2 == 0 => {
            println!("evens!!");
        },
        (x, _) if x % 2 == 1 => println!("1st one is odd!!"),
        _ => println!("No correlation ..."),
    }
}

/* --------------------------------------------------------------
 * Nested Functions in Rust
 *
 * A private function of the outer function.
 *
 * Function Pointers
 *
 * Function Alias
 * --------------------------------------------------------------
 */
fn test_fp(x: u32) -> u32 {
    x * x
}

/* Never Type
 *
 */

#[derive(Debug)]
enum Direction {
    South,
    North,
    East,
    West,
    Lost,
}

fn give_direction(op: u32) -> Direction {
    match op {
        1 => Direction::South,
        2 => Direction::North,
        3 => Direction::East,
        4 => Direction::West,
        _ => Direction::Lost,
    }
}

// #[allow(unused)]

/* --------------------------------------------------------------
 * break    - exit the loop early
 * continue - immediately start the next iteration
 *
 * Both continue and break can optionally take a label
 * argument which is used to break out of nested loop.
 *
 * The loop label - can break or continue in an outer loop.
 * 'label: while
 * 'label: for
 * 'label: loop
 * --------------------------------------------------------------
 */
fn loop_thru() {
    let mut outer_cnt: u8 = 0;
    'loopto: while outer_cnt < 10 {
        outer_cnt += 1;
        let mut inner_cnt: u8 = 1;
        println!();

        /* The loop statement just loops forever, until a break.
         */
        loop {
            print!("{} ", inner_cnt);
            if inner_cnt >= outer_cnt {
                continue 'loopto;
            }
            inner_cnt += 1;
        }
    }

    println!();

    let arr = [0_u16; 16];

    /* Rust will check for out-of-bounds access on manual array
     * indexing to guarantee memory safety, while C will happily
     * index outside the array.
     */
    for element in arr.iter() {
        print!("{} ", element);
    }

    println!();
}

/* Collections - Array, Vector, HashMap
 */
fn test_collections() {
    /* Array [T; N]
     */
    let array_0: [u32; 4] = [1, 2, 3, 4];

    /* Slices
     *
     * A slice is a partial array that references a contiguous
     * sequence of elements. A slice consists of two fields: a
     * starting location and length. Define a slice with start
     * -ing and ending indexes, with ellipses as a delimiter.
     */
    println!("[1, 3) => {:?}", &array_0[1..3]);
    println!("[1, 3] => {:?}", &array_0[1..=3]);

    /* Vectors - Dynamic Array */
    let mut vec_0: Vec<u32> = Vec::new();
    vec_0.push(21);
    vec_0.push(13);
    vec_0.push(17);
    println!("{:?} capacity {} len {}", vec_0, vec_0.capacity(),
             vec_0.len());

    let mut vec_1 = vec![1, 2, 3, 4];
    vec_1.push(5);

    /* Vectors
     *
     * capacity - The size of the backing array on heap.
     * length   - The actual number of values in the vector.
     */
    println!("{:?} capacity {} len {}", vec_1, vec_1.capacity(),
             vec_1.len());

    let mut vec_3 = Vec::with_capacity(8);

    /* Move Semantics as default in Rust
     */
    let mut vec_2 = vec_1; /* moved, vec_1 is no longer valid */
    println!("{:?} capacity {} len {}", vec_2, vec_2.capacity(),
             vec_2.len());

    vec_3.append(&mut vec_2);
    vec_3.push(6);
    println!("{:?} capacity {} len {}", vec_3, vec_3.capacity(),
             vec_3.len());

    /* HashMap - Dynamic Key & Value Pairs
     */
    let mut map_0 = HashMap::new();
    map_0.insert("Math", 113);
    map_0.insert("Chinese", 96);

    let result = map_0.get(&"Math");

    /* ----------------------------------------------------------
     * Both Option and Result provide a pair of methods that
     * extract their inner value and panic! if it's absent:
     * .unwrap() and .expect().
     *
     *
     * Option<T> stores either a value of type T or nothing.
     *
     * unwrap will return the value in an option, or panic.
     * expect is similar but takes an error message.
     *
     * enum Option<T> {
     *     None,           - Indicate failure/lack of value
     *     Some(T),        - a tuple struct that wraps a
     * }                     value with type T.
     * ----------------------------------------------------------
     */
    match result {
        Some(value) => println!("Found {}", value),
        None => println!("Not Found"),
    }

    /* ----------------------------------------------------------
     * Whereas Result<T, E> enum:
     * a) Ok(value)        - Indicate operation succeeded,
     *                       and wrap the value returned by
     *                       operation.
     * b) Err(why)         - Indicate operation failed, and
     *                       wrap why which hopefully explained.
     *
     * ? used at the end of an expression returning a Result.
     *
     * This piece of syntactic sugar takes care of matching the
     * Err arm, transforming the error type if necessary, and
     * building the return Err(...) expression, all in a single
     * character.
     * ----------------------------------------------------------
     */
}

/* overflowing_*() - (add, sub, mul, div, pow)
 */


/* --------------------------------------------------------------
 * Macros
 *
 * macro_rule! - metaprogramming (元编程)
 *
 * Declarative macros
 *
 * The arguments of a macro are prefixed by $ and type
 * annotated with a designator:
 *
 * $ name : fragment-specifier
 *
 * Matches a Rust syntax fragment of the kind specified and
 * binds it to the metavariable $name.
 *
 * 1)  block
 * 2)  expr
 * 3)  ident      - variable/function names
 * 4)  item
 * 5)  literal    - literal constants
 * 6)  pat        - pattern
 * 7)  path
 * 8)  stmt       - statement
 * 9)  tt         - token tree
 * 10) ty         - type
 * 11) vis        - visibility qualifier
 * 12) meta       - an Attr, the contents of an attribute
 * 13) pat_param
 *
 * Repetition operators:
 *
 * 1) * — indicates any number of repetitions.
 * 2) + — indicates any number but at least one.
 * 3) ? — indicates an optional fragment with zero or one
 *        occurrence.
 * --------------------------------------------------------------
 */
#[allow(unused_macros)]
macro_rules! vec_simple {
    ($( $x:expr ), *) => {
        /* A block in Rust contains a sequence of expressions,
         * enclosed by braces {}.
         *
         * A variable's scope is limited to the enclosing
         * block.
         */
        {
            let mut tmp_vec = Vec::new();
            $(
                tmp_vec.push($x);
            )*
            tmp_vec
        }
    }
}

macro_rules! pr_debug {
    ($expression:expr) => {
        println!("[Debug] {:?} = {:?}",
                 stringify!($expression),
                 $expression);
    };
}

/* --------------------------------------------------------------
 *
 * Generics (泛型编程)
 *
 * Generics are templates for building unique functions and type
 * definitions.
 *
 * (a) Code reuse
 * (b) Refactoring
 * (c) Extensibility
 * (d) Less error prone
 * (e) Unique capabilities
 *
 * Generic functions are templates for creating concrete
 * functions, using type parameters.
 *
 * fn funcname<T>(param: T) -> T {
 *    let variable: T;
 * }
 *
 * --------------------------------------------------------------
 *
 * (a) Type parameters in the impl definition:
 *
 *           Type Parameter
 *   keyword  within impl  struct name
 *          \    |         /
 *         impl <T>   Wrapper<T> {...}
 *
 *
 * (b) A generic trait for a generic struct:
 *
 *   impl block  Trait name    Struct name
 *       \           |             /
 *     impl<T>    Trait<T> for Struct<T> {...}
 *
 * --------------------------------------------------------------
 *
 * Pattern (模式)
 *
 * Patterns come in two forms:
 * a) refutable   (可反驳的模式)
 * b) irrefutable (不可反驳的模式)
 *
 * Patterns that will match for any possible value passed are
 * irrefutable.
 *
 * Pattern Syntax (模式语法)
 *
 * 1) Ignore an entire value with _
 * 2) Ignore an unused variable by starting its name with _
 * 3) Ignore remaining parts of a value with ..
 * 4) Extra conditionals with match guards (匹配守卫)
 * 5) @ bindings - create a variable that holds a value at the
 *                 same time as testing that value for a pattern
 *                 match. (绑定并验证模式匹配)
 *
 * --------------------------------------------------------------
 */
fn swap<T, U>(tuple: (T, U)) -> (U, T) {
    (tuple.1, tuple.0)
}

#[allow(unused)]
struct GenericBlk<T, U> {
    size: T,
    name: U,
}

#[allow(unused)]
impl<T, U> GenericBlk<T, U> {
    /* Associated functions (without a Self parameter)
     */
    fn func_0(x: T) -> T {
        x
    }

    fn func_1(self) -> (T, U) {
        (self.size, self.name)
    }
}

/* --------------------------------------------------------------
 *
 * Trait Bounds - Indicate that generic code that is
 *                parameterized by some type T can be used only
 *                when that type T implements some specific
 *                trait.
 *
 * impl Trait syntax can be used in function arguments and
 * return values. It allows us to work with types which
 * cannot name.
 *
 * --------------------------------------------------------------
 * Trait Objects
 *
 * dyn Trait syntax - type-erased and dynamic dispatch
 *
 * A trait object is the other way to make use of the
 * encapsulation defined by a trait, but here, different possible
 * implementations of the trait are chosen at runtime rather than
 * compile time. This dynamic dispatch is analogous to using
 * virtual functions in C++, and under the covers, Rust has
 * “vtable” objects that are roughly analogous to those in C++.
 *
 * dynamic aspect of trait objects also means that they always
 * have to be handled indirectly, via a reference
 * (e.g., &dyn Trait) or a pointer (e.g., Box<dyn Trait>) of some
 * kind. The reason is that the size of the object implementing
 * the trait isn’t known at compile time—it could be a giant
 * struct or a tiny enum—so there’s no way to allocate the right
 * amount of space for a bare trait object.
 *
 * Trait objects are fat pointers that combine a pointer to the
 * underlying concrete item with a pointer to a vtable that in
 * turn holds function pointers for all of the trait
 * implementation’s methods.
 *
 * struct X on Stack
 *    +------------+
 * +->|            |     +------------+
 * :  :            :  +->|            | vtable
 * |  |            |  |  +------------+
 * |  +------------+  |
 * |                  |
 * |  +------------+  |
 * +--| trait_ptr0 |  |
 *    +------------+  |
 *    | trait_ptr1 |--+
 *    +------------+
 * --------------------------------------------------------------
 */

/// fn duplicate<T: Clone>(a: T) -> (T, T) {
///     (a.clone(), a.clone())
/// }

fn duplicate<T>(a: T) -> (T, T)
    where T: Clone /* Trait Bound */
{
    (a.clone(), a.clone())
}

fn test_generic() {
    let bookname = String::from("Shit gon' happen");
    let pair = duplicate(bookname);
    println!("Book name: {pair:?}");
}

// --------------------------------------------------------------

/* test Modules
 *
 * --------------------------------------------------------------
 * Operator Overloading
 *
 * Unary operators
 *     std::ops::Neg                 -x
 *     std::ops::Not                 !x
 * Arithmetic operators
 *     std::ops::Add                 x + y
 *     std::ops::Sub                 x - y
 *     std::ops::Mul                 x * y
 *     std::ops::Div                 x / y
 *     std::ops::Rem                 x % y
 * Bitwise operators
 *     std::ops::BitAnd              x & y
 *     std::ops::BitOr               x | y
 *     std::ops::BitXor              x ^ y
 *     std::ops::Shl                 x << y
 *     std::ops::Shr                 x >> y
 * Compound assignment arithmetic operators
 *     std::ops::AddAssign           x += y
 *     std::ops::SubAssign           x -= y
 *     std::ops::MulAssign           x *= y
 *     std::ops::DivAssign           x /= y
 *     std::ops::RemAssign           x %= y
 * Compound assignment bitwise operators
 *     std::ops::BitAndAssign        x &= y
 *     std::ops::BitOrAssign         x |= y
 *     std::ops::BitXorAssign        x ^= y
 *     std::ops::ShlAssign           x <<= y
 *     std::ops::ShrAssign           x >>= y
 * Comparison
 *     std::cmp::PartialEq           x == y, x != y
 *     std::cmp::PartialOrd          x < y, x <= y, x > y, x >= y
 * Indexing
 *     std::ops::Index               x[y], &x[y]
 *     std::ops::IndexMut            x[y] = z, &mut x[y]
 * --------------------------------------------------------------
 */
#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use std::ops::{Add, Mul};
    /* operations + and * are needed for below generic function
     * Add<Output=N> ensures N + N = N.
     */
    fn dot<N>(v1: &[N], v2: &[N]) -> N
        where N: Add<Output=N> + Mul<Output=N> + Default + Copy
    {
        let mut total = N::default();
        for i in 0..v1.len() {
            total = total + v1[i] * v2[i];
        }
        total
    }

    #[test]
    fn test_dot() {
        assert_eq!(dot(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);
    }
}


/* --------------------------------------------------------------
 * Unsafety
 *
 * Unsafe operations can potentially violate the memory-safety
 * guarantees of Rust's static semantics.
 *
 * 1) Dereferencing a raw pointer.
 * 2) Reading or writing a mutable or external static variable.
 * 3) Accessing a field of a union, other than to assign to it.
 * 4) Calling an unsafe function (including an intrinsic or
 *    foreign function).
 * 5) Implementing an unsafe trait.
 * --------------------------------------------------------------
 */
use std::arch::asm;

fn test_unsafety() {
    let m: u64 = 3;
    let n: u64;

    unsafe {
        asm!(
            "mov {0}, {1}",
            "add {0}, {0}, 5",
            out(reg) n,
            in(reg) m,
        );
    }

    println!("unsafe op {}", n);

    let mut s0 = String::from("Leemi ");
    s0.push_str("loves me so much");

    unsafe {
        let (ptr, capacity, len): (usize, usize, usize) =
                                  mem::transmute(s0);
        println!("ptr: {ptr:#x} len: {len} capacity: {capacity}");
    }
}

// --------------------------------------------------------------

/* Closures (闭包) are anonymous functions that save in a variable
 * or pass as arguments to other functions.
 *
 * a.k.a lambda => e.g. |val| val + x
 *
 * Closures can capture values from their environment in three
 * ways, which directly map to the three ways a function can
 * take a parameter: borrowing immutably, borrowing mutably,
 * and taking ownership. The closure will decide which of
 * these to use based on what the body of the function does
 * with the captured values.
 */
fn test_closure() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut __borrows = || list.push(7);
    __borrows();

    println!("After definining closure: {list:?}");
}

/* An iterator is responsible for the logic of iterating over
 * each item and determining when the sequence has finished.
 */

// --------------------------------------------------------------

/* Modules in Rust - Hierarchically split code in logical units
 *                   (modules), and manage visibility (public
 *                   and private) between them.
 *
 * A module is a collection of items, named features.
 * a) Module Items - contain within a source file
 * b) Module Files - span a file
 *
 * pub(...) also used to restrict the scope of public visibility.
 *
 * One function is marked pub(crate), meaning that it is
 * available anywhere inside this crate.
 *
 * Paths
 *
 * a) relative path
 * b) absolute path
 *
 */

/* Path attribute - explicitly set the physical location of a
 *                  module, overriding the default.
 */
#[path = "coda/cola.rs"]
mod cola;

/* Module Files span an entire file.
 *
 * Omitting the module content will tell Rust to look for it
 * in another file.
 */
mod deep; // -> src/deep.rs

/* Each crate is a complete, cohesive unit.
 */
mod hustler {
    use crate::deep::checklists;

    pub fn hustler_out() {
        println!("This is hustler module!!!");
        println!("{:?}", checklists::in_range(22));
    }

    fn hustler_do() {
        println!("This is private function in hustler!!!");
    }

    mod dawg {
        pub fn dawg_crap() {
            /* Relative Path: refer to parent module */
            super::hustler_do();
            println!("This is private dawg module!!!");
        }
    }

    pub mod hiphop {
        pub fn hiphop_thug() {
            /* Absolute Path */
            crate::hustler::dawg::dawg_crap();
            println!("This is public hiphop module!!!");
        }
    }
}

fn test_mods() {
    cola::check_spot();

    /* Relative Path */
    hustler::hustler_out();
    hustler::hiphop::hiphop_thug();
}

/* --------------------------------------------------------------
 * Threads  - Native OS threads via the spawn function.
 *
 * Thread panics are independent of each other.
 *
 * Fearless Concurrency (无畏并发)
 *
 * (并发编程)
 * a) Concurrent programming - different parts of a program
 *                             execute independently.
 * (并行编程)
 * b) Parallel programming   - different parts of a program
 *                             execute at the same time.
 *
 * Message passing (消息传递)
 *
 * Channels - Communication between threads.
 *            Unidirectional flow of information allowed
 *            e.g. Sender -> Receiver
 *
 * Mutex<T> (互斥器)
 * Rc<T>
 * Arc<T> (原子引用计数)
 * --------------------------------------------------------------
 */
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::process::{Command, Stdio};

/* Static variables will live during the whole execution of the
 * program.
 *
 * static provideds object identity - an address in memory and
 * state as required by types with interior mutability.
 *
 * --------------------------------------------------------------
 *
 * whereas const has no object identity. and contants are
 * evaluated at compile time and the values are inlined
 * wherever used.
 *
 * Only function marked const can be called at compile time to
 * generate const values. const function can however be called
 * at runtime.
 * --------------------------------------------------------------
 */
static NR_THREADS: u32 = 10;
static NR_THRDS: u32 = 5;
static TOML_PATH: &'static str = "Cargo.toml";
static TAG_BEST: &'static str =
"the best out of the best!!";

fn test_threads() {
    let mut children = vec![];

    for i in 0..NR_THREADS {
        children.push(thread::spawn(move || {
            let tid = thread::current().id();
            println!("{:#?} number {}",
                tid, i)
        }));
    }

    for child in children {
        let _ = child.join(); // block 'til all threads finished.
    }

    /* Test on Channels
     *
     * Rust supports using pattern matching to destructure a
     * large value into its constituent parts.
     */
    let (tx, rx): (Sender<u32>, Receiver<u32>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..NR_THRDS {
        let thread_tx = tx.clone();

        /* ------------------------------------------------------
         * thread::spawn used to create a new thread, and pass a
         * closure containing the code that runs in the new
         * thread.
         *
         * move here used to transfer ownership of values from
         * one thread to another.
         * ------------------------------------------------------
         */
        let child = thread::spawn(move || {
            thread_tx.send(id).unwrap();
            println!("thread {} finished", id);
        });

        children.push(child);
    }

    let mut ids = Vec::with_capacity(NR_THRDS as usize);

    for _ in 0..NR_THRDS {
        ids.push(rx.recv());
    }

    for child in children {
        child.join().expect("oops! the child thread crashed!!");
    }

    println!("{:?}", ids);

    /* File I/O Operations
     */
    let path = Path::new(TOML_PATH);
    let show = path.display();

    println!();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}",
                           show, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}",
                           show, why),
        Ok(_) => print!("{} contains: \n{}",
                        show, s),
    }

    /* Pipes */
    let process = match Command::new("wc")
                                 .stdin(Stdio::piped())
                                 .stdout(Stdio::piped())
                                 .spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", why),
        Ok(process) => process,
    };

    match process.stdin.unwrap().write_all(TAG_BEST.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}",
                           why),
        Ok(_) => println!("sent pangram to wc"),
    }

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}",
                           why),
        Ok(_) => print!("wc responded with: \n{}", s),
    }
}

/* --------------------------------------------------------------
 * Smart Pointer (智能指针)
 *
 * Using Box<T> to point to data on the heap
 *
 * Reference Counting (引用计数智能指针)
 *
 * The Rc<T> type keeps track of the number of references to
 * a value to determine whether or not the value is still in
 * use. If there are zero references to a value, the value
 * can be cleaned up without any references becoming invalid.
 *
 * use Rust’s smart pointers for interconnected data
 * structures:
 *
 * a) Rc allows shared ownership, with multiple things
 *    referring to the same item. Rc is often combined with
 *    RefCell.
 * b) RefCell allows interior mutability so that internal
 *    state can be modified without needing a mutable
 *    reference. This comes at the cost of moving borrow
 *    checks from compile time to runtime.
 * c) Arc is the multithreading equivalent to Rc.
 * d) Mutex (and RwLock) allows interior mutability in a
 *    multithreading environment, roughly equivalent to
 *    RefCell.
 * e) Cell allows interior mutability for Copy types.
 * --------------------------------------------------------------
 */
use std::{
    sync::Arc,
};

fn test_thread() {
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    thread::spawn(move || dbg!(a));
    thread::spawn(move || dbg!(b));
}

// --------------------------------------------------------------

/* Standard Library Types
 *
 * core    - most basic types, ain't depend on libc
 * alloc   - types rquire a global heap allocator, Vec, Box, etc.
 * std
 *
 * --------------------------------------------------------------
 *
 * Standard Library Traits
 *
 * PartialEq and Eq         ==, !=
 * PartialOrd and Ord       <, <=, >=, >
 *
 * Operator overloading:    std::ops
 *
 * --------------------------------------------------------------
 * User-Defined Type Conversions
 *
 * From<T>
 * TryFrom<T>
 * Into<T>
 * TryInto<T>
 *
 * --------------------------------------------------------------
 * Closure (Lambda Expression)
 *
 * Fn* Traits
 *
 * The code that receives the closure has to accept an instance
 * of one of the Fn* trauts:
 *
 * Fn        - consume/mutate captured values,
 *             can be called multiple times, concurrently
 * FnMut     - might mutate captured values
 *             can be called multiple times, not concurrently
 * FnOnce    - may call it once, might consume captured values
 *
 * --------------------------------------------------------------
 */
fn closure_trait(func: impl FnOnce(u32) -> u32, arg: u32) -> u32 {
    println!("called with {arg}");
    func(arg)
}

fn select_op(who: String) -> impl Fn(&str) {
    return move |action| println!("{} got {}", who, action);
}

fn test_closure_trait() {
    println!("Add 3: {}", closure_trait(|x| x + 3, 10));

    let act = select_op("Rock".to_string());
    act("smacked");
}

fn main() {
    /* Type Inference
     */
    let var_0 = 65 as char;
    let var_1 = 'A' as u32;
    let value: u32 = 100;

    let var_2: u32 = 134;
    let var_3: u64 = var_2.into();

    /* ----------------------------------------------------------
     * Explicit Type Casting - "as" and from/into
     *
     * >_@ Rock
     *
     * The as version also allows lossy conversions, whereas
     * from/into do not.
     *
     * Type coercions are implicit operations that change the
     * type of a value. They happen automatically at specific
     * locations and are highly restricted in what types
     * actually coerce.
     *
     * ----------------------------------------------------------
     */
    let addr_var_1 = &var_1 as *const u32;

    /* ----------------------------------------------------------
     * The two most used string types in Rust are:
     * a) String - stored as a vector of bytes (Vec<u8>), but
     *             guaranteed to always be a valid UTF-8
     *             sequence.
     * b) &str   - a slice (&[u8]) that always points to a
     *             valid UTF-8 sequence, and can be used to
     *             view into a String.
     *
     * &[T] is a view into Vec<T>
     *
     * String Type consists of three fields:
     *
     * Far pointer on Stack         Data on Heap
     * +---------------+            +-------+-------+
     * |    pointer    |----------->| Index | Value |
     * +---------------+            +-------+-------+
     * |    length     |            :       :       :
     * +---------------+            +-------+-------+
     * |   capacity    |
     * +---------------+
     *
     * push
     * push_str
     * insert
     * insert_str
     *
     * slices:
     *
     * string[starting index..ending index]
     * ----------------------------------------------------------
     */
    let mut s_0 = String::from("Hustler");
    let s_1 = "Hacker".to_string();
    let mut s_2 = String::from("Fuk");

    let data_0: u32 = 520;
    /* Tuple
     *
     * The empty tuple () is referred to as the "unit type" and
     * signifies absence of a return value, akin to void in C.
     */
    let tuple_0 = (10, "ten");

    let tuple_1 = (12, 13);

    /* Function Alias
     *
     * The 'type' statement can be used to give a new name to
     * an existing type. Types must have UpperCamelCase names,
     * or the compiler will raise a warning.
     */
    type Funcptr = fn(u32) -> u32;
    /* Function pointer
     *
     * A pointer to some code, with a type that reflects the
     * signature of the function.
     */
    let fp_0: Funcptr = test_fp;

    println!("---------------------------------");

    check_ops(&value);

    check_ops_guard(&tuple_1);

    println!("Function pointer: {}", fp_0(100));

    println!("Before swap: {:?}", tuple_0);

    println!("After swap:  {:?}", swap(tuple_0));

    s_0.push_str(" Rust");

    s_2.insert(2, 'c');

    println!("---------------------------------");

    println!("Hello, world!");

    println!("D: {:?}", give_direction(2));

    println!("Decimal {}, Binary {:b}, Octal {:o}, Hexadecimal {:x}",
             data_0, data_0, data_0, data_0);

    println!("{}", format!("{} is a {} (capacity {} len {}) {}",
        s_0, s_1, s_1.capacity(), s_1.len(), s_2.to_uppercase()));

    iter_str(&s_1);

    println!("u32 [{}, {}]", u32::MIN, u32::MAX);

    println!("Type casting: {} {} {}", var_0, var_1, var_3);

    /* ----------------------------------------------------------
     * Print Implementation
     *
     * (a) Public facing {}
     * (b) Developer facing {:?}
     * {:#?} adds a linefeed between each element displayed.
     * ----------------------------------------------------------
     */
    println!("Address of var_1: {:?}", addr_var_1);

    println!("---------------------------------");


    test_thread();

    // interact_console();
    loop_thru();

    println!();

    test_collections();

    let rock = Person("Rock Lo".to_string(), 30,
                      "Programmer".to_string());

    println!("Name: {}, Age: {}, Job: {}",
             rock.0, rock.1, rock.2);

    /* ----------------------------------------------------------
     * Let
     *
     * a) if let expression - execute different code depending
     *    on whether a value matches a pattern.
     * b) let else expression
     * c) while let expression - loop keeps going as long as
     *    the value matches the pattern.
     *
     * Binding values to names - @
     *
     * In match expressions, you can match multiple patterns
     * using the | (as an or) syntax, which is the pattern or
     * operator.
     * ----------------------------------------------------------
     */
    if let a@(5|65|100) = value {
        println!("Binding @: Match found {}", a);
    }

    test_struct();

    test_closure();

    pr_debug!({
        let x = 55_u32;
        x * x + x * 2 - 1
    });

    test_generic();

    test_closure_trait();

    test_mods();

    test_unsafety();

    println!("---------------------------------");

    // test_threads();

    println!("---------------------------------");
}

// --------------------------------------------------------------

/* Stack
 * 1) values have fixed sizes known at compile time.
 * 2) Extremely fast: just move a stack pointer.
 * 3) Easy to manage: follows function calls.
 * 4) Great memory locality.
 *
 * Heap
 * 1) Values have dynamic sizes determined at runtime.
 * 2) Slightly slower than the stack: some book-keeping
 *    needed.
 * 3) No guarantee of memory locality.
 *
 * Resource Acquisition is Initialization (RAII paradigm)
 * A stack-based replace may contain a reference to heap memory
 * When the stack-based value is dropped (removed from memory),
 * the related heap memory must also be freed.
 *
 * Shallow Copy vs. Deep Copy
 *
 * Types with the Copy trait have copy semantics. When copied,
 * a bitwise copy is performed.
 * Types that reference external resources or memory, such as
 * Strings, do not support copy semantics.
 *
 * In Rust, all scalar types have the copy trait.
 *
 * Implement the Clone trait when a deep copy is required.
 *
 * All values in Rust are stack allocated by default. Values
 * can be boxed (allocated on the heap) by creating a Box<T>.
 * A box is a smart pointer to heap allocated value of type T.
 * When a box goes out of scope, its desctructor is called,
 * the inner object is destroyed, and the memory on the heap
 * is freed.
 *
 * Conditional compilation
 *
 * all(ConfigurationPredicateList)
 * any(ConfigurationPredicateList)
 * not(ConfigurationPredicate)
 *
 * OOP
 *
 * State Pattern (状态模式) is an object-oriented design pattern.
 * The states are represented by a set of state objects, and
 * the value's behavior changes based on its state.
 */

// --------------------------------------------------------------
