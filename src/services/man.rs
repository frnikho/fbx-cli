/*use man::prelude::*;

pub struct Man;

impl Man {
    pub fn new() -> Manual {
        Manual::new("basic")
            .about("A basic example")
            .author(Author::new("Alice Person").email("alice@person.com"))
            .author(Author::new("Bob Human").email("bob@human.com"))
            .flag(
                Flag::new()
                    .short("-d")
                    .long("--debug")
                    .help("Enable debug mode"),
            )
            .flag(
                Flag::new()
                    .short("-v")
                    .long("--verbose")
                    .help("Enable verbose mode"),
            )
            .option(
                Opt::new("output")
                    .short("-o")
                    .long("--output")
                    .help("The file path to write output to"),
            )
            .example(
                Example::new()
                    .text("run basic in debug mode")
                    .command("basic -d")
                    .output("Debug Mode: basic will print errors to the console")
            )
            .custom(
                Section::new("usage note")
                    .paragraph("This program will overwrite any file currently stored at the output path")
            )
    }
}*/
