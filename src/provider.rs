use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{self, Write, Read};
use std::any::type_name;
use std::marker::PhantomData;
use thiserror::Error;
use log::{info, error};

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub struct Provider<T> {
  pub items: Vec<T>,
  _marker: PhantomData<T>,
}

impl<T> Provider<T> 
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    pub fn new() -> Self {
        Provider { 
            items: Vec::new(), 
            _marker: PhantomData,
        }
    }

    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn save(&self) -> Result<(), serde_json::Error> {
        let type_name = type_name::<T>();
        // Replace "Provider" with an empty string
        type_name.replace("Provider", "");
        // Save the file
        let file_name = format!("data/{}.json", type_name);
        let serialized = serde_json::to_string(&self.items)?;
        let mut file = File::create(&file_name).expect("Unable to create file");
        file.write_all(serialized.as_bytes()).expect("Unable to write data");
        Ok(())
    }

    pub fn load(&self) -> Result<Self, ProviderError> {
        let type_name = type_name::<T>().replace("Provider", "");
        let file_name = format!("data/{}.json", type_name);

        fs::create_dir_all("data").expect("Unable to create data directory");
        match File::open(&file_name) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents);
                let items = serde_json::from_str(&contents)?;
                Ok(Provider { items, _marker: PhantomData })
            },
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
                info!("Initialized new {} provider [{}]", type_name, file_name);
                let provider = Provider { 
                    items: Vec::new(), 
                    _marker: PhantomData 
                };
                provider.save()?;
                Ok(provider)
            },
            Err(e) => Err(e.into()),
        }
    }
}