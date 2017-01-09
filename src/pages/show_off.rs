pub struct ShowOff;

impl ShowOff {
    pub fn get_page(host: &str, data: Data) -> maud::PreEscaped<String> {
        html!{
            link rel="stylesheet" type="text/css"
                href=("http://".to_string()+host+"/style/style.css") /

                @for image in &data.data {
                    a href=("http://".to_string()+host+image.src){
                        img src=(url){}
                    }
                }
        }
    }
}
