/*****************************************
 * File Name: movies.rs
 * Date: 11/8/24
 * File Description: Movies module for
 *                   Rust verison of Movies
 * Author(s): Ivan Wong
 *****************************************/
 

use std::fmt::{self};
use std::fs::File;
use std::io::{self, Write, BufRead};
use std::path::Path;
use crate::list::List;
use crate::list::Node;


/***************
 * Movie Struct
 ***************/
pub struct Movie {
    pub title: String,
    pub year: i32,
    pub languages: List<String>,
    pub rating: f32,
}


/************************
 * Movie Implementations
 ************************/
impl Movie {
    fn new(title: String, year: i32, languages: List<String>, rating: f32) -> Movie {
        Movie {
            title,
            year,
            languages,
            rating,
        }
    }

    fn create_lang(languages: &str) -> List<String> {
        // Tokenize languages string
        let tokens: Vec<&str> = languages.split(";").collect();
        let mut lang_list: List<String> = List::new();

        // Iterate through tokens
        for mut token in tokens {
            // Remove brackets
            if token.contains("[") {
                token = token.trim_start_matches('[');
            }
            if token.contains("]") {
                token = token.trim_end_matches(']');
            }
            // Add language string to list
            let language: String = token.parse::<String>().unwrap();
            lang_list.add_list(language);
        }
        lang_list
    }

    fn create_movie(line: String) -> Movie {
        // Tokenize line
        let tokens: Vec<&str> = line.split(",").collect();

        // Parse data
        let title: String = tokens[0].parse::<String>().unwrap();
        let year: i32 = tokens[1].parse::<i32>().unwrap();
        let languages: List<String> = Movie::create_lang(tokens[2]);
        let rating: f32 = tokens[3].parse::<f32>().unwrap();

        // Create and return new movie
        let new_movie: Movie = Movie::new(title, year, languages, rating);

        new_movie
    }
}


// Format for printing Movie value
impl fmt::Display for Movie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} {} {}", self.year, self.rating, self.title)
    }
}


// Code from Official Rust Document, read file using a buffer, return iterator of lines of file
fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(file_name)?;
        Ok(io::BufReader::new(file).lines())
    }


pub fn proccess_file(movies: &mut List<Movie>, file_name: &str) {
    // Check for valid line in file
    if let Ok(lines) = read_lines(file_name) {
        // Read overy line in file, except for first
        for line in lines.flatten().skip(1) {
            let new_movie: Movie = Movie::create_movie(line);

            // Add new movie to list
            movies.add_list(new_movie);
        }
    }
}


pub fn display_options() {
    println!("\n1. Show movies released in specified year.");
    println!("2. Show highest rated movie for each year.");
    println!("3. Show the title and year of release of all movies in a specific language.");
    println!("4. Exit program");
    print!("\nEnter 1 to 4: ");

    io::stdout().flush().unwrap();
}


pub fn process_options(movies: &mut List<Movie>) -> bool{
    let mut user_input: String = String::new();

    // Get user input
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    // Convert input to integer
    let choice: Result<i32, std::num::ParseIntError> = user_input.trim().parse::<i32>();

    // Match input and option
    match choice {
        Ok(1) => {
            movies_in_year(movies)
        }
        Ok(2) => {
            movies_highest_rated(movies)
            
        }
        Ok(3) => {
            movies_in_language(movies)
        }
        Ok(4) => {
            false 
        }
        _ => {
            println!("\nInvalid Input, Try again");
            true
        }
    }
}


fn contains_language(lang_list: &List<String>, language: &String) -> bool {
    let mut temp: &Option<Box<Node<String>>> = &lang_list.head;

    // Traverse language list
    while let Some(node) = temp {
        // Found
        if node.val == *language {
            true
        }
        temp = &node.next;
    }
    // Not found
    false
}


fn contains_year(years_list: &List<i32>, check_year: &i32) -> bool {
    let mut temp: &Option<Box<Node<i32>>> = &years_list.head;

    while let Some(node) = temp {
        if node.val == *check_year {
            true
        }
        temp = &node.next;
    }
    false
}


fn movies_in_year(movies: &mut List<Movie>) -> bool {
    let mut user_input: String = String::new();

    print!("Enter the year for which you want to see movies: ");

    // Flush stdout stream and get user input
    io::stdout().flush().unwrap();
    io::stdin().read_line( &mut user_input).expect("Failed to read line");

    // Convert input to 32-bit int
    let selected_year: Result<i32, std::num::ParseIntError> = user_input.trim().parse::<i32>();
    let mut temp: &Option<Box<Node<Movie>>> = &movies.head;
    let mut flag: bool = true;

    // Traverse list
    while let Some(node) = temp {
        match selected_year {
            Ok(year) => {
                // Matching year, print movie data
                if year == node.val.year {
                    print!("{}", node.val);
                    flag = false;
                }
            }
            Err(_) =>  {
                //Error handle
                println!("Failed to parse input");
                true
            }
        }
        temp = &node.next;
    }
    // No movies found, special message
    if flag == true {
        println!("No data about movies released in the year {}", selected_year.unwrap());
    }
    true
}


fn movies_highest_rated(movies: & List<Movie>) -> bool {
    let mut cur_node: &Option<Box<Node<Movie>>> = &movies.head;
    let mut passing_node: &Option<Box<Node<Movie>>>;
    let mut highest_rated_movie: &Movie;
    let mut check_movie: &Movie;
    let mut checked_years: List<i32> = List::new();

    // Traverse movies
    while let Some(node) = cur_node {
        passing_node = &node.next;

        // Year not checked
        if contains_year(&checked_years, &node.val.year) == false {
            let new_year: i32 = node.val.year;

            // Add year to list
            checked_years.add_list(new_year);

            highest_rated_movie = &node.val;

            // Traverse movies again
            while let Some(pass) = passing_node {
                check_movie = &pass.val;

                // Find higher rated movie in same year
                if new_year == check_movie.year  && check_movie.rating > highest_rated_movie.rating {
                    highest_rated_movie = &pass.val;
                }
                passing_node = &pass.next;
            }
            print!("{}", highest_rated_movie);
        }
        cur_node = &node.next;
    }
    true
}


fn movies_in_language(movies: &mut List<Movie>) -> bool {
    let mut user_input: String = String::new();

    print!("Enter the language for which you want to see movies: ");

    // Flush stdout stream and get user input
    io::stdout().flush().unwrap();
    io::stdin().read_line( &mut user_input).expect("Failed to read line");

    let mut temp: &Option<Box<Node<Movie>>> = &movies.head;
    let mut flag: bool = true;

    // Parse user input
    user_input = user_input.trim().to_string();

    // Traverse movies list
    while let Some(node) = temp {
        // Print movie data if matching language found
        if contains_language(&node.val.languages, &user_input) == true {
            print!("{}", node.val);
            flag = false;
        }
        temp = &node.next;
    }
    // No movie found, special message
    if flag == true {
        println!("No data about movies released in {}", user_input);
    }
    true
}
