use crate::{
    constances::DATA_FILE_NAME,
    entry::{Entry, EntryContent, EntryType},
    error::{Error, Result},
    utils::clean_path,
};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Read, Write},
    path,
    time::SystemTime,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Locker {
    password: String,
    entries: Entry,
}

impl Locker {
    ///
    ///
    ///
    pub fn create_archive(src: &str, password: &str) -> Result<()> {
        let mut locker = Locker {
            password: password.to_string(),
            entries: Entry {
                ty: EntryType::Folder,
                path: "".to_string(),
                contents: EntryContent::Many(Box::new(vec![])),
                created_at: SystemTime::now(),
            },
        };

        dir_reader(&src, &src, &mut locker.entries)?;

        let file = fs::File::create(path::Path::new(&src).join(DATA_FILE_NAME))
            .map_err(|_| Error::WriteFile)?;

        serde_json::to_writer(io::BufWriter::new(file), &locker).map_err(|_| Error::WriteFile)
    }

    ///
    ///
    ///
    pub fn open_archive(src: &str, password: &str) -> Result<()> {
        let file = fs::File::open(src).map_err(|_| Error::ReadFile)?;

        let locker: Locker =
            serde_json::from_reader(io::BufReader::new(file)).map_err(|_| Error::ParseJson)?;

        if locker.password.ne(&password) {
            return Err(Error::InvalidPassword);
        }

        dir_creator(path::Path::new(src).parent().unwrap(), locker.entries)?;

        Ok(())
    }
}

///
///
///
fn dir_reader(src_dir: &str, current_dir: &str, data: &mut Entry) -> Result<()> {
    for entry in fs::read_dir(current_dir)
        .map_err(|_| Error::ReadDir)?
        .flatten()
    {
        let entry_path = entry.path().to_str().unwrap().to_string();
        let entry_relative_path = entry_path.replace(&src_dir, "");
        let entry_created_at = entry.metadata().unwrap().created().unwrap();

        let new_content = if entry.metadata().unwrap().is_dir() {
            let mut new_data = Entry {
                ty: EntryType::Folder,
                path: entry_relative_path,
                contents: EntryContent::Many(Box::new(vec![])),
                created_at: entry_created_at,
            };

            dir_reader(src_dir, &entry_path, &mut new_data)?;

            new_data
        } else {
            let contents = fs::read_to_string(&entry_path).unwrap();

            Entry {
                ty: EntryType::File,
                path: entry_relative_path,
                contents: EntryContent::Data(contents),
                created_at: entry_created_at,
            }
        };

        match &mut data.contents {
            EntryContent::Many(contents) => {
                contents.push(new_content);
            }
            EntryContent::Data(_) => {
                println!("NOTHING TO DO RIGHT NOW");
            }
        }
    }

    Ok(())
}

fn dir_creator(dist: &path::Path, entry: Entry) -> Result<()> {
    match entry.contents {
        EntryContent::Many(contents) => {
            for content in contents.into_iter() {
                dir_creator(dist, content)?;
            }
        }
        EntryContent::Data(data) => {
            let entry_path = dist.join(clean_path(&entry.path));

            if let Some(parent) = entry_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }

            println!("path: {entry_path:?}");

            let mut file = fs::File::create(entry_path).unwrap();

            file.write_all(data.as_bytes()).unwrap();
            file.flush().unwrap();
        }
    }

    Ok(())
}
