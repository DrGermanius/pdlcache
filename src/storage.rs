use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, Write},
};

pub struct Storage {
    file: File,
}

impl Storage {
    pub fn new(file_path: String) -> Storage {
        let f: File = match OpenOptions::new().write(true).read(true).open(file_path) {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        Storage { file: f }
    }

    pub fn load(&mut self) -> Result<String, std::io::Error> {
        let mut buf = String::new();
        self.file.read_to_string(&mut buf)?;
        Ok(buf)
    }

    pub fn save(&mut self, payload: &[u8]) -> std::io::Result<()> {
        self.file.rewind()?;
        self.file.write_all(payload)?;
        self.file.flush()?;
        Ok(())
    }
}

impl Default for Storage {
    fn default() -> Self {
        // TODO: create file first
        let f: File = match OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("./cache.json")
        {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        Storage { file: f }
    }
}
