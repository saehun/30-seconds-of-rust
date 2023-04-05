use clap::{App, Arg};
use regex::Regex;
use std::io::BufRead;

fn main() {
    let args = App::new("grep-lite")
        .version("0.1.0")
        .about("Search for a pattern in a file and display the lines that contain it.")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("The input file")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let reader = get_reader_from_file_or_stdin(args.value_of("input"));
    process_lines(reader, re);
}

fn get_reader_from_file_or_stdin(maybe_input: Option<&str>) -> Box<dyn BufRead> {
    match maybe_input {
        Some(input) => {
            let file = std::fs::File::open(input).unwrap();
            let reader = std::io::BufReader::new(file);
            Box::new(reader)
        }
        None => {
            let stdin = std::io::stdin();
            let handle = stdin.lock();
            Box::new(handle)
        }
    }
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use tempfile::NamedTempFile;

    #[test]
    fn test_get_reader_from_file() {
        // Create a temporary file with some content.
        let mut tmpfile = NamedTempFile::new().unwrap();

        writeln!(tmpfile, "Hello, World!").unwrap();

        // Get a reader for the temporary file.
        let path = tmpfile.path().to_str();
        let mut reader = get_reader_from_file_or_stdin(path);

        // Read the content from the reader.
        let mut contents = String::new();
        reader.read_to_string(&mut contents).unwrap();

        // Check if the content is correct.
        // cargo insta test --review
        // second argument should be @""
        insta::assert_debug_snapshot!(contents, @r###""Hello, World!\n""###);
    }

    // #[test]
    // fn test_get_reader_from_stdin() {
    //     // Use the `duct` crate to pipe data into the program.
    //     let input = "Hello, stdin!";
    //     let output = duct::cmd!(
    //         "cargo",
    //         "run",
    //         "--quiet",
    //         "--",
    //         "-",
    //         stdin_bytes(input.as_bytes())
    //     )
    //     .stdout_capture()
    //     .unchecked()
    //     .run()
    //     .unwrap();

    //     // Check if the captured output is correct.
    //     assert_eq!(output.stdout_str(), input);
    // }

    // Helper function to create `duct::Expression` for stdin data.
    fn stdin_bytes(bytes: &[u8]) -> duct::Expression {
        duct::cmd!("cat").stdin_bytes(bytes.to_vec())
    }
}
