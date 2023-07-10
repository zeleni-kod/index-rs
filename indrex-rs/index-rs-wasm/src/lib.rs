use std::sync::{Arc, Mutex};
use eframe::egui::{ScrollArea, RichText};
use eframe::egui;
use eframe::emath::Numeric;
use serde_derive::{Serialize, Deserialize};

use egui::plot;
use egui::*;
use plot::{
     Bar, BarChart,
    Legend, Plot, PlotPoint
};

use chrono::{NaiveDateTime, Timelike};




use std::{self, fmt};




#[derive(PartialEq, Eq)]
enum Chart {
    GaussBars
}

impl Default for Chart {
    fn default() -> Self {
        Self::GaussBars
    }
}

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
    url: String,
    sekcija:String
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




#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum Method {
    Get,
    Post,
}

enum Download {
    None,
    InProgress,
    Done(ehttp::Result<ehttp::Response>),
}

pub struct DemoApp {
    url: String,
    method: Method,
    request_body: String,
    download: Arc<Mutex<Download>>,
    articles: Vec<ArticleInfo>,
    initial_fetch:bool,
    show_settings:bool,
    load_all_comments_thread:bool,
    load_all_comments_user:bool,
    article_summary:ArticleSummary,
    fetch:bool,
    komentari:Entry,
    komentari_korisnika:Entry,
    odgovori:KomentarOdgovor,
    history:Vec<String>
}

impl Default for DemoApp {
    fn default() -> Self {
        Self {
            url: "http://fbi.com:8000".to_owned(),
            method: Method::Get,
            request_body: r#"["posting some json"]"#.to_owned(),
            download: Arc::new(Mutex::new(Download::None)),
            articles:vec![],
            article_summary:ArticleSummary { title: "".to_string(), text: "".to_string(), date:"".to_string(),authors:vec![], comment_thread_id: "".to_string(), url: "".to_string() },
            komentari:Entry{ total_count: 0, total_count_with_replies: 0, comments: vec![] },
            komentari_korisnika:Entry{ total_count: 0, total_count_with_replies: 0, comments: vec![] },
            odgovori:KomentarOdgovor { comment_id: 0, total_count: 0, comments: vec![] },
            initial_fetch:true,
            history:vec![],
            fetch:true,
            show_settings:false,
            load_all_comments_thread:true,
            load_all_comments_user:true
        }
    }
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {


        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            egui::Area::new("my_area")
            .anchor(egui::Align2::CENTER_TOP, vec2(0.0, 0.0))
            .show(ctx, |ui| {

  

       ui.horizontal(|ui|{
                if ui.button(RichText::new("🌲").heading().font(FontId::proportional(50.0)).color(Color32::GREEN)).clicked(){
                self.url= "http://fbi.com:8000/".to_string();
                self.fetch=true;
            }
            if ui.button(RichText::new("⬅").heading().font(FontId::proportional(50.0)).color(Color32::GREEN)).clicked(){
                if self.history.len() > 0{
                self.url= self.history.pop().unwrap();
            }else{
                self.url="http://fbi.com:8000/".to_string();
            }
                self.fetch=true;
        }
        egui::Area::new("my_area2")
        // .fixed_pos(egui::pos2(32.0, 32.0))
         .anchor(egui::Align2::RIGHT_BOTTOM, vec2(0.0, 0.0))
         .show(ctx, |ui| {
            if ui.button(RichText::new("POSTAVKA").heading().font(FontId::proportional(50.0)).color(Color32::GREEN)).clicked(){
                self.url= "http://fbi.com:8000/".to_string();
                self.show_settings=true;
            }
        });
        
    });
    
    });

            //    });
});
        egui::CentralPanel::default().show(ctx, |ui| {

            if self.show_settings {
               
                let my_frame = egui::containers::Frame {
                    inner_margin: egui::style::Margin { left: 10., right: 10., top: 10., bottom: 10. },
                    outer_margin: egui::style::Margin { left: 10., right: 10., top: 10., bottom: 10. },
                    rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
                    shadow: eframe::epaint::Shadow { extrusion: 1.0, color: Color32::YELLOW },
                    fill: Color32::LIGHT_GREEN,
                    stroke: egui::Stroke::new(0.0, Color32::GOLD),
                };
                egui::Area::new("my_area3")
                .anchor(egui::Align2::CENTER_TOP, vec2(0.0, 0.0))
                .show(ctx, |ui| {
                my_frame
    .fill(egui::Color32::RED)
    .show(ui, |ui| {
        ui.checkbox(&mut self.load_all_comments_thread, RichText::new("Učitaje sve komentare iz dretve").heading().font(FontId::proportional(50.0)).color(Color32::GREEN));
        if  ui.button("U redu!").clicked() {
            self.show_settings=false;
            }
        });
    });
                  
                
            }
            ScrollArea::both().show(ui,|ui|{
            if self.fetch  {

                let request = match self.method {
                    Method::Get => ehttp::Request::get(&self.url),
                    Method::Post => {
                        ehttp::Request::post(&self.url, self.request_body.as_bytes().to_vec())
                    }
                };
                let download_store = self.download.clone();
                *download_store.lock().unwrap() = Download::InProgress;
                let ctx = ctx.clone();
                ehttp::fetch(request, move |response| {
                    *download_store.lock().unwrap() = Download::Done(response);
                    ctx.request_repaint(); // Wake up UI thread
                });
                self.fetch=false;
            }


            let download: &Download = &self.download.lock().unwrap();
            match download {
                Download::None => {}
                Download::InProgress => {
                    ui.label(RichText::new("🌲✋⏱ 🎄 ⬅ ⬆ ➡ 🌱🌳🌴🌿💚").color(Color32::GREEN).font(FontId::proportional(300.0)));
                }
                Download::Done(response) => match response {
                    Err(err) => {
                        ui.label(err);
                    }
                    Ok(response) => {
                        
                       if response.url.contains("/%C4%8Dlanak/") 
                       {
                        self.article_summary=serde_json::from_str(&response.text().unwrap()).unwrap();
                        let slovnjaci_json:ArticleSummary = serde_json::from_str(&response.text().unwrap()).unwrap();
                        self.url= format!("http://fbi.com:8000/komentari/{}/2",slovnjaci_json.comment_thread_id);


                        self.fetch=true;
                        self.article_summary=slovnjaci_json;

                        

                    }
                    else if response.url.contains("/komentari/"){
                        self.komentari = serde_json::from_str(&response.text().unwrap()).unwrap();
                        ui.label(RichText::new(&self.article_summary.title).color(Color32::GOLD).font(FontId::proportional(40.0)));
                        for author in &self.article_summary.authors{
                        ui.label(RichText::new(&author.full_name).color(Color32::RED).font(FontId::proportional(30.0)));
                    }
                        ui.label(RichText::new(&self.article_summary.text).color(Color32::WHITE).font(FontId::proportional(18.0)));
                       // ui.label(&self.article_summary.text);
                        for comment in &self.komentari.comments{
                            ui.add_space(12.0);
                            if ui.button(RichText::new(&comment.poster_full_name).color(Color32::RED)
                        .font(FontId::proportional(25.0))).clicked(){
                                let url = &self.url;
                                self.history.push(url.to_string());
                                self.url= format!("http://fbi.com:8000/korisnik/{}",&comment.poster_id);
  
                                self.fetch=true;
                            }
                            ui.label(RichText::new(&comment.content).color(Color32::WHITE)
                            .font(FontId::proportional(25.0)));
                            ui.label(RichText::new(&comment.created_date_utc).color(Color32::RED)
                            .font(FontId::proportional(25.0)));
                           if ui.button(RichText::new(format!("👍{}      👎{} - Odgovora {}",&comment.number_of_likes,&comment.number_of_dislikes,&comment.reply_count)).color(Color32::YELLOW).font(FontId::proportional(25.0))).clicked()
                            {
                                let url = &self.url;
                                self.history.push(url.to_string());
                                self.url= format!("http://fbi.com:8000/odgovori/{}",&comment.comment_id);

                                self.fetch=true;
                            };
                        }
                        if self.url.contains("/odgovori/") || self.url.contains("/korisnik/"){
                            self.fetch=true;
                        }
                        else{
                            self.fetch=false;
                        }
                    }
                    else if response.url.contains("/korisnik/"){
                        self.komentari_korisnika = serde_json::from_str(&response.text().unwrap()).unwrap();
                        let mut bars:Vec<Bar> = vec![];
                        let mut x = 0.5;
                        for komentar in &self.komentari_korisnika.comments{
                            let parse_from_str = NaiveDateTime::parse_from_str;
                           let parsed= parse_from_str(&komentar.created_date_utc, "%Y-%m-%dT%H:%M:%S%.f");
                         //  println!("{}");
                           // println!("{}",komentar.created_date_utc);
                           
                            bars.push(Bar::new(x,parsed.unwrap().hour().to_f64()+(parsed.unwrap().minute().to_f64()/60.0)).name(format!("{}\n{}\n👍{}      👎{} - Odgovora {}",&komentar.content,&komentar.created_date_utc,&komentar.number_of_likes,&komentar.number_of_dislikes,&komentar.reply_count)));
                            x+=1.0;
                        }
      

                        let chart1:BarChart=BarChart::new(bars).width(1.0).color(Color32::BLUE)
                        .name("Vrijeme");
                        let mut bars:Vec<Bar> = vec![];
                        let mut x = 0.5;
                        for komentar in &self.komentari_korisnika.comments{
                            bars.push(Bar::new(x,komentar.number_of_likes.to_f64()-komentar.number_of_dislikes.to_f64()).name(format!("{}\n{}\n👍{}      👎{} - Odgovora {}",&komentar.content,&komentar.created_date_utc,&komentar.number_of_likes,&komentar.number_of_dislikes,&komentar.reply_count)));
                            x+=1.0;
                        }

                        let chart2:BarChart=BarChart::new(bars).width(1.0).color(Color32::GREEN)

                        .width(1.0)
                        .name("Glasovi")
                        .stack_on(&[&chart1]);
                    


                        let mut bars:Vec<Bar> = vec![];
                        let mut x = 0.5;
                        for komentar in &self.komentari_korisnika.comments{
                            bars.push(Bar::new(x,komentar.reply_count.to_f64()).name(format!("{}\n{}\n👍{}      👎{} - Odgovora {}",&komentar.content,&komentar.created_date_utc,&komentar.number_of_likes,&komentar.number_of_dislikes,&komentar.reply_count)));
                            x+=1.0;
                        }

                        let chart3:BarChart=BarChart::new(bars).width(1.0).color(Color32::WHITE)
                        .width(1.0)
                        .name("Odgovori")
                        .stack_on(&[&chart2]);
                       let mut clicked_pos:PlotPoint = PlotPoint{x:0.0,y:0.0}; 
                       let mut clicked:bool = false;

                        Plot::new("Stacked Bar Chart Demo")
            .legend(Legend::default())
            .data_aspect(1.0)
            .include_y(24)
            .height(340.0)

           
                        .allow_scroll(false)
            .show(ui, |plot_ui| {
               plot_ui.bar_chart(chart1);
               plot_ui.bar_chart(chart2);
               plot_ui.bar_chart(chart3);

                if plot_ui.plot_clicked(){
                                clicked_pos = plot_ui.pointer_coordinate().unwrap();
                                clicked=true;
                    
                }
            });
            let len_com:i64  = self.komentari_korisnika.comments.len() as i64;
            let round:i64 = clicked_pos.x.round() as i64;
            if clicked && clicked_pos.x > 0.0 && round < len_com  {
                let url = &self.url;
            self.history.push(url.to_string());

                           self.url= format!("http://fbi.com:8000/članak/{}",&self.komentari_korisnika.comments[round as usize].relationship_entity_id);
                            //format!("http://fbi.com:8000/korisnik/{}",&comment.poster_id);

                            self.fetch=true;
            }

                        for comment in &self.komentari_korisnika.comments{
                            ui.add_space(12.0);
                            if ui.button(RichText::new(&comment.poster_full_name).color(Color32::RED)).clicked(){
                                let url = &self.url;
                                self.history.push(url.to_string());

                                self.url= format!("http://fbi.com:8000/članak/{}",&comment.relationship_entity_id);
                                //format!("http://fbi.com:8000/korisnik/{}",&comment.poster_id);

                                self.fetch=true;
                            }
                        
                            ui.label(&comment.content);
                            ui.label(&comment.created_date_utc);
                        }
                    }
                    else if response.url.contains("/odgovori/"){


                        self.odgovori = serde_json::from_str(&response.text().unwrap()).unwrap();
                        ui.label(RichText::new(&self.article_summary.title).color(Color32::GOLD).font(FontId::proportional(40.0)));
                        for author in &self.article_summary.authors{
                            ui.label(RichText::new(&author.full_name).color(Color32::RED).font(FontId::proportional(30.0)));
                        }
                        ui.label(RichText::new(&self.article_summary.text).color(Color32::WHITE).font(FontId::proportional(20.0)));

                       // ui.label(&self.article_summary.text);
                        for comment in &self.komentari.comments{
                            if comment.comment_id == self.odgovori.comment_id{
                            ui.add_space(12.0);
                            if ui.button(RichText::new(&comment.poster_full_name).color(Color32::RED)).clicked(){
                                let url = &self.url;
                                self.history.push(url.to_string());
                                self.url= format!("http://fbi.com:8000/korisnik/{}",&comment.poster_id);

                                self.fetch=true;
                            }
                            ui.label(&comment.content);
                            ui.label(&comment.created_date_utc);
                        
                           //if ui.button(RichText::new(format!("👍{}      👎{} - Odgovora {} ->",&comment.number_of_likes,&comment.number_of_dislikes,&comment.reply_count)).color(Color32::YELLOW)).clicked()
                            //{
                                for odogovir in &self.odgovori.comments{
                                    ui.add_space(12.0);
                                    if ui.button(RichText::new(&odogovir.poster_full_name).color(Color32::RED)).clicked(){
                                        let url = &self.url;
                                        self.history.push(url.to_string());
                                        self.url= format!("http://fbi.com:8000/korisnik/{}",&odogovir.poster_id);
 
                                        self.fetch=true;
                                    }
                                    ui.label(&odogovir.content);
                                    ui.label(&odogovir.created_date_utc);
                                }
                            }
                              //  self.url= format!("https://www.index.hr/api/comments/replies?commentId={}&skip=0&take=20",&comment.comment_id);
                              //  self.fetch=true;
                            //};
                        }



                    }
                    else{
                       // println!("{}",response.url);
                       
                        self.articles = serde_json::from_str(&response.text().unwrap()).unwrap();
                        let slovnjaci_json:Vec<ArticleInfo> = serde_json::from_str(&response.text().unwrap()).unwrap();
                       
                       for article in slovnjaci_json{
                        let mut color:Color32 = Color32::LIGHT_RED;
                        if article.sekcija.contains("magazin")
                        {
                            color = Color32::GOLD;
                        }else if article.sekcija.contains("sport"){
                            color = Color32::GREEN;
                        }
                        else if article.sekcija.contains("shopping"){
                            color = Color32::WHITE;
                        }
                        else if article.sekcija.contains("ljubimci"){
                            color = Color32::GREEN;
                        }
                        else if article.sekcija.contains("chill"){
                            color = Color32::BLACK;
                        }
                        else if article.sekcija.contains("auto"){
                            color = Color32::LIGHT_BLUE;
                        }
                        else if article.sekcija.contains("fit"){
                            color = Color32::LIGHT_GREEN;
                        }
                        else if article.sekcija.contains("mame"){
                            color = Color32::LIGHT_RED;
                        }
                        else if article.sekcija.contains("horoskop"){
                            color = Color32::DARK_BLUE;
                        }
                        else if article.sekcija.contains("food"){
                            color = Color32::KHAKI;
                        }

                        
                           ui.add_space(12.0);
                           ui.label(RichText::new(article.date).font(FontId::proportional(40.0)));
                           if ui.button(RichText::new(article.title).color(color).font(FontId::proportional(25.0))).clicked() {
                            // …

                            let url = &self.url;
                        self.history.push(url.to_string());
                        self.url=article.url;
                        self.fetch=true;
                        };
                           ui.label(RichText::new(article.summary).color(Color32::WHITE).font(FontId::proportional(18.0)));


           
           
                       }
                    }
                    }
                },
            }
        });
        });
    }
}




// ----------------------------------------------------------------------------

use eframe::epaint::{Color32, FontId};
use eframe::epaint::text::TextWrapping;
#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), wasm_bindgen::JsValue> {
    eframe::start_web(
        canvas_id,
        Default::default(),
        Box::new(|_cc| {
            let style = Style {
                visuals: Visuals::dark(),
                ..Style::default()
            };
            _cc.egui_ctx.set_style(style);
            Box::new(DemoApp::default())}),
    )?;
    Ok(())
}
