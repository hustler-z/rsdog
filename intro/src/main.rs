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
 *	  let ref_0; -------------------------------+
 *	  {                                         |
 *	      let num_0 = 2; ----+                  |
 *		  ref_0 = &num_0;    | 'a for num_0     | 'b for ref_0
 *	  } ---------------------+                  |
 *	  println!("{}", ref_0);                    |
 * } -------------------------------------------+
 *
 * Lifetime Elision
 *
 * a) Each input lifetime that is elided is assigned a separate
 *    lifetime.
 * b) When there is a single input lifetime, it is elided to the
 *    output lifetime.
 * c) For methods, if self is a reference, the lifetime for self
 *    is elided to the output lifetimes.
 *
 * Lifetime sharing
 *
 * A static lifetime, 'static, spans the entire application.
 *
 * Subtyping Lifetime creates an extensible relationship
 * between two lifetimes.
 * 'subtype: 'basetype
 *
 * Anonymous Lifetime ??
 * '_
 *
 * --------------------------------------------------------------
 *
 * Rust has two types of pointers: safe pointers and raw pointers.
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

/* An attribute is metadata applied to some module, crate or
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
 * trait - interface
 *         A collection of methods defined for an unknown
 *         type: Self. Traits can be implemented for any
 *         data type.
 *
 * Supertraits
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
		 */
		_ => println!("step n"),
	}
}

/* Guard
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

/* Nested Functions in Rust
 *
 * A private function of the outer function.
 *
 * Function Pointers
 *
 * Function Alias
 */
fn test_fp(x: u32) -> u32 {
	x * x
}

/* Never Type
 *
 */

// #[allow(unused)]

/* The loop label - can break or continue in an outer loop.
 * 'label: while
 * 'label: for
 * 'label: loop
 */
fn loop_thru() {
	let mut outer_cnt: u8 = 0;
	'loopto: while outer_cnt < 10 {
		outer_cnt += 1;
		let mut inner_cnt: u8 = 1;
		println!();

		loop {
			print!("{} ", inner_cnt);
			if inner_cnt >= outer_cnt {
				continue 'loopto;
			}
			inner_cnt += 1;
		}
	}

	println!();
}

/* Collections - Array, Vector, HashMap
 */
fn test_collections() {
	/* Array */
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

	/* enum Option<T> {
	 *     None,           - Indicate failure/lack of value
	 *     Some(T),        - a tuple struct that wraps a
	 * }                     value with type T.
	 */
	match result {
		Some(value) => println!("Found {}", value),
		None => println!("Not Found"),
	}

	/* Whereas Result<T, E> enum:
	 * a) Ok(value)        - Indicate operation succeeded,
	 *                       and wrap the value returned by
	 *                       operation.
	 * b) Err(why)         - Indicate operation failed, and
	 *                       wrap why which hopefully explained.
	 *
	 * ? used at the end of an expression returning a Result.
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
macro_rules! vec_simple {
	($( $x:expr ), *) => {
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
 *     impl<T>   ATrait<T> for XStruct<T> {...}
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

// --------------------------------------------------------------

/* test Modules
 */
#[cfg(test)]
mod tests {
	// use super::*;

	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
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
 */
mod hustler {

}

/* --------------------------------------------------------------
 * Threads  - Native OS threads via the spawn function.
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

static NR_THREADS: u32 = 10;
static NR_THRDS: u32 = 5;
static TOML_PATH: &'static str = "Cargo.toml";
static TAG_BEST: &'static str =
"the best out of the best!!";

fn test_threads() {
	let mut children = vec![];

	for i in 0..NR_THREADS {
		children.push(thread::spawn(move || {
			println!("This is thread number {}", i)
		}));
	}

	for child in children {
		let _ = child.join(); // block 'til all threads finished.
	}

	/* Test on Channels
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

/**
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
 * RefCell<T>
 */

// --------------------------------------------------------------

fn main() {
    let var_0 = 65 as char;
    let var_1 = 'A' as u32;
	let value: u32 = 100;

	/* Type Casting - "as"
	 *
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
	 * +---------------+
	 * | backing array |
	 * +---------------+
	 * |    length     |
	 * +---------------+
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

	let tuple_0 = (10, "ten");

	let tuple_1 = (12, 13);

	/* Function Alias
	 *
	 * The 'type' statement can be used to give a new name to
	 * an existing type. Types must have UpperCamelCase names,
	 * or the compiler will raise a warning.
	 */
	type Funcptr = fn(u32) -> u32;
	/* Function pointer */
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

    println!("Decimal {}, Binary {:b}, Octal {:o}, Hexadecimal {:x}",
             data_0, data_0, data_0, data_0);

    println!("{}", format!("{} is a {} (capacity {} len {}) {}",
        s_0, s_1, s_1.capacity(), s_1.len(), s_2.to_uppercase()));

    iter_str(&s_1);

    println!("u32 [{}, {}]", u32::MIN, u32::MAX);

    println!("Type casting: {} -> {}", var_0, var_1);

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


    // interact_console();
    loop_thru();

	println!();

	test_collections();

	let rock = Person("Rock Lo".to_string(), 30,
					  "Programmer".to_string());

	println!("Name: {}, Age: {}, Job: {}",
			 rock.0, rock.1, rock.2);

	/* Binding values to names - @
	 *
	 * In match expressions, you can match multiple patterns
	 * using the | syntax, which is the pattern or operator.
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

	test_unsafety();

	println!("---------------------------------");

	test_threads();

	println!("---------------------------------");
}

// --------------------------------------------------------------

/* Resource Acquisition is Initialization (RAII paradigm)
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
