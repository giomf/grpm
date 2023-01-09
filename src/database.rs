use jammdb::DB;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    path::Path,
};

const BUCKET_NAME: &str = "PACKAGES";

pub struct Database {
    database: DB,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub binary: String,
    pub path: String,
}

impl Database {
    pub fn new(path: impl AsRef<Path>) -> Result<Database, Box<dyn Error>> {
        let database = DB::open(path)?;
        let tx = database.tx(true)?;

        if tx.buckets().count() == 0 {
            tx.create_bucket(BUCKET_NAME)?;
        }

        tx.commit()?;
        Ok(Database { database })
    }

    pub fn put(&self, key: &str, package: &Package) -> Result<(), Box<dyn Error>> {
        let tx = self.database.tx(true)?;
        let bucket = tx.get_bucket(BUCKET_NAME)?;
        let value = bincode::serialize(&package)?;
        bucket.put(key.as_bytes(), value)?;
        tx.commit()?;
        Ok(())
    }
    
    pub fn remove(&self, key: &str) -> Result<(), Box<dyn Error>>{
        let tx = self.database.tx(true)?;
        let bucket = tx.get_bucket(BUCKET_NAME)?;
        bucket.delete(key)?;
        tx.commit()?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<Package>, Box<dyn Error>> {
        let tx = self.database.tx(false)?;
        let bucket = tx.get_bucket(BUCKET_NAME)?;
        if let Some(kv) = bucket.get_kv(key) {
            return Ok(Some(bincode::deserialize(kv.value())?));
        }
        Ok(None)
    }

    pub fn get_all(&self) -> Result<Vec<Package>, Box<dyn Error>> {
        let tx = self.database.tx(false)?;
        let mut packages: Vec<Package> = Vec::new();
        for data in tx.get_bucket(BUCKET_NAME)?.cursor() {
            if let jammdb::Data::KeyValue(kv) = &*data {
                let package: Package = bincode::deserialize(kv.value())?;
                packages.push(package)
            }
        }
        Ok(packages)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::{fs, path::Path};
    static PATH: &str = "test.db";

    fn setup() -> Database {
        let path = Path::new(PATH);
        if path.exists() {
            fs::remove_file(PATH).unwrap();
        }
        Database::new(path).unwrap()
    }

    #[test]
    fn new() {
        let db = setup();
        assert!(Path::new(PATH).exists());
        let tx = db.database.tx(false).unwrap();
        let _ = tx.get_bucket(BUCKET_NAME).unwrap();
    }

    #[test]
    fn put_get() {
        let db = setup();
        let key = "Test";
        let package = Package {
            name: "Test Package".to_string(),
            path: "/test/path".to_string(),
            version: "v1.2.3".to_string(),
            binary: "test".to_string()
        };
        db.put(key, &package).unwrap();
        let result = db.get(key).unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), package);
    }

    #[test]
    fn get_all() {
        let db = setup();
        let key1 = "Test1";
        let key2 = "Test2";

        let package = Package {
            name: "Test Package".to_string(),
            path: "/test/path".to_string(),
            version: "v1.2.3".to_string(),
            binary: "test".to_string()

        };

        db.put(key1, &package).unwrap();
        db.put(key2, &package).unwrap();
        
        let packages = db.get_all().unwrap();
        
        assert!(packages.len() == 2);
    }
    
    #[test]
    fn delete() {
        let db = setup();
        let key = "Test";
        let package = Package {
            name: "Test Package".to_string(),
            path: "/test/path".to_string(),
            version: "v1.2.3".to_string(),
            binary: "test".to_string()

        };
        db.put(key, &package).unwrap();
        db.remove(key).unwrap();
        let result = db.get(key).unwrap();
        assert!(result.is_none());
    }
}
