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
 * */ 

fn main() {
    println!("Hello, world!");
}
