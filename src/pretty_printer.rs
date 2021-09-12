use crate::Function;
use colored::*;
use pad::{Alignment, PadStr};

pub fn print_functions(functions: Vec<Function>, script: &String) {
    if functions.len() == 0 {
        println!(
            "{} has found no functions in {}. You could add some like this:",
            "Runsh",
            script.bright_blue()
        );
        let example_function = r#"# Some great comment
# More insightful and fascinating insights into bash scripting
blow_mind() {
    echo "OMG so cool"
} "#;
        println!("{}", example_function.green());
    } else {
        let example_command = format!("./{} {}", script, functions[0].name);
        print!("{}", script.on_blue());
        println!(" - Usage: {}\n", example_command.blue());

        // Get the longest function name
        const INDENT: usize = 2;
        let padding = functions
            .iter()
            .max_by(|x, y| x.name.len().cmp(&y.name.len()))
            .unwrap()
            .name
            .len()
            + INDENT;
        for function in functions {
            // We'll pad right so everything aligns nicely.
            // First print the function name
            let to_print = function
                .name
                .pad_to_width_with_alignment(padding, Alignment::Right)
                .green();
            if function.comment.len() > 0 {
                print!("{}", to_print);
            } else {
                println!("{}", to_print);
            }

            // Then follow up with the comment lines
            function.comment.iter().enumerate().for_each(|(i, line)| {
                if i == 0 {
                    println!(" {}", line);
                } else {
                    println!(
                        "{} {}",
                        "".pad_to_width_with_alignment(padding, Alignment::Right),
                        line
                    );
                }
            });
        }
    }
}
