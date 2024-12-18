/*****************************************
 * File Name: main.rs
 * Date: 11/8/24
 * File Description: Driver/main file for
 *                   Rust version of Movies
 * Author(s): Ivan Wong
 *****************************************/


mod list;
mod movies;


use std::env;
use std::fs::File;
use list::List;
use movies::{display_options, proccess_file, process_options, Movie};


fn main() {
    println!("\nThis is extra credit assignment 1. Rust Version of Movies\n");

    let args: Vec<String> = env::args().collect();

    // Error check command line arguments
    if args.len() < 2 {
        println!("You must provide the name of the file to process");
        println!("Example usage: cargo run -- movies_example.txt\n");
        return
    }
    // Error check opening file
    let _file = match File::open(&args[1]) {
        Ok(file) => file,
        Err(_) => panic!("Failed to open file, gracefully exiting...\n"),
    };
    let mut movies: List<Movie> = List::new();

    proccess_file(&mut movies, &args[1]);

    println!("Processed File {} and parsed data for {} movies", args[1], movies.len());

    // Program loop
    loop {
        display_options();

        if process_options(&mut movies) == false {
            break
        }
    }
    println!("\nExiting program...");
    return
}
