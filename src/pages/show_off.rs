
use data_type::data_type::Data;
use maud::PreEscaped;

pub struct ShowOff;

impl ShowOff {
    pub fn get_page(host: &str, data: Data) -> PreEscaped<String> {

        html!{
            link rel="stylesheet" type="text/css"
                href=("http://".to_string()+host+"/style/style.css") /

                @for image in &data.data {
                    a href=("http://".to_string()+host+"/"+&image.id.to_string()){
                        img.photoGallery src=("http://".to_string()+host+"/images/min"+&image.src.split_at(9).1){}
                    }
                }
        }
    }
}
