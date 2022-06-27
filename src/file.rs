use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct File {
    pub filepath: String,
    pub filename: String,
    pub filecontents: String,
}
impl File {
    pub fn new() -> File {
        File {
            filename: "".to_string(),
            filepath: "".to_string(),
            filecontents: "".to_string(),
        }
    }

    pub fn get(self: &mut Self, args: &[String]) -> Result<(), Box<dyn Error>> {
        let filepath = args[1].clone();
        let filename = Path::new(&filepath.clone())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let filecontents = fs::read_to_string(filepath.clone())?;
        println!("\r\nINPUT contents of filepath: {:?}", filepath.clone());
        self.filename = filename;
        self.filepath = filepath;
        self.filecontents = filecontents;
        Ok(())
    }

    pub fn writefile_or_error(
        self: &Self,
        output: &String,
        is_error: bool,
    ) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        if is_error {
            println!("DIDN'T SAVE");
        } else {
            fs::write("../../src/bin/output.rs", output)?;
            println!("SAVED to '../../src/bin/output.rs'");
        }
        Ok(())
    }
}
