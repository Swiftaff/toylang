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
}

pub struct DebugFileContents<'a>(pub &'a FileContents);

impl<'a> fmt::Debug for DebugFileContents<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = "".to_string();
        let vec = self.0.split("\r\n");
        for (i, el) in vec.enumerate() {
            let el_debug = format!("{}: {:?}", i, el);
            debug = format!("{}\r\n  {},", debug, el_debug);
        }
        write!(f, "Custom Debug of FileContents{}\r\n", debug)
    }
}

impl File {
    pub fn new() -> File {
        File {
            filename: "".to_string(),
            filepath: "".to_string(),
            filecontents: "".to_string(),
        }
    }

    pub fn get(self: &mut Self, filepath: &str) -> Result<(), Box<dyn Error>> {
        let filename = Path::new(&filepath)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let filecontents = fs::read_to_string(&filepath)?;
        println!("INPUT:  {:?}", &filepath);
        self.filename = filename;
        self.filepath = filepath.to_string().clone();
        self.filecontents = filecontents;
        Ok(())
    }

    pub fn writefile_or_error(
        self: &Self,
        output: &String,
        outputdir: &String,
        is_error: bool,
    ) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        if is_error {
            println!("DIDN'T SAVE");
        } else {
            let current_dir = env::current_dir().unwrap();
            let final_path = current_dir.join(outputdir).join("output.rs");
            fs::write(&final_path, output)?;
            println!("SAVED to {:?}", final_path);
        }
        Ok(())
    }
}
