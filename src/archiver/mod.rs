use std::{
    fs::{self, File},
    path::Path,
    error::Error,
    io::{Write, Read}
};

use bincode;

use lz4::{
    EncoderBuilder,
    Decoder
};

use serde::{
    Serialize,
    Deserialize
};


#[allow(unused)]
pub fn serialize<T: Serialize>(data: T, path: &str, level: u32) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(&path)?;

    let mut encoder = EncoderBuilder::new()
        .level(level)
        .build(file)?;

    let encoded = bincode::serialize(&data)?;
    encoder.write_all(&encoded)?;

    let (_, result) = encoder.finish();
    result?;

    Ok(())
}

#[allow(unused)]
pub fn deserialize<T: for<'de> Deserialize<'de>>(path: &str) -> Result<T, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut decoder = Decoder::new(file)?;

    let mut decoded = Vec::new();
    decoder.read_to_end(&mut decoded)?;
    let data = bincode::deserialize(&decoded)?;

    Ok(data)
}
