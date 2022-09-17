#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
use std::path::Path;
use std::fs::{write,remove_file};
use exif::Error;
mod json;
use json::MetaData;
mod utils;
pub use utils::get_value_exif;
use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::response;
use rocket::http::ContentType;
use rocket::request::Request;
mod migratrions;
pub use migratrions::TableConnection;

pub struct ImageFile<'a>{
    pub image_path:&'a Path
}

//create a new file
impl<'a> ImageFile<'a>{
    pub fn new(image_data:Vec<u8>)->Result<ImageFile<'a>,std::io::Error>{
        
       let path = Path::new("image.jpg");
       write(&path, image_data)?;
       Ok( 
        ImageFile{
            image_path: &path,
        })
      }
}


// impl drop tarit for ImageFile to delete file when request handler return 
impl<'a> Drop for ImageFile<'a>{

    fn drop(&mut self) {
        remove_file(self.image_path)
        .expect("File delete failed");
    }
}


#[derive(Debug)]
pub struct HttpError<T> {
    json: Json<T>,
    status: Status,
}

//impl Responder trait for Error struct to return it from handler
impl<'r, T: serde::Serialize> Responder<'r> for HttpError<T> {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}



//we will be using stream of bytes sent by client in form on binary data
#[post("/",data="<data>")]
pub fn hello(data:Vec<u8>)-> Result<Json<MetaData>,HttpError<String>> {
    
    
    let image = ImageFile::new(data).unwrap();
    let file = std::fs::File::open(image.image_path).expect(" ");
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = match exifreader.read_from_container(&mut bufreader){

        Ok(data)=>{
            data
        },
        // if error occur match it with Exif error enums
        Err(err)=>  match err{
             Error::NotFound(_reason)=>{
                return Err(HttpError{
                json : Json("error : image does not have meta data".to_string()),
                status : Status::BadRequest
             })
            },
             Error::TooBig(_reason)=>{
                 return  Err(HttpError{
                json : Json("error : the supplied image is too big".to_string()),
                status : Status::BadRequest
             })
               
               },
             Error::NotSupported(_reason)=>{
                return  Err(HttpError{
                json : Json("error : filed is not supported and can not be encoded".to_string()),
                status : Status::BadRequest
             })
              
             },
             _ =>{
                print!("{err}");
                 return Err(HttpError{
                   json : Json("error : internal server error".to_string()),
                   status : Status::BadRequest
             })
               
            } 

        }
    };
       
    //sql does not implment sync trait, we can not pass this as app data to handler as this is not thread safe
    //get connection to table
    //struct Metdadata is Serelized , can be converted into json
   
    let data = MetaData::generate_meta_data_json(&exif);
    let database = TableConnection::new();
    database.create_table();

    match database.insert_content(&data){
        Ok(data)=>println!("{data}"),
        Err(data)=>println!("{data}")

    };
    
    
    return Ok(Json(data));


}



fn main() {
    
    

    rocket::ignite()
    .mount("/", routes![hello]).launch();
}