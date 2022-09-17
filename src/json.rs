
extern crate serde_json;
use serde::{Serialize,Deserialize};
use exif::Tag;
use exif::Exif;
use crate::get_value_exif;


#[derive(Serialize,Deserialize,Debug)]
pub struct MetaData{
    pub capture_by:String,
    pub capture_time:String,
    pub software:String,
    pub resolution:String,
    pub bits_per_sample:String,
    pub model:String,
    pub gamma:String,
    pub device_setting:String,
    pub contrast:String,
    pub sharpness:String,
    pub saturation:String,
    pub compression:String,
    pub brigthnes_value:String,
    pub focal_length:String,
}


impl MetaData{

    pub fn generate_meta_data_json(image:&Exif)->MetaData{

        let y_res = get_value_exif(&image,Tag::YResolution).to_string();
        let x_res = get_value_exif(&image,Tag::XResolution).to_string();
        let resolution = format!("{} * {}",x_res,y_res);

        let data = MetaData{
            capture_by : get_value_exif(&image,Tag::Artist),
            capture_time:get_value_exif(&image,Tag::DateTimeOriginal),
            software:get_value_exif(&image,Tag::Software),
            resolution:resolution,
            bits_per_sample:get_value_exif(&image,Tag::BitsPerSample), 
            model:get_value_exif(&image,Tag::Model),
            gamma:get_value_exif(&image,Tag::Gamma),
            device_setting:get_value_exif(&image,Tag::DeviceSettingDescription),
            contrast:get_value_exif(&image,Tag::Contrast),
            sharpness:get_value_exif(&image,Tag::Sharpness),
            saturation:get_value_exif(&image,Tag::Saturation),
            compression:get_value_exif(&image,Tag::Compression),
            brigthnes_value:get_value_exif(&image,Tag::BrightnessValue),
            focal_length:get_value_exif(&image,Tag::FocalLength),
        };

        return data;

        

    }
}
