use sqlite::Connection;
use crate::json::MetaData;

pub struct TableConnection{
    connection:Connection
}

impl TableConnection{

    pub fn new()->TableConnection{

        TableConnection{
            connection:Connection::open("src/migratrions/image.db").expect("fail to unwrap")
        }
    }

    pub fn create_table(&self){

        //executing the same this may produce error
        match self.connection.execute(
            "
                CREATE TABLE Image(
                imagecapture_by TEXT,
                capture_time TEXT,
                software TEXT,
                resolution TEXT,
                bits_per_sample TEXT, 
                model TEXT,
                gamma TEXT,
                device_setting TEXT,
                contrast TEXT,
                sharpness TEXT,
                saturation TEXT,
                compression TEXT,
                brigthnes_value TEXT,
                focal_length TEXT).
            "
         ){
            Ok(_ok)=>{},
            Err(_err)=>{}
         };

     }
    
    pub fn insert_content(&self,metadata:&MetaData)->Result<&str,String>{


        let insert_query = format!("INSERT INTO Image VALUES ('{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}');",metadata.capture_by,metadata.capture_time
        ,metadata.software,metadata.resolution,metadata.bits_per_sample,metadata.model,metadata.gamma,metadata.device_setting,metadata.contrast,metadata.sharpness
        ,metadata.saturation,metadata.compression,metadata.brigthnes_value,metadata.focal_length);

        match self.connection.execute(insert_query.as_str()){
            Ok(_ok)=>return Ok("succes in posting data"),
            Err(_e)=>return Err(format!("{_e}"))
        };

    }
}
