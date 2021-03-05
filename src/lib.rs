#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug)]
pub struct MFile {
    content: Vec<u8>,
}
#[derive(Debug)]
pub struct MDir {
    sub_dir: HashMap<String, Item>,
}
#[derive(Debug)]
pub enum Item {
    File(MFile),
    Dir(MDir),
}

impl MDir {
    pub fn mkdir(&mut self, paths: &'static str) -> Result<(), String> {
    
        let names = paths.split("/");
        let mut d = self;
        
        
        for name in names {
            match d._mkdir(name) {
                Result::Ok(new_dir) => {
                    match new_dir {
                        Item::Dir(dir)=>{
                            d = dir;        
                        },
                        Item::File(_)=>{
                            return Err(format!("err: dir {} is file", name)); 
                        }
                    }
                    
                },
                Err(err) =>{
                    return Err(err);
                }
            }
        }

        Ok(())
    }
    fn _mkdir(&mut self, name: &'static str) -> Result<&mut Item, String> {
        
        if self.exist(name) {
            if let Some(d) = self.sub_dir.get_mut(name) {
                return Ok(d)
            }else{
                return Err(format!("err: dir {} is not exist", name))
            }
        };
        println!("new dir {}", name);
        let dir = Item::Dir(MDir {
            sub_dir: HashMap::new(),
        });
        self.sub_dir.insert(String::from(name), dir);
        
        if let Some(d) = self.sub_dir.get_mut(name) {
            Ok(d)
        }else{
            Err(format!("err: dir {} is not exist", name))
        }
    }
    
    
    pub fn touch(&mut self, paths: &'static str)-> Result<(), String> {
    
        let names = paths.split("/").collect::<Vec<&str>>();
        let mut d = self;
        // let count = names.count();
        let mut i = 0;
        
        for name in names.iter() {
            
            if i == names.len() - 1 {
                match d._touch(name){
                    Result::Ok(new_dir) => {
                        match new_dir {
                            Item::Dir(_)=>{
                                return Err(format!("err: dir {} is dir", name));      
                            },
                            Item::File(_)=>{
                                
                            }
                        }
                    
                    },
                    Err(err) =>{
                        return Err(err); 
                    }
                }
            }else{
                match d._mkdir(name) {
                    Result::Ok(new_dir) => {
                        match new_dir {
                            Item::Dir(dir)=>{
                                d = dir;        
                            },
                            Item::File(_)=>{
                                return Err(format!("err: dir {} is file", name)); 
                            }
                        }
                    },
                    Err(err) =>{
                        return Err(err); 
                    }
                }
            }
            i += 1;
        }
        
        Ok(())
    }
    
    fn _touch(&mut self, filename: &'static str) -> Result<&mut Item, String>{
        
        if self.exist(filename) {
            return Err(format!("err: dir {} is exist", filename));
        };

        println!("new file {}", filename);
        let file = Item::File(MFile{
            content: vec![],
        });
        self.sub_dir.insert(String::from(filename), file);
        
        if let Some(f) = self.sub_dir.get_mut(filename) {
            Ok(f)
        }else{
            Err(format!("err: dir {} is exist", filename))
        }
        
    }
    
    pub fn exist(&self, dir_name: &'static str) -> bool {
        match self.sub_dir.get(dir_name) {
            Some(_) => true,
            None => false,
        }
    }
    
    pub fn count(&self) -> usize {
        return self.sub_dir.len();
    }
    
    pub fn get(&mut self, dir_name: &'static str)-> Option<&mut Item>{
        self.sub_dir.get_mut(dir_name)
    }
}

lazy_static! {
    static ref DISK: Mutex<MDir> = Mutex::new(MDir {
        sub_dir: HashMap::new()
    });
}

pub fn mkdir(dir_name: &'static str) -> Result<(), String> {
    let mut d = DISK.lock().unwrap();
    d.mkdir(dir_name)
}
// tree Dir{ "/":   Dir{ "User": Dir{ "config.js": File{ } }  } }
#[test]
fn test() {
    match mkdir("user/name") {
        Ok(_) => true,
        Err(err) => {
            println!("{}", err);
            false
        }
    };
    match mkdir("user") {
        Ok(_) => true,
        Err(err) => {
            println!("{}", err);
            false
        }
    };
    let mut d = DISK.lock().unwrap();
    assert_eq!(d.count(), 1);
    assert!(d.exist("user"));
    
    println!("{:#?}", d);
    
    let user = d.get("user").unwrap();
    match user {
        Item::Dir(dir)=>{
            dir.mkdir("name");
            dir.touch("config.js");
            
            assert!(dir.exist("name"));
            assert!(dir.exist("config.js"));
            assert_eq!(dir.count(), 2);
        },
        Item::File(_)=>{}
        
    };
    println!("{:#?}", user);
    println!("{:#?}", d);
    
    
}
