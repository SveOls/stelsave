
use std::fs::File;
use std::fs;
use std::io::{prelude::*, BufReader};
use std::error::Error;



pub fn analyse(inp: &str) -> Result<impl Iterator<Item = String>, Box<dyn Error>> {

    let mut zipper: zip::ZipArchive<std::fs::File>;
    let mut writer: Vec<u8> = vec![];
    let file:       zip::read::ZipFile;
    let f:          std::fs::File;

    f = File::open(format!("data//{}.sav", inp))?;

    zipper  = zip::ZipArchive::new(f)?;
    file    = zipper.by_name("gamestate")?;

    let mut file = file;
    std::io::copy(&mut file, &mut writer)?;
    std::mem::drop(&mut file);

    fs::write(format!("saves//{}.txt", inp), &writer)?;

    let ret = BufReader::new(File::open(format!("saves//{}.txt", inp))?).lines().filter_map(|x| x.ok());
    Ok(ret)
}


