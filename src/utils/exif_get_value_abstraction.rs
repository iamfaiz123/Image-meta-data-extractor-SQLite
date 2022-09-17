
//get values from image using exif Tag::enum, convert them to string as &str does not live long enough
pub mod value {

       use exif::Tag;
       use exif::Exif;

       pub fn get_value_exif(exif:&Exif,tag:Tag)->String{

       let data = exif.get_field(tag,exif::In::PRIMARY);

       let _ =  match data{
         Some(data)=>{ 
             return data.display_value().to_string()
        },
         None=>{
             return "value not found".to_string()
        }
    };


  }
}