use std::{fs::File, io::Write};

pub struct Emitter {
    full_path: String,
    header: String,
    code: String,
}

impl Emitter {
    pub fn new(path: String) -> Self {
        Self {
            full_path: path,
            header: String::new(),
            code: String::new(),
        }
    }

    pub fn emit(&mut self, code: &str) {
        self.code.push_str(code);
    }

    pub fn emit_line(&mut self, code: &str) {
        self.code.push_str(&format!["{}\n", code]);
    }

    pub fn header_line(&mut self, code: &str) {
        self.header.push_str(&format!["{}\n", code])
    }

    // this function only needs to be called once, so it consumes self
    // else I would have to clone `self.header` and `self.code`
    pub fn write_file(self) -> std::io::Result<()> {
        let mut file = File::create(&self.full_path)?;
        file.write_all(&self.header.into_bytes())?;
        file.write_all(&self.code.into_bytes())?;

        Ok(())
    }
}
