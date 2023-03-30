use indexmap::IndexMap;
use std::io::Write;
use std::path::{Path, PathBuf};
pub struct SorseHeader {
    pub path: PathBuf,
    pub defines: IndexMap<String, Option<String>>,
    pub functions: IndexMap<String, String>,
    pub structs: IndexMap<String, String>,
}

impl SorseHeader {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            defines: IndexMap::new(),
            functions: IndexMap::new(),
            structs: IndexMap::new(),
        }
    }

    pub fn define<T: ToString>(&mut self, name: String, value: impl Into<Option<T>>) -> &mut Self {
        self.defines
            .insert(name, value.into().map(|x| x.to_string()));
        self
    }

    fn _function<T: ToString>(&mut self, name: String, value: T) -> &mut Self {
        self.functions.insert(name, value.to_string());
        self
    }

    fn _struct_<T: ToString>(&mut self, name: String, value: T) -> &mut Self {
        self.structs.insert(name, value.to_string());
        self
    }

    pub fn write(&self) -> std::io::Result<()> {
        let mut file = std::fs::File::create(&self.path)?;
        let mut buf_file = std::io::BufWriter::new(&mut file);
        self.write_to(&mut buf_file)?;
        Ok(())
    }

    pub fn write_to(&self, mut writer: impl Write) -> std::io::Result<()> {
        for (name, value) in &self.defines {
            if let Some(value) = value {
                writer.write_all(format!("#define {} {}\n", name, value).as_bytes())?;
            } else {
                writer.write_all(format!("#define {}\n", name).as_bytes())?;
            }
        }
        Ok(())
    }
}
