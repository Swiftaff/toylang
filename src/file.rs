use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

type FileContents = String;

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
    pub fn new(nosave: bool) -> File {
        File {
            filename: "".to_string(),
            filepath: "".to_string(),
            filecontents: "".to_string(),
            nosave,
        }
    }

    pub fn get(self: &mut Self, filepath: &str) -> Result<(), Box<dyn Error>> {
        let filename = Path::new(&filepath).file_name().unwrap().to_str().unwrap().to_string();
        let filecontents = fs::read_to_string(&filepath)?;
        println!("INPUT:  {:?}", &filepath);
        self.filename = filename;
        self.filepath = filepath.to_string().clone();
        self.filecontents = filecontents;
        Ok(())
    }

    pub fn writefile_or_error(self: &Self, output: &String, outputdir: &String, is_error: bool) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        if self.nosave {
            println!("-n / -nosave flag is true - DIDN'T SAVE");
        } else {
            if is_error {
                println!("DIDN'T SAVE");
            } else {
                let current_dir = env::current_dir().unwrap();
                let final_path = current_dir.join(outputdir).join("output.rs");
                fs::write(&final_path, output)?;
                println!("SAVED to {:?}", final_path);
            }
        }
        Ok(())
    }
}
