use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{anyhow, Result};

pub fn read_entire_file<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    let s = fs::read_to_string(path)?;
    Ok(s)
}

pub fn read_file_lines<P>(path: P) -> Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();
    let mut vec = vec![];

    for line in lines {
        match line {
            Ok(line) => vec.push(line),
            Err(err) => return Err(anyhow!(err)),
        }
    }

    Ok(vec)
}
