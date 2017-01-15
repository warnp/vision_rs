use data_type::data_type::{Data, Image};
use std::rc::Rc;
use maud;

pub struct ImagePage;

impl ImagePage {
    pub fn get_page(&self, host: &str, image_data: Data, i: usize) -> maud::PreEscaped<String> {

        let src = self.find_next_occurence(&image_data, i);
        // println!("{:#?}", src);
        let url = "http://".to_string() + &host + "/images" + &src.src;

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

    fn find_next_occurence(&self, image_data: &Data, i: usize) -> &Image{
        let result = image_data.data.iter().find(|x| x.id == i as i32);
        &match result {
            Some(x) => image_data.data.iter().find(|x| x.id == i as i32),
            None => Some(self.find_next_occurence(image_data, i))
        }.unwrap()
        // Image{id:0,jour:0, src:"".to_string()}
    }
}
