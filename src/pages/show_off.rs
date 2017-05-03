
use data_type::data_type::Data;
use maud::PreEscaped;

pub struct ShowOff;

impl ShowOff {
    pub fn get_page(host: &str, data: Data) -> PreEscaped<String> {

        html!{
            link rel="stylesheet" type="text/css"
                href=("http://".to_string()+host+"/style/style.css") /

                div.gallery {
                    @for image in &data.data {
                        a.super-item href=("http://".to_string()+host+"/"+&image.id.to_string()) id=(&image.id.to_string()){
                            img.item.lazy data-original=("http://".to_string()+host+"/images/min"+&image.src.split_at(9).1){}

                        }
                    }
                }
                div{
                    "© 2017 Romain ASSIE"
                }
                (PreEscaped("<script src=\"https://ajax.googleapis.com/ajax/libs/jquery/3.1.1/jquery.min.js\"></script>"))
                (PreEscaped("<script src=\"http://".to_string() + host+"/js/jquery.lazyload.min.js\"></script>"))
                (PreEscaped("<script src=\"http://".to_string() + host+"/js/customloading.js\"></script>"))

        }
    }
}
