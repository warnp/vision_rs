#![feature(proc_macro, plugin)]
#![plugin(maud_macros)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate iron;
extern crate maud;
extern crate staticfile;
extern crate mount;
extern crate router;

mod pages;
mod data_type;

use iron::prelude::*;
use iron::status;

use router::Router;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use staticfile::Static;
use mount::Mount;
use pages::image::ImagePage;
use pages::show_off::ShowOff;
use data_type::data_type::{Data, Image};


fn main() {
    let mut router = Router::new();
    let mut mount = Mount::new();

    router.get("/", handler, "index");
    router.get("/:imageId", handler, "image");

    mount.mount("/images", Static::new(Path::new("content/images")));
    mount.mount("/style", Static::new(Path::new("content/style")));
    mount.mount("/", router);


    fn handler(req: &mut Request) -> IronResult<Response> {
        let mut file = File::open("./content/text.json").unwrap();
        let mut data = String::new();
        let image_page = ImagePage {};

        file.read_to_string(&mut data).unwrap();
        let deserialized: Data = serde_json::from_str(&data).unwrap();

        let ref query_image =
            req.extensions.get::<Router>().unwrap().find("imageId").unwrap_or("/");
        let ref query_index = req.extensions.get::<Router>().unwrap().find("/").unwrap_or("/");
        let mut host: String = "".to_string();
        for item in req.headers.iter() {
            if item.name() == "Host" {
                host = item.value_string();
            }
        }

        println!("{}", host);
        let result = query_image.parse::<usize>();
        match result {
            Ok(i) => Ok(Response::with((status::Ok, image_page.get_page(&host, deserialized, i)))),
            Err(_) => {
                let markup = html!{
                    h1 "Bad request!"
                };
                Ok(Response::with((status::Ok, markup)))
            }
        }

        let result_index = query_index.parse::<String>();
        match result_index {
            Ok() => Ok(Response::with((status::Ok, ShowOff::get_page(&host, deserialized)),
            Err(_) => {
                let markup = html!{
                    h1 "Bad request!"
                };
                Ok(Response::whith((status::Ok, markup)))
            }
        }

    

    Iron::new(mount).http("localhost:3000").unwrap();
}
