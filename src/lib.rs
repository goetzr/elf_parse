use byteorder::ReadBytesExt;
use std::fmt;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;

const EI_NIDENT: usize = 16;
const ELF_MAG0: u8 = 0x7f;
const ELF_MAG1: u8 = b'E';
const ELF_MAG2: u8 = b'L';
const ELF_MAG3: u8 = b'F';
const EV_CURRENT: u8 = 1;

#[derive(Debug)]
pub struct Elf {
    pub ident: Ident,
}

#[derive(Debug, Copy, Clone)]
pub struct Ident {
    pub class: FileClass,
    pub encoding: DataEncoding,
    pub version: u8,
}

impl Ident {
    fn parse(file: &mut File) -> ElfResult<Ident> {
        let mut buf = [0; EI_NIDENT];
        file.read_exact(buf.as_mut_slice())
            .map_err(|e| error!("failed to read ELF identification from file", e))?;
        let mut cursor = Cursor::new(buf);

        Self::check_magic(&mut cursor)?;
        let class = FileClass::parse(&mut cursor)?;
        let encoding = DataEncoding::parse(&mut cursor)?;
        let version = Self::parse_version(&mut cursor)?;

        Ok(Ident {
            class,
            encoding,
            version,
        })
    }

    fn check_magic<R: ReadBytesExt>(reader: &mut R) -> ElfResult<()> {
        let mag0 = reader.read_u8().unwrap();
        let mag1 = reader.read_u8().unwrap();
        let mag2 = reader.read_u8().unwrap();
        let mag3 = reader.read_u8().unwrap();
        if mag0 == ELF_MAG0 && mag1 == ELF_MAG1 && mag2 == ELF_MAG2 && mag3 == ELF_MAG3 {
            Ok(())
        } else {
            Err(error!("magic number not found"))
        }
    }

    fn parse_version<R: ReadBytesExt>(reader: &mut R) -> ElfResult<u8> {
        let version = reader.read_u8().unwrap();
        match version {
            EV_CURRENT => Ok(version),
            v => Err(error!("invalid header version", v)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FileClass {
    Class32,
    Class64,
}

impl FileClass {
    fn parse<R: ReadBytesExt>(reader: &mut R) -> ElfResult<FileClass> {
        use FileClass::*;
        let value = reader.read_u8().unwrap();
        match value {
            1 => Ok(Class32),
            2 => Ok(Class64),
            c => Err(error!("invalid file class", c)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DataEncoding {
    Data2Lsb,
    Data2Msb,
}

impl DataEncoding {
    fn parse<R: ReadBytesExt>(reader: &mut R) -> ElfResult<DataEncoding> {
        use DataEncoding::*;
        let value = reader.read_u8().unwrap();
        match value {
            1 => Ok(Data2Lsb),
            2 => Ok(Data2Msb),
            e => Err(error!("invalid data encoding", e)),
        }
    }
}

pub struct ElfParser {
    ident: Option<Ident>,
}

impl ElfParser {
    pub fn new() -> ElfParser {
        ElfParser { ident: None }
    }

    pub fn parse<P: AsRef<Path>>(&mut self, file_path: P) -> ElfResult<Elf> {
        let mut file = File::open(file_path).map_err(|e| error!("failed to open file", e))?;
        self.ident = Some(
            Ident::parse(&mut file).map_err(|e| error!("failed to parse ELF identification", e))?,
        );
        Ok(Elf {
            ident: self.ident.as_ref().copied().unwrap(),
        })
    }
}

pub type ElfResult<T> = std::result::Result<T, ElfError>;

#[derive(Debug)]
pub struct ElfError(pub String);

impl std::error::Error for ElfError {}

impl fmt::Display for ElfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

macro_rules! error {
    ($msg:literal) => {
        ElfError(String::from($msg))
    };
    ($msg:literal, $data:expr) => {
        ElfError(format!("{}: {}", $msg, $data))
    };
}
use error;

#[cfg(test)]
mod tests {}
