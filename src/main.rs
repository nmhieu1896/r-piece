#![feature(unsize)]
#![feature(trait_upcasting)]

mod ast;
mod errors;
mod evaluator;
mod lexer;
mod parser;
mod repl;
use repl::repl::run_repl;

mod utils;

fn main() {
    defer!(println!("DONE!!!!"));
    run_repl();
}

// A trait defines a unit of behavior.
// In this case, we declare an object that has a name
// trait Named {
//     fn get_name(&self) -> &str;
// }

// // Now we declare a trait that _requires_ Named to be implemented.
// // Named methods will be available to Foo, but this isn't a class inheritance relation
// // in the sense it is on OOP languages. It means that if you have a type that
// // implements Foo, it will also implement Named
// trait Foo: Named {
//     fn say_foo(&self);
// }

// struct Person(String);

// impl Named for Person {
//     fn get_name(&self) -> &str {
//         &self.0
//     }
// }

// impl Foo for Person {
//     fn say_foo(&self) {
//         // Foo gets access to the Named methods
//         println!("{} says foo", self.get_name());
//     }
// }

// // Here we take a type that is a Foo + Named.
// // By default, function arguments must be Sized, but we can add `?Sized` so
// // it also accepts trait objects (static + dynamic dispatch in one)
// fn test1<T: Foo + ?Sized>(x: &T) {
//     println!("Foo: my name is: {}", x.get_name());
//     x.say_foo();
// }
// fn test1_box(x: &Box<dyn Foo>) {
//     println!("Foo: my name is: {}", x.get_name());
//     x.say_foo();
// }

// // In this case, we know the type is Named, but nothing else.
// // Thats because Named isn't a "base class", it's just a Named. Can't use other
// // methods from there. Each trait bound itself is roughly like a base class,
// // so you need to specify all the needed functionality.
// fn test2<T: Named + ?Sized>(x: &T) {
//     println!("Named: my name is {}", x.get_name());
//     //x.say_foo();  // can't call this from here, Foo not implemented
// }

// fn main() {
//     let a = Box::new(Person("John".to_owned()));

//     test1(a.as_ref()); // static dispatch
//     test1(&a as &dyn Foo); // dynamic dispatch
//                            // test1_box(&Box::new(a) as &Box<dyn Foo>); // dynamic dispatch
//                            //test1(&a as &Named); // won't work, Foo not implemented
//     test2(&a as &dyn Named);
// }
