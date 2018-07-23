#[macro_use]
extern crate rouille;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate handlebars;

mod controller;
mod data_type;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;
use handlebars::Handlebars;
use data_type::{TemplateContext, DayTemplate, TemplateWrapper, Trip};


const HOST : &'static str = "localhost:8000";


//#[get("/peru")]
//fn index() -> Template {
//    let mut file = File::open("./content/data/text_content.json").unwrap();
//    let mut data = String::new();
//    file.read_to_string(&mut data).unwrap();
//    let trip: Trip = serde_json::from_str(&data).unwrap();
//
//    Template::render("index", &trip)
//}
//
//
//#[get("/peru/style/<file..>")]
//fn style(file: PathBuf) -> Option<NamedFile> {
//    NamedFile::open(Path::new("content/style/").join(file)).ok()
//}
//
//#[get("/peru/js/<file..>")]
//fn js(file: PathBuf) -> Option<NamedFile> {
//    NamedFile::open(Path::new("content/js/").join(file)).ok()
//}
//
//#[get("/peru/images/<file..>")]
//fn images(file: PathBuf) -> Option<NamedFile> {
//    NamedFile::open(Path::new("content/images/").join(file)).ok()
//}
//


fn main() {
    println!("Server is running on {}", HOST);
    rouille::start_server(HOST, move |req| {
        //Serve static file
        {
            let response = rouille::match_assets(req, ".");
            if response.is_success() {
                return response;
            }
        }
        router!(req,
            (GET)(/peru/{id : i32}) => {
                controller::serve_article(&req, &id)
                },
            (GET)(/peru) => {
                controller::serve_index(&req)
                },
                _ => rouille::Response::empty_404()
            )
    },
    );
}

//    router.get("/blog", index_handler, "index_handler");
//    router.get("/images/:imageId", handler, "handler");
//
//    mount.mount("/images/", Static::new(Path::new("content/images")));
//    mount.mount("/images/min", Static::new(Path::new("content/images/min")));
//    mount.mount("/style/", Static::new(Path::new("content/style")));
//    mount.mount("/js/", Static::new(Path::new("content/js")));
//    mount.mount("/", router);
//
//
//    fn index_handler(req: &mut Request) -> IronResult<Response> {
//        let mut file = File::open("./content/photos.json").unwrap();
//        let mut data = String::new();
//        file.read_to_string(&mut data).unwrap();
//        let deserialized: Data = serde_json::from_str(&data).unwrap();
//
//        let mut host: String = "".to_string();
//        for item in req.headers.iter() {
//            if item.name() == "Host" {
//                host = item.value_string();
//            }
//        }
//
//        Ok(Response::with((status::Ok, ShowOff::get_page(&host, deserialized))))
//    }
//    fn handler(req: &mut Request) -> IronResult<Response> {
//        let mut file = File::open("./content/photos.json").unwrap();
//        let mut data = String::new();
//        file.read_to_string(&mut data).unwrap();
//        let deserialized: Data = serde_json::from_str(&data).unwrap();
//
//
//        let image_page = ImagePage {};
//
//        let ref query_image =
//            req.extensions.get::<Router>().unwrap().find("imageId").unwrap_or("/");
//        let mut host: String = "".to_string();
//        for item in req.headers.iter() {
//            if item.name() == "Host" {
//                host = item.value_string();
//            }
//        }
//
//        let result = query_image.parse::<usize>();
//        match result {
//            Ok(i) => Ok(Response::with((status::Ok, image_page.get_page(&host, deserialized, i)))),
//            Err(_) => {
//      //          let markup = html!{
//      //              h1 "Bad request!"
//      //          };
//                Ok(Response::with((status::Ok, "".to_string())))
//            }
//        }
//
//
//    }
//
//0.0.0.0 For docker purpose
//    Iron::new(mount).http("0.0.0.0:3000").unwrap();
// Iron::new(mount).http("localhost:3000").unwrap();
//}
