use data_type::data_type::Data;
use maud;

pub struct ImagePage;

impl ImagePage {
    pub fn get_page(&self, host: &str, image_data: Data, i: usize) -> maud::PreEscaped<String> {
        let url = "http://".to_string() + &host + "/images" + &image_data.data[i].src;

        html!{
            link rel="stylesheet" type="text/css"
                href=("http://".to_string()+host+"/style/style.css") /
            div class="photo"{
                img src=(url){}
            }
            div class="wrapper"{
                a href=(self.prec_page(i as i32, (image_data.data.len() -1) as i32)){
                    div class="arrowLeft"{}
                }

                a href=(self.next_page(i as i32, (image_data.data.len()-1) as i32)){
                    div class="arrowRight"{}
                }
            }
        }
    }

    fn prec_page(&self, actual: i32, total: i32) -> i32 {
        if actual - 1 < 0 {
            return total as i32;
        } else {
            return actual as i32 - 1;
        }

    }

    fn next_page(&self, actual: i32, total: i32) -> i32 {
        if actual + 1 > total {
            return 0i32;
        } else {
            return (actual + 1) as i32;

        }
    }
}
