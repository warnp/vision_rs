#![feature(proc_macro)] 

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
use data_type::data_type::Data;



fn main() {
    let mut router = Router::new();
    let mut mount = Mount::new();

    router.get("/", index_handler, "index_handler");
    router.get("/:imageId", handler, "handler");

    mount.mount("/images", Static::new(Path::new("content/images")));
    mount.mount("/images/min", Static::new(Path::new("content/images/min")));
    mount.mount("/style", Static::new(Path::new("content/style")));
    mount.mount("/js", Static::new(Path::new("content/js")));
    mount.mount("/", router);


    fn index_handler(req: &mut Request) -> IronResult<Response> {
        let mut file = File::open("./content/text.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let deserialized: Data = serde_json::from_str(&data).unwrap();

        let mut host: String = "".to_string();
        for item in req.headers.iter() {
            if item.name() == "Host" {
                host = item.value_string();
            }
        }

        Ok(Response::with((status::Ok, ShowOff::get_page(&host, deserialized))))
    }
    fn handler(req: &mut Request) -> IronResult<Response> {
        let mut file = File::open("./content/text.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let deserialized: Data = serde_json::from_str(&data).unwrap();


        let image_page = ImagePage {};

        let ref query_image =
            req.extensions.get::<Router>().unwrap().find("imageId").unwrap_or("/");
        let mut host: String = "".to_string();
        for item in req.headers.iter() {
            if item.name() == "Host" {
                host = item.value_string();
            }
        }

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


    }

//0.0.0.0 For docker purpose
    Iron::new(mount).http("0.0.0.0:3000").unwrap();
    // Iron::new(mount).http("localhost:3000").unwrap();
}
