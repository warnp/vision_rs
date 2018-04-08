#![feature(plugin, decl_marco)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use rocket::response::NamedFile;
use rocket::http::RawStr;
use std::path::{Path, PathBuf};
use rocket_contrib::Template;
use rocket::config::{Config, Environment};
use std::fs::File;
use std::io::prelude::*;

//const HOST : &'static str = "http://localhost:8000";

#[derive(Serialize, Deserialize, Debug)]
struct TemplateWrapper {
    trip: Trip,
    host: String,
}

#[derive(Serialize, Deserialize,Debug)]
struct Trip {
    data: Vec<DayTemplate>,
}

#[derive(Serialize, Deserialize,Debug)]
struct DayTemplate {
    id: u32,
    title: String,
    presentation: String,
    content: Vec<TemplateContext>,
}

#[derive(Serialize, Deserialize,Debug)]
struct TemplateContext {
    paragraph: String,
    img1: String,
    img2: String,
    img3: String,
}

#[get("/peru")]
fn index() -> Template {
    let mut file = File::open("./content/data/text_content.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let trip: Trip = serde_json::from_str(&data).unwrap();

    Template::render("index", &trip)
}

//#[get("/data/<file..>")]
//fn data(file: PathBuf) -> Option<NamedFile> {
//    NamedFile::open(Path::new("content/data").join(file)).ok()
//}

#[get("/peru/style/<file..>")]
fn style(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("content/style/").join(file)).ok()
}

#[get("/peru/js/<file..>")]
fn js(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("content/js/").join(file)).ok()
}

#[get("/peru/images/<file..>")]
fn images(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("content/images/").join(file)).ok()
}

fn main() {
    let mut config = Config::new(Environment::Staging).unwrap();

    let mut extras = std::collections::HashMap::new();
    extras.insert("templates".to_string(), "content".into());
    config.set_extras(extras);


    rocket::ignite().mount("/", routes![index, style, js, images])
        .attach(Template::fairing())
        .launch();
}

//fn main() {

//    router.get("/blog", index_handler, "index_handler");
//    router.get("/images/:imageId", handler, "handler");
//
//    mount.mount("/images/", Static::new(Path::new("content/images")));
//    mount.mount("/images/min", Static::new(Path::new("content/images/min")));
//    mount.mount("/style/", Static::new(Path::new("content/style")));
//    mount.mount("/js/", Static::new(Path::new("content/js")));
//    mount.mount("/", router);


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


//    }

//0.0.0.0 For docker purpose
//    Iron::new(mount).http("0.0.0.0:3000").unwrap();
// Iron::new(mount).http("localhost:3000").unwrap();
//}
