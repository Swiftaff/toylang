/*! Stores information about the input File, for use in the Compiler
 */
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
extern crate base64;
use base64::{engine::general_purpose, Engine as _};

type FileContents = String;

/// filepath, filename, filecontents, nosave.
///
/// nosave is a flag to avoid saving the output file on compilation.
/// It's handy when you know the compiler could generate invalid rust in the output.rs file, e.g. from an incorrect input file,
/// and thus could stop the main toylang crate from compiling the next time,
/// which would otherwise need to be fixed manually in the output.rs file!
#[derive(Clone, Debug, Default)]
pub struct File {
    pub filepath: String,
    pub filename: String,
    pub filecontents: FileContents,
    pub nosave: bool,
}

pub struct DebugFileContents<'a>(pub &'a FileContents);

impl<'a> fmt::Debug for DebugFileContents<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = "".to_string();
        let vec = self.0.split("\r\n");
        for (i, el) in vec.enumerate() {
            debug = format!("{}\r\n  {}: {},", debug, i, el);
        }
        write!(f, "Custom Debug of FileContents{}\r\n", debug)
    }
}

impl File {
    /// Initialise a new File Struct with defaults
    pub fn new(nosave: bool) -> File {
        File {
            filename: "".to_string(),
            filepath: "".to_string(),
            filecontents: "".to_string(),
            nosave,
        }
    }

    /// Get filename, path, contents from the users supplied filepath
    pub fn get(
        self: &mut Self,
        filepath: &str,
        tokens: bool,
        code: bool,
    ) -> Result<(), Box<dyn Error>> {
        if !tokens {
            println!("INPUT:  {:?}", &filepath);
        }
        self.filepath = filepath.to_string().clone();
        if code {
            self.filename = "".to_string();
            let bytes = general_purpose::STANDARD.decode(filepath).unwrap();
            self.filecontents = String::from_utf8_lossy(&*bytes).to_string(); // unsure if good enough
        } else {
            let filename = Path::new(&filepath)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            self.filename = filename;
            self.filecontents = fs::read_to_string(&filepath)?;
        };

        Ok(())
    }

    /// Write the output file, except if nosave is true or if there are any compilation errors
    pub fn writefile_or_error(
        self: &Self,
        output: &String,
        outputdir: &String,
        is_error: bool,
        tokens: bool,
    ) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        if self.nosave {
            if !tokens {
                println!("-n / -nosave flag is true - DIDN'T SAVE");
            }
        } else {
            if is_error {
                if !tokens {
                    println!("Error saving - DIDN'T SAVE");
                }
            } else {
                let current_dir = env::current_dir().unwrap();
                let final_path = current_dir.join(outputdir).join("output.rs");
                fs::write(&final_path, output)?;
                if !tokens {
                    println!("SAVED to {:?}", final_path);
                }
            }
        }
        Ok(())
    }
}
