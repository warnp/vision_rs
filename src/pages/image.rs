use data_type::data_type::Data;
use maud::PreEscaped;

pub struct ImagePage;

impl ImagePage {
    pub fn get_page(&self, host: &str, image_data: Data, i: usize) -> PreEscaped<String> {
        // Bug possible avec ça je pense
        let src = match image_data.data.iter().find(|x| x.id == i as i32) {
            Some(x) => x,
            None => image_data.data.iter().find(|x| x.id == i as i32 + 1).unwrap(),
        };

        // println!("{:#?}", src);
        let url = "http://".to_string() + &host + "/images" + &src.src;
        let coordinates = (&src.coord[0]).to_string()+"/"+&(&src.coord[1]).to_string();
        html!{
            link rel="stylesheet" type="text/css"
                href=("http://".to_string()+host+"/style/style.css") /

            link href="https://fonts.googleapis.com/css?family=Open+Sans" rel="stylesheet" /

            div class="photo"{

                div class="controls"{
                    a class="control" href=(self.prec_page(i as i32, (image_data.data.len() -1) as i32)){
                        div class="arrowLeft"{}
                    }

                    // a class="control" href=("http://".to_string()+host+"/#"+&src.id.to_string()){
                    //     div {"Toto"}
                    // }
                    img src=(url){}

                    a class="control" href=(self.next_page(i as i32, (image_data.data.len()-1) as i32)){
                        div class="arrowRight"{}
                    }
                }
            }
            div class="wrapper"{
                div class="text" {
                    (&src.text)
                    br /
                    a href=("https://www.openstreetmap.org/#map=14/".to_string() + &coordinates){
                        ("https://www.openstreetmap.org/#map=14/".to_string() + &coordinates)
                    }
                }


            }
            (PreEscaped("<script type=\"text/javascript\" src=\"https://ajax.googleapis.com/ajax/libs/jquery/3.1.1/jquery.min.js\"></script>"))
            script {
                (PreEscaped("$(\"html\").keypress(function(event){
                            // alert(event.key);
                            if(event.key === \"keyLeft\"){
                                $.load(".to_string()+&self.prec_page(i as i32, (image_data.data.len() -1) as i32).to_string()+");
                            }
                            if(event.key === \"keyRight\"){
                                $.load("+&self.next_page(i as i32, (image_data.data.len() -1) as i32).to_string()+");
                            }
                });"))
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
