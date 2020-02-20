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

    if (args[1] == "a_example") {
        // a_example
        // cargo run --release -- a_example
        problem_name = String::from("a_example");
        problem = String::from(a_example);
    } else if (args[1] == "b_read_on") {
        // b_read_on
        // cargo run --release -- b_read_on
        problem_name = String::from("b_read_on");
        problem = String::from(b_read_on);
    } else if (args[1] == "c_incunabula") {
        // c_incunabula
        // cargo run --release -- c_incunabula
        problem_name = String::from("c_incunabula");
        problem = String::from(c_incunabula);
    } else if (args[1] == "d_tough_choices") {
        // d_tough_choices
        // cargo run --release -- d_tough_choices
        problem_name = String::from("d_tough_choices");
        problem = String::from(d_tough_choices);
    } else if (args[1] == "e_so_many_books") {
        // e_so_many_books
        // cargo run --release -- e_so_many_books
        problem_name = String::from("e_so_many_books");
        problem = String::from(e_so_many_books);
    } else if (args[1] == "f_libraries_of_the_world") {
        // f_libraries_of_the_world
        // cargo run --release -- f_libraries_of_the_world
        problem_name = String::from("f_libraries_of_the_world");
        problem = String::from(f_libraries_of_the_world);
    } else {
        return
    }

    println!("Problem Name: {}", problem_name);
}
