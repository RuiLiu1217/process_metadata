use std::{collections::HashMap, path::PathBuf, hash::{self, Hash}, fs::File, io::{BufReader, BufRead}, sync::{RwLock, Arc}};

use inotify::{Inotify, WatchMask};
use tokio::select;

#[derive(Debug, Clone)]
pub struct PasswdEntry {
    pub user_name: String,
    pub passwd: String,
    pub uid: u64,
    pub gid: u64,
    pub user_info: String,
    pub home_directory: String,
    pub command: String
}

#[derive(Debug)]
pub enum PasswdError {
    Generic(String)
}

impl TryFrom<&String> for PasswdEntry {
    type Error = PasswdError;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let mut token = value.split(':');
        let user_name = token.next().unwrap_or_default();
        if user_name.is_empty() {
            return Err(PasswdError::Generic(String::from("uid cannot be empty")));
        }
        let user_name = user_name.to_string();
        let passwd = token.next().unwrap_or_default().to_string();
        let uid = token.next().unwrap_or("unknown").parse::<u64>().map_err(|e| { PasswdError::Generic(format!("{e:?}"))})?;
        let gid = token.next().unwrap_or("unknown").parse::<u64>().map_err(|e| { PasswdError::Generic(format!("{e:?}"))})?;
        let user_info = token.next().unwrap_or_default().to_string();
        let home_directory = token.next().unwrap_or_default().to_string();
        let command = token.next().unwrap_or_default().to_string();
        Ok(Self {
            user_name,
            passwd,
            uid,
            gid,
            user_info,
            home_directory,
            command,
        })
    }
}

pub struct PasswdCache {
    pub cache: Arc<RwLock<HashMap<u64, PasswdEntry>>>,
    pub file_path: String,
}

impl PasswdCache {
    pub fn get(&self, uid: u64) -> Option<PasswdEntry> {
        if let Some(entry) = self.cache.read().unwrap().get(&uid) {
            Some(entry.clone())
        } else {
            None
        }
    }

    pub fn refresh(&mut self) -> Result<(), PasswdError> {
        let new_cache = Self::create_new_cache_map(&self.file_path)?;
        let mut c = self.cache.write().unwrap();
        c.clear();
        for (k, v) in new_cache {
            c.insert(k, v);
        }
        Ok(())
    }

    fn create_new_cache_map(file_path: &str) -> Result<HashMap<u64, PasswdEntry>, PasswdError> {
        let mut cache = HashMap::new();
        let file = File::open(file_path).map_err(|e| { PasswdError::Generic(format!("{e:?}"))})?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.map_err(|e| { PasswdError::Generic(format!("{e:?}"))})?;
            let passwd_entry = PasswdEntry::try_from(&line).map_err(|e| { PasswdError::Generic(format!("{e:?}"))})?;
            cache.insert(passwd_entry.uid.clone(), passwd_entry);
        }

        Ok(cache)
    }

    pub fn new(file_path: &str) -> Result<PasswdCache, PasswdError> {        
        // 1. Open file and read it to the cache
        let cache = Arc::new(RwLock::new(Self::create_new_cache_map(file_path).map_err(|e| { PasswdError::Generic(format!("{e:?}"))})?));

        let pc = PasswdCache {
            cache,
            file_path: file_path.to_string(),
        };        

        pc.watch();

        Ok(pc)
    }

    fn watch(&self) {
        let cache_clone = self.cache.clone();
        let file_path = self.file_path.clone();

        std::thread::spawn(move || {
            let mut inotify = Inotify::init().unwrap_or_else(|e| {
                panic!("An error occurred: {:?}", e);
            });
            inotify.watches().add(
                PathBuf::from(&file_path),
                WatchMask::MODIFY
            ).unwrap_or_else(|e| {
                panic!("An error occurred: {:?}", e);
            });
            let mut buffer = [0u8; 4096];
            loop {
                if let Ok(_) = inotify.read_events(&mut buffer) {
                    println!("XXX Detect the file is changed");
                    cache_clone.write().unwrap().clear();
                    let file = File::open(&file_path).unwrap_or_else(|e| {
                        panic!("An error occurred: {:?}", e);
                    });
                    let reader = BufReader::new(file);
                    for line in reader.lines() {
                        let line = line.unwrap_or_else(|e| {
                            panic!("An error occurred: {:?}", e);
                        });
                        let passwd_entry = PasswdEntry::try_from(&line).unwrap_or_else(|e| {
                            panic!("An error occurred: {:?}", e);
                        });
                        cache_clone.write().unwrap().insert(passwd_entry.uid.clone(), passwd_entry);
                    }
                }
            }
        });
    }

}
