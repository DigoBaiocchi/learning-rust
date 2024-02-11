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
 * */ 

 use crate::garden::vegetables::Asparagus;

 pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
