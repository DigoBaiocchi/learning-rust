/**
 * A crate is the smallest amount of code that the Rust compiler
 * considers at a time. Crates can contain modules and they
 * might be defined in other files that get compiled with the crate.
 * 
 * Crates can be binary and library crates.
 * 
 * Binary crates are programs that you can compile to an executable 
 * that you can run and each must have a main function that defines
 * what is going to run.
 * 
 * Library crates don't have main function so they don't compile 
 * to an executable. They define functionality intended to be shared
 * with multiple projects.
 * 
 * Library crates are usually referred to as crates.
 * 
 * The crate root is a source file that the rust compiler starts from and
 * makes up the root module of your crate. Located in my-project/src/main.rs
 * 
 * A package is a bundle of one or more crates that provides a set of 
 * functionality. It can contain as many binary crates as you like, 
 * but at most only one library crate. Package e.g. Cargo.toml
 * 
 * When compiling a crate, the compiler first looks in the crate root file,
 * looking for a library crate or a binary crate to compile
 * 
 * You can declare new modules in the crate root file. You can declare
 * a "garden" module with mod garden;
 * 
 * In any file other than the crate root, you can declare submodules.
 * 
 * Once a module is part of your crate, you can refer to code in that 
 * module from anywhere else in that crate, as long as the privacy rules allow,
 * using the path to the code. For example, an Asparagus in the garden module
 * would be found at crate::garden::vegetable::Asparagus.
 * 
 * Code within a module is private from its parent modules by default.
 * If you want to make it public, declare it with pub mod instead of just mod.
 * To make items within a module public as well, use pub before their declarations.
 * 
 * The use keyword creates shortcuts to item to avoid repetition. In a scope you can do the following:
 * use crate::garden::vegetable::Asparagus
 * And from there you only need to refer to this module as Asparagus in the scope
 * 
 * Modules are private by default.
 * If a module A is contained inside module B, we say that module A is the child of module B
 * and module B is the parent of module A.
 * 
 * To call a function we need to know its path.
 * A path can take two forms.
 * Absolute path: full path starting from the crate, it starts with the literal crate.
 * Relative path: starts from the current module and uses self, super or an identifier
 * in the current module.
 * Both paths are followed by a ::.
 * 
 * Choosing whether to use a relative or absolute path is a decision you'll make based
 * on your project and depends if you are more likely to move item definion code separately from
 * or together with the code that uses the item.
 * 
 * The privacy rules apply to structs, enums, functions, methods and modules.
 * 
 * We can user super to define a relative path, it works as .. is used in the start of a filestream command.
 * This makes the rearranging of the module tree easier when the module is closely related to
 * the parent but the parent mightt be moved elsewhere in the module tree someday.
 * 
 * In structs, we can make the struct public but choose which of the struct's fields should be kept private.
 * If we have a private field in a public struct, the struct needs to provide a fublic associated function
 * that constructs an instance of the struct (see restaurant project, summer function is defined in Breakfast struct).
 * 
 * If we make an enum public, all of its variants are then public. We only need the pub before the enum keyword.
 * 
 * We can use the keyword use to use the last module specified, e.g. use crate::front_of_house::hosting and now
 * we only need to use hosting across our code to access what is inside hosting.
 * 
 * use only creates the shortcut for the particular scope in which the use occurs.
 * 
 * The idiomatic way to use the keyword use is specifying the path up to the function's module parent.
 * It is clear that the function belongs to a different module.
 * 
 * However, when we are bringing in structs, enums and other items with use, it is idiomatic
 * to specify the full path.
 * The only exception for this rule is if we are trying to bring two items that have they share
 * the same name.
 * An alternative for this situation is to use the keyword as. If we have two path like:
 * std::fmt::Result;
 * std::io::Result;
 * We can rename one of them using the keyword as 
 * use std::fmt::Result;
 * use std::io::Result as IoResult;
 * 
 * When we bring a name into scope with the use keyword, the name available in 
 * the new scope is private. In order to make the name available to external code
 * we would have to add pub before the use keyword. This is called re-exporting.
 * 
 * use is also used to make available packages that are declared in Cargo.toml.
 * And also rust library like std.
 * 
 * if we are using multiple items defined in the same crate we can group them between {}.
 * The following:
 * use std::cmp::Ordering;
 * use std::io;
 * would be like:
 * use std::{cmp::Orderng, io};
 * We can use a nested path at any level in a path, which is useful when combining two
 * use statements that share a subpath.
 * The following,
 * use std::io;
 * use std::io::Write;
 * would be,
 * use std::io::{self, Write};
 * We use self to merge the two paths into one use statement.
 * 
 * If we want to bring all public items defined in a path into scope, we can specify that
 * path followed by the * glob operator.
 * use std::collections::*;
 * All items definied in collections will be brought to current scope.
 * Glob operator is usually used under the test module.
 * */ 

 use crate::garden::vegetables::Asparagus;

 pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
