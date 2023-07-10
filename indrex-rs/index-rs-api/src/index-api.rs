#[macro_use] extern crate rocket;


use colored::Colorize;
use reqwest::header::{self};
use rocket::form::validate::Contains;
use rocket::fs::FileServer;
use std::io::ErrorKind;
use std::{self, fmt, fs::{OpenOptions, self}, io::Write};

use futures::stream::StreamExt;
use reqwest::{StatusCode, Client, ClientBuilder};
use serde::{Deserialize, Serialize};


use scraper::{Html, Selector};

#[derive(Debug,  Serialize,  Deserialize)]
#[serde(rename_all = "camelCase")]
struct Author{
    id:u64,
    first_name:String,
    last_name:String,
    full_name:String,
    public_id:String
}
#[derive(Debug,  Serialize,  Deserialize)]
#[serde(rename_all = "camelCase")]
struct ArticleSummary {
    title: String,
    text: String,
    date: String,
    authors: Vec<Author>,
    comment_thread_id: String,
    url: String
}


#[derive(Debug,  Serialize,  Deserialize)]
#[serde(rename_all = "camelCase")]
struct ArticleInfo {
    title: String,
    summary: String,
    image: String,
    date: String,
    sekcija:String,
    url: String
}


#[derive(Debug,  Serialize,  Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiProfileGet {
    is_authenticated: bool,
    user_id: u32,
    external_user_id: String,
    profile_image_path: Option<String>,
    initials:String,
    is_commenting_banned:bool,
    full_name:String,
    can_moderate_comments:bool,
    number_of_comments:u32
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Entry {
    total_count: u16,
    total_count_with_replies: u16,
    comments: Vec<Komentar>,
}


#[derive(Debug,  Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct KomentarOdgovor {
    comment_id:u32,
    total_count:u16,
    comments:Vec<Komentar>,
}

#[derive(Debug)]
enum CommentSortEnum  {
    CreatedDateDesc=0,
    CreatedDateAsc=1,
    RelevanceScore=2,
    NumberOfRepliesDesc=3
}

impl fmt::Display for CommentSortEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug,  Serialize,  Deserialize)]
#[serde(rename_all = "camelCase")]
struct Komentar {
    comment_id: u32,
    comment_thread_id:u32,
    thread_type:u8,
    state:u8,
    state_string:String,
    content:String,
    created_date_utc:String,
    number_of_likes:u16,
    relationship_entity_id:u32,
    number_of_dislikes:u16,
    reply_count:u16,
    poster_first_name:String,
    poster_last_name:String,
    parent_comment_poster_first_name:Option<String>,
    parent_comment_poster_last_name:Option<String>,
    parent_comment_poster_user_id:Option<u32>,
    parent_comment_poster_public_id:Option<String>,
    parent_comment_id:Option<u32>,
    poster_disable_comment_actions_until_utc:Option<String>,
    poster_comment_actions_disabled:bool,
    reported:bool,
    poster_full_name:String,
    poster_initials:String,
    poster_id:u32,
    poster_profile_image_path:Option<String>,
    poster_public_id:String,
    reaction_type:Option<u8>,
    reaction_typetring:Option<String>,
    parent_comment_poster_full_name:Option<String>,
    replies:Option<KomentarOdgovor>,
}


#[get("/")]
async fn index() -> String {
    let response = reqwest::get("https://www.index.hr/najnovije")
    .await
    .unwrap()
    .text()
    .await;

let a = response.unwrap();

let document = Html::parse_document(&a);
let ul_selector = Selector::parse("ul.latest").unwrap();
let li_selector = Selector::parse("li").unwrap();
let ul = document.select(&ul_selector).into_iter();


let mut data_članci:Vec<ArticleInfo>=vec![];
for članak in ul{
    let mut date:String;
    let mut url:String;
    let mut img:String;
    let mut title:String;
    let mut summary:String;

for element in članak.select(&li_selector) {
    let mut sekcija:&str="";
    let fragment =  Html::parse_fragment(&element.inner_html());
    let mut selector = Selector::parse("span.num").unwrap();
    let date_time = fragment.select(&selector).next().unwrap().inner_html();
    selector = Selector::parse("span.desc").unwrap();
    let date_value = date_time+&fragment.select(&selector).next().unwrap().inner_html().to_string();
    date = date_value;

    let fragment =  Html::parse_fragment(&element.inner_html());
    selector = Selector::parse("a[href]").unwrap();
    let url_current = fragment.select(&selector).into_iter();
    url="http://fbi.com:8000/članak/".to_string();
    for u in url_current {
        if u.value().attr("href").unwrap().contains(".aspx") {
            let a = u.value().attr("href").unwrap().to_string();
            match &a {
                vijesti if vijesti.contains("/vijesti/") => {sekcija="vijesti"},
                magazin if magazin.contains("/magazin/") => {sekcija="magazin";},
                shopping if shopping.contains("/shopping/") => {sekcija="shopping"},
                sport if sport.contains("/sport/") => {sekcija="sport"},
                ljubimci if ljubimci.contains("/ljubimci/") => {sekcija="ljubimci"},
                chill if chill.contains("/chill/") => {sekcija="chill"},
                auto if auto.contains("/auto/") => {sekcija="auto"},
                fit if fit.contains("/fit/") => {sekcija="fit"},
                food if food.contains("/food/") => {sekcija="food"},
                _ => {sekcija="nepoznata"}
            }
            url += &a[a.find(".aspx").unwrap()-7..a.find(".aspx").unwrap()];
            println!("{}",url);
        }
    }

    selector = Selector::parse("img").unwrap();
    let img_a = fragment.select(&selector).next().unwrap();
    img = img_a.value().attr("src").unwrap().to_string();


    selector = Selector::parse(r#"h3[class="title"]"#).unwrap();
    let title_a = fragment.select(&selector).next().unwrap().inner_html().to_string();
    title = title_a;

    let mut title = title.replace("<span class=\"title-parsed-text\">", "");
    title = title.replace("</span>", "");
    
    let selector = Selector::parse(r#"span[class="summary"]"#).unwrap();
    let summary_a = fragment.select(&selector).next().unwrap().inner_html().to_string();
    summary = summary_a;

    summary = summary.replace("</span>", "");
    summary = summary.replace("&nbsp;", " ");
     summary = summary.replace("&lt;", "<");
     summary = summary.replace("&gt;", ">");

     data_članci.push(ArticleInfo{title:title,summary:summary,image:img,date:date,url:url,sekcija:sekcija.to_string()});
}

}

let json_string_to_save_as = serde_json::to_string(&data_članci).unwrap();
json_string_to_save_as
}


#[get("/komentari/<id>/<sort>")]
 async fn daj_komentare(id:&str,sort:u16) -> String {
    let url =format!("https://www.index.hr/api/comments?sortBy={}&commentThreadId={}&skip=0&take=20",sort,id);
    println!("{}",url);
    let response = reqwest::get(url)
    .await
    .unwrap()
    .text()
    .await;

    let mut slovnjaci_json:Entry = serde_json::from_str(response.unwrap().as_str()).unwrap();
    let mut refresh_count = slovnjaci_json.total_count/20;
    if refresh_count > 0{
        refresh_count += 1;
    }
    println!("Refresh count = {}, Total count = {}",refresh_count,slovnjaci_json.total_count);

    slovnjaci_json.total_count=slovnjaci_json.total_count;
    slovnjaci_json.total_count_with_replies=slovnjaci_json.total_count_with_replies;

    for x in 1..refresh_count{
        let url =format!("https://www.index.hr/api/comments?sortBy={}&commentThreadId={}&skip={}&take=20",2,id,x*20);
        println!("{}",url);
        let response = reqwest::get(url)
        .await
        .unwrap()
        .text()
        .await;
        let mut slovnjaci_json_1:Entry = serde_json::from_str(response.unwrap().as_str()).unwrap();

        slovnjaci_json.comments.append(&mut slovnjaci_json_1.comments);
    }


    let ser = serde_json::to_string(&slovnjaci_json).unwrap();

    ser
    
 }


#[get("/članak/<id>")]
 async fn daj_članak(id:&str) -> String {

    
    let url =format!("https://www.index.hr/clanak.aspx?id={}",id);
    println!("{}",url);
    let response = reqwest::get(url)
    .await
    .unwrap()
    .text()
    .await;

let a = response.unwrap();
let b=&a[a.find("commentThreadId=").unwrap()+16..a.find("commentThreadId=").unwrap()+7+16];

let document = Html::parse_document(&a);

let mut article_text:String=String::new();
let mut article_authors:String = String::new();
let mut article_title:String = String::new();


let div_selector = Selector::parse("div.article-title-holder").unwrap();

let h1_selector = Selector::parse("h1").unwrap();

let ul = document.select(&div_selector).next().unwrap();
for element in ul.select(&h1_selector) {
    article_title.push_str(element.inner_html().trim());
}

                                let selector = Selector::parse("div.text-holder").unwrap();
                                let p_selector = Selector::parse("p").unwrap();
                                let p = document.select(&selector).next().unwrap();
                                for element in p.select(&p_selector) {
                                    article_text.push_str(&element.inner_html());
                                }
                                let selector = Selector::parse("div.flex-1").unwrap();
                                let p = document.select(&selector).next().unwrap();

                                let selector = Selector::parse("script").unwrap();
                               
                                for element in document.select(&selector) {
                                   if element.inner_html().contains("var authors = (")
                                   {
                                    let a = element.inner_html().find("var authors = (").unwrap();
                                    let b = &element.inner_html()[a+"var authors = (".len()..];
                                    let c = b.find(").map(").unwrap();
                                    let d = &b[..c];
                                    article_authors = d.to_string();
                                    break;
                                   }
                                }

                                let author_authors:Vec<Author>= serde_json::from_str(&article_authors).unwrap();
                                article_title = article_title.replace("<span class=\"title-parsed-text\">", "");
                                article_title = article_title.replace("</span>", "");
                                article_text = article_text.replace("&nbsp;", " ");
                                article_text = article_text.replace("&lt;", "<");
                                article_text = article_text.replace("&gt;", ">");
                              let  article:ArticleSummary = ArticleSummary { title: String::from(&article_title),comment_thread_id:b.to_string(),text: article_text, date: p.inner_html(), authors: author_authors, url: id.to_string() };
                              let json_string_to_save_as = serde_json::to_string(&article).unwrap();
                              json_string_to_save_as
    
}


#[get("/odgovori/<comment_id>")]
 async fn daj_odgovore(comment_id:u64) -> String {
    let url =format!("https://www.index.hr/api/comments/replies?commentId={}&skip=0&take=20",comment_id);
    println!("{}",url);
    let response = reqwest::get(url)
    .await
    .unwrap()
    .text()
    .await;

    let  slovnjaci_json:KomentarOdgovor = serde_json::from_str(response.unwrap().as_str()).unwrap();
    let ser = serde_json::to_string(&slovnjaci_json).unwrap();

    ser
 }


 #[get("/korisnik/<user_id>")]
 async fn daj_korisnika(user_id:u64) -> String {

    let mut učitane_poveznice_oglasa: Vec<String> = vec![];

    učitane_poveznice_oglasa.push(format!("https://www.index.hr/api/comments/user?createdById={}&skip=0&take=20",user_id));

    let client:ClientBuilder = reqwest::ClientBuilder::new();
    let client:Box<Client> = Box::new(client.gzip(true).build().unwrap());
    let mut trenutni_broj_oglasa:u32 = 0; let  ukupni_broj_oglasa:usize = učitane_poveznice_oglasa.len();
    let iteratator_poveznica_oglasa = futures::stream::iter(
        učitane_poveznice_oglasa.into_iter().map(|poveznica_oglasa| {

        let client1 = &client;
        trenutni_broj_oglasa+=1;
        async move {
        match client1.get(&poveznica_oglasa)
        .header(reqwest::header::USER_AGENT, format!(r"Mozilla/5.0 (Windows NT 10.0; WOW64; Trident/7.0; rv:11.0) like Gecko"))//,trenutni_broj_oglasa))
        .header(header::HeaderName::from_static("accept"),header::HeaderValue::from_static("application/json, text/plain, */*"))
        .header(header::HeaderName::from_static("content-type"),header::HeaderValue::from_static("application/json"))
        .send()
            .await {
                Ok(resp) => {
                    match resp.status(){
                    StatusCode::OK => {
                        match resp.bytes().await {
                            Ok(slovnjaci_bajt) => {

                                let mut json_old_a:Entry = Entry{total_count:0,total_count_with_replies:0,comments:vec![]};
                                    let data1 = fs::read_to_string(format!("./spremljeno/{}.json",user_id)).unwrap_or_else(|error| {
                                        if error.kind() == ErrorKind::NotFound {
                                             json_old_a = Entry{total_count:0,total_count_with_replies:0,comments:vec![]};
                                             "".to_string()
                                        } else {
                                            panic!("Problem opening the file: {:?}", error);
                                        }
                                    });
                                    if data1.len() > 0 {
                                    json_old_a = serde_json::from_str(&data1).unwrap();
                                }


                                let slovnjaci_json:Entry = serde_json::from_slice(&slovnjaci_bajt).unwrap();
                                let mut refresh_count = &slovnjaci_json.total_count_with_replies/20;
                                println!("Refresh count = {}, Total count = {}",refresh_count,slovnjaci_json.total_count_with_replies);

                                json_old_a.total_count=slovnjaci_json.total_count;
                                json_old_a.total_count_with_replies=slovnjaci_json.total_count_with_replies;
                                for com in &json_old_a.comments{
                                    for kom in &slovnjaci_json.comments{
                                        if kom.comment_id == com.comment_id{
                                            refresh_count=0; 
                                        }
                                    }
                                }
                                let mut current=0;
                                let mut svi_komentari:Vec<Komentar> = vec![]; 
                                loop{
                                    if current > refresh_count
                                    {
                                        println!("broj komentara {}",svi_komentari.len());
                                        let poruka_spremljeno = format!("Spremljeno {}/{} oglasa, primljeno {} slovnjaka sa poveznice {}",user_id,ukupni_broj_oglasa,slovnjaci_bajt.len(), poveznica_oglasa);
                                        eprintln!("{}",poruka_spremljeno.green().bold());
                                        let mut datoteka_oglasa = OpenOptions::new()
                                        .create(true)
                                        .write(true)
                                        .open(format!("./spremljeno/{}.json",user_id))
                                        .unwrap();
                                        let mut temp_ids:Vec<u32> = vec![];
                                        for com in &json_old_a.comments{
                                            temp_ids.push(com.comment_id);
                                        }
                                        for sv in svi_komentari{
                                            if temp_ids.contains(&sv.comment_id)
                                            {
                                                continue;
                                            }
                                            else
                                            {
                                                json_old_a.comments.push(sv);
                                            }
                                        }
                                        let ser = serde_json::to_string(&json_old_a).unwrap();
                                        let podatci = ser.as_bytes();
                                        datoteka_oglasa.write_all(&podatci).expect("Zabuna prilikom upisivanja podataka!");
                                        
                                        break;
                                    }
                                    match client1.get(format!("https://www.index.hr/api/comments/user?createdById={}&skip={}&take=20",user_id,current*20))
                                    .header(reqwest::header::USER_AGENT, format!(r"Mozilla/5.0 (Windows NT 10.0; WOW64; Trident/7.0; rv:11.0) like Gecko"))//,trenutni_broj_oglasa))
                                   .header(header::HeaderName::from_static("accept"),header::HeaderValue::from_static("application/json, text/plain"))
                                    .send().await {
                                     Ok(resp1) => {
                                         match resp1.json::<Entry>().await {
                                             Ok(mut slovnjaci1) => {
                                                svi_komentari.append(&mut slovnjaci1.comments);
                                                   for com in &json_old_a.comments{
                                                    for kom in &slovnjaci1.comments{
                                                        if kom.comment_id == com.comment_id{
                                                            refresh_count=0; 
                                                        }
                                                    }
                                                }
                                             },
                                             Err(zabuna2) => {eprintln!("ERR0R: {}",zabuna2)}
                                         }
                                     },
                                     Err(zabuna1) => {eprintln!("ERR0R: {}",zabuna1)}


                                    }

                                    current+=1;
                                }
                        }
                            Err(zabuna) => eprintln!("Zabuna {} prilikom učitavanja slovnjaka, {}",zabuna, poveznica_oglasa),
                        }
                        
                    },
                    _=>{
                        match resp.text().await {
                            Ok(slovnjaci) => {
                                eprintln!("{}",slovnjaci.red().bold());
                            },
                            Err(zabuna) => eprintln!("JEBIGA"),
                        }
                    }
                    }
                }
                Err(zabuna) => {
                    let poruka_zabuna = format!("Zabuna {} prilikom preuzimanja, {}",zabuna,poveznica_oglasa);
                    eprintln!("{}",poruka_zabuna.yellow().bold());
                },
            }
        }
    })
    ).buffer_unordered(100).collect::<Vec<()>>();
    iteratator_poveznica_oglasa.await;
   let data = fs::read_to_string(format!("./spremljeno/{}.json",user_id))
   .expect("Unable to read file");
data
 }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,daj_članak,daj_komentare,daj_odgovore,daj_korisnika]).mount("/public", FileServer::from("static/"))
}
