//! Help to extract any data from files


pub struct ExHardDrive{
    pub hd_size: u32,
}

pub trait IExtractor {
    fn info(&self);
    fn dump();
    fn check_data();
    fn search_delete_data();
}

impl IExtractor for ExHardDrive {
    fn info(&self){
        println!("size = {}",self.hd_size);
    }
    fn dump(){}
    fn check_data(){}
    fn search_delete_data(){}
}