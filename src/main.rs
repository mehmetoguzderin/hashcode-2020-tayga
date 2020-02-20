use rayon::prelude::*;

fn main() {
    let a_example = include_str!("data/a_example.txt");
    let b_read_on = include_str!("data/b_read_on.txt");
    let c_incunabula = include_str!("data/c_incunabula.txt");
    let d_tough_choices = include_str!("data/d_tough_choices.txt");
    let e_so_many_books = include_str!("data/e_so_many_books.txt");
    let f_libraries_of_the_world = include_str!("data/f_libraries_of_the_world.txt");
    let problem_name;
    let problem;

    let args: Vec<String> = std::env::args().collect();

    if args[1] == "a_example" {
        // a_example
        // cargo run --release -- a_example
        problem_name = String::from("a_example");
        problem = String::from(a_example);
    } else if args[1] == "b_read_on" {
        // b_read_on
        // cargo run --release -- b_read_on
        problem_name = String::from("b_read_on");
        problem = String::from(b_read_on);
    } else if args[1] == "c_incunabula" {
        // c_incunabula
        // cargo run --release -- c_incunabula
        problem_name = String::from("c_incunabula");
        problem = String::from(c_incunabula);
    } else if args[1] == "d_tough_choices" {
        // d_tough_choices
        // cargo run --release -- d_tough_choices
        problem_name = String::from("d_tough_choices");
        problem = String::from(d_tough_choices);
    } else if args[1] == "e_so_many_books" {
        // e_so_many_books
        // cargo run --release -- e_so_many_books
        problem_name = String::from("e_so_many_books");
        problem = String::from(e_so_many_books);
    } else if args[1] == "f_libraries_of_the_world" {
        // f_libraries_of_the_world
        // cargo run --release -- f_libraries_of_the_world
        problem_name = String::from("f_libraries_of_the_world");
        problem = String::from(f_libraries_of_the_world);
    } else {
        return;
    }

    println!("Problem Name: {}", problem_name);
    let mut problem_lines: Vec<String> =
        problem.split("\n").map(|line| String::from(line)).collect();
    for line in &mut problem_lines {
        if line.ends_with("\r") {
            line.pop().unwrap();
        }
    }
    while problem_lines.last().unwrap().trim().len() == 0 {
        println!("Problem Lines Pop: {}", problem_lines.pop().unwrap());
    }
    println!("Problem Lines Length: {}", problem_lines.len());

    let first_line: Vec<String> = problem_lines[0]
        .split(" ")
        .map(|token| String::from(token))
        .collect();
    let number_of_different_books = first_line[0].parse::<i32>().unwrap();
    let number_of_libraries = first_line[1].parse::<i32>().unwrap();
    let number_of_days = first_line[2].parse::<i32>().unwrap();

    println!(
        "There are {} books, {} libraries, and {} days for scanning.",
        number_of_different_books, number_of_libraries, number_of_days
    );

    let scores: Vec<i32> = problem_lines[1]
        .split(" ")
        .map(|token| token.parse::<i32>().unwrap())
        .collect();

    println!("The scores of the books are {} (in order).", scores[0]);

    let mut number_of_books_in_library = Vec::new();
    let mut number_of_days_in_library = Vec::new();
    let mut number_of_shipments_in_library = Vec::new();
    let mut books_in_library: Vec<Vec<i32>> = Vec::new();

    for library in 0..number_of_libraries {
        let library_line: Vec<String> = problem_lines[(2 + library * 2) as usize]
            .split(" ")
            .map(|token| String::from(token))
            .collect();
        number_of_books_in_library.push(library_line[0].parse::<i32>().unwrap());
        number_of_days_in_library.push(library_line[1].parse::<i32>().unwrap());
        number_of_shipments_in_library.push(library_line[2].parse::<i32>().unwrap());

        books_in_library.push(
            problem_lines[(2 + library * 2 + 1) as usize]
                .split(" ")
                .map(|token| token.parse::<i32>().unwrap())
                .collect(),
        );
    }

    println!("Library {} has {} books, the signup process takes {} days, and the library can ship {} books per day.", 0, number_of_books_in_library[0], number_of_days_in_library[0], number_of_shipments_in_library[0]);
    println!("The books in library {} are: book {}.", 0, books_in_library[0][0]);

    
    let mut scores_of_libraries: Vec<(i32, i32, Vec<i32>)> = (0..number_of_libraries).map(|index| (index, 0, Vec::new())).collect();
    scores_of_libraries.par_iter_mut().for_each(|(library, score, books)| {
        *score = 100;
    });
}
