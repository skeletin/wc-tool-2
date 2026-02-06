use std::fs;
use std::env;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    // Get the arguements from the commandline
    let args: Vec<String> = env::args().collect();

    // Initialize CCWC FileReader 
    let ccwc_file_reader = CCWCFileReader::new(args);

    // Output to std the results
    ccwc_file_reader.log();
}
struct CCWCFileReader {
   files: Vec<CCWCFile>,
   opts_map: HashMap<char, bool>,
   total_bytes: usize,
   total_chars: usize,
   total_words: usize,
   total_lines: usize,
}
struct CCWCFile {
    name: String,
    bytes: usize,
    chars: usize,
    words: usize,
    lines: usize,
}

impl CCWCFileReader {
    fn new(args: Vec<String> ) -> CCWCFileReader {
        let (options, files) = Self::parse_args(args);

        let mut reader = CCWCFileReader { files: Vec::new(), opts_map: HashMap::from([
            ('c', false),
            ('l', false),
            ('w', false),
            ('m', false)
            ]),
            total_bytes: 0,
            total_chars: 0,
            total_words: 0,
            total_lines: 0,
        };

        for option in options {
            if let Err(err) = Self::validate_option(option, &mut reader) {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        }

        for file in files {
            Self::validate_file(file, &mut reader);
        }

        reader
    }

    fn parse_args(args: Vec<String>) -> (Vec<String>, Vec<String>) {
        let mut end_of_options = false;
        let(mut options, mut files ) = (vec![], vec![]);

        for arg in args.iter().skip(1) {
            if arg.starts_with("-") && !end_of_options {
                options.push(arg.clone());
            } else {
                end_of_options = true;
                files.push(arg.clone());
            }
        }

        (options, files)
    }

    fn validate_option(option: String, reader: &mut CCWCFileReader) -> Result<(), String>{
        let valid_options: [char; 4]  = ['c', 'l', 'w', 'm'];
        for c in option.chars().skip(1) {
            if valid_options.contains(&c) {
                reader.opts_map.insert(c, true);
            } else {
                return Err(format!("ccwc: illegal option -- {}\nusage: ccwc [-clmw] [file ...]", c))
            }
        }
        Ok(())
    }

    fn validate_file(file: String, reader: &mut CCWCFileReader ) {      
        let path_obj: &Path = Path::new(&file);
        if path_obj.is_file() {
            reader.process_file(file);
        } else {
            println!("ccwc: {}: open: No such file or directory", file);
        }
    }

    fn process_file(&mut self, path_to_file: String) {
        let content = fs::read(&path_to_file)
                .expect("Failed to read File");
    
        let bytes = content.len();
        let content_str = String::from_utf8_lossy(&content);

        let mut chars = 0;
        let mut lines = 0;
        let mut words = 0;
        let mut in_word = false;


        for c in content_str.chars() {
            chars += 1;
            if c == '\n' {
                lines += 1;
            }
            if !c.is_whitespace() && !in_word {
                words += 1;
                in_word = true;
            } else if c.is_whitespace() {
                in_word = false;
            }
        }

        self.total_bytes += bytes;
        self.total_chars += chars;
        self.total_words += words;
        self.total_lines += lines;

        self.files.push(CCWCFile { name: path_to_file, bytes, chars, words, lines });
    }

    fn log(&self) {
        // Which flags are active
        let show_lines = self.opts_map.get(&'l').copied().unwrap_or(false);
        let show_words = self.opts_map.get(&'w').copied().unwrap_or(false);
        let show_bytes = self.opts_map.get(&'c').copied().unwrap_or(false);
        let show_chars = self.opts_map.get(&'m').copied().unwrap_or(false);

        // Show total if more than 1 file
        let show_total = self.files.len() > 1;

        // No flags → default to lines, words, bytes
        let no_flags = !show_lines && !show_words && !show_chars && !show_bytes;

        for file in &self.files {
            if no_flags || show_lines {
                print!("{:>8} ", file.lines);
            }
            if no_flags || show_words {
                print!("{:>8} ", file.words);
            }
            if no_flags || show_bytes {
                print!("{:>8} ", file.bytes);
            }
            if show_chars {
                print!("{:>8} ", file.chars);
            }

            println!("{}", file.name);
        }

        // Print total if more than 1 file
        if show_total {
            if no_flags || show_lines {
                print!("{:>8} ", self.total_lines);
            }
            if no_flags || show_words {
                print!("{:>8} ", self.total_words);
            }
            if no_flags || show_bytes {
                print!("{:>8} ", self.total_bytes);
            }
            if show_chars {
                print!("{:>8} ", self.total_chars);
            }

            println!("total");
        }
    }
}


