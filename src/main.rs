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

use iron::prelude::*;
use iron::status;

use router::Router;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use staticfile::Static;
use mount::Mount;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    data: Vec<Image>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Image{
    src : String,
    jour: i32,
}

fn prec_page(actual :i32, total :i32)->i32{
    if actual - 1 <0 {
        return total as i32;
    }else{
        return actual as i32 - 1;
    }

}

fn next_page(actual :i32, total :i32) -> i32{
    if actual + 1 > total {
        return 0i32;
    }else{
        return (actual + 1) as i32;

    }
}


fn main() {
    let mut router = Router::new();
    let mut mount = Mount::new();

    router.get("/:imageId", handler,"index");

    mount.mount("/images", Static::new(Path::new("content/images")));
    mount.mount("/style", Static::new(Path::new("content/style")));
    mount.mount("/", router);


    fn handler(req: &mut Request) -> IronResult<Response> {
        let mut file = File::open("./content/text.json").unwrap();
        let mut data = String::new();

        file.read_to_string(&mut data).unwrap();
        let deserialized: Data = serde_json::from_str(&data).unwrap();

        let ref query = req.extensions.get::<Router>().unwrap().find("imageId").unwrap_or("/");
        let mut host :String = "".to_string();
        for item in req.headers.iter() {
            if item.name() == "Host" {
                host = item.value_string();

            }
        }

        println!("{}", host);
        // let
        let result = query.parse::<usize>();
        match result {
            Ok(i) => {
                let url = "http://".to_string()+&host+"/images"+ &deserialized.data[i].src;
                let markup = html!{
                    link rel="stylesheet" type="text/css" href=("http://".to_string()+&host+"/style/style.css") /
                    div class="photo"{
                        img src=(url){}
                    }
                    div class="wrapper"{
                        a href=(prec_page(i as i32, (deserialized.data.len() -1) as i32)){
                            div class="arrowLeft"{}
                        }

                        a href=(next_page(i as i32, (deserialized.data.len()-1) as i32)){
                            div class="arrowRight"{}
                        }
                    }
                };
                Ok(Response::with((status::Ok, markup)))
            }
            Err(_) => {
                let markup = html!{
                    h1 "Bad request!"
                };
                Ok(Response::with((status::Ok, markup)))
            }
        }

    }

    Iron::new(mount).http("localhost:3000").unwrap();
}
