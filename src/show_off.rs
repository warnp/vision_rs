#![feature(proc_macro, plugin)]
#![plugin(maud_macros)]

extern crate maud;

use data::{Data,Image};

struct ShowOff;

impl ShowOff {
    fn get_page(images: &Vec<Data>) -> String {

        html!{
            link rel="stylesheet" type="text/css" href=("http://".to_string()+&host+"/style/style.css") /

            @for image in images {
                div class="photo"{
                    img src=(image.src){}
                }
            }

        }
    }
}
