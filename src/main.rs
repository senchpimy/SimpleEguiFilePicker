use eframe::{egui::{CentralPanel,ScrollArea,TopBottomPanel,Ui,Context}, run_native, NativeOptions};
use std::fs;
use std::fs::metadata;
use egui::widgets::Link;
use std::env;
use directories::BaseDirs;

const APP_NAME: &str = "Files";
//const IMAGES:Vec<&str> = vec!["png","jpg","jpeg","gif"];
//const VIDEOS:Vec<&str> = vec!["mp4","webm"];
//const DOCUMENTS:Vec<&str> =vec!["doc","docx","ppt","html","pdf","xml"];

pub struct MyFiles {
    path:String,
    prev_path:String,
    search:String,
    type_to_search:Vec<String>,
}

impl MyFiles {
    fn new(ext_vec:Vec<String>)-> MyFiles {
    println!("{:?}",&ext_vec);
    println!("creado");
            Self {
            path:BaseDirs::new().unwrap().home_dir().to_str().unwrap().to_string(),
            prev_path:String::from("/"),
            search:"".to_owned(),
            type_to_search:ext_vec,
            }

    }
}

impl eframe::App for MyFiles{
    fn update(&mut self, ctx:  &Context, frame: &mut eframe::Frame){
        CentralPanel::default().show(ctx, |ui| {
            if ui.button("return").clicked(){
                self.path=self.prev_path.to_owned();
            }
            ScrollArea::vertical().show(ui, |ui| {
                for file in fs::read_dir(&self.path).unwrap() {
                        let file_path = file.as_ref().unwrap().path().to_str().unwrap().to_string();
                        if self.search.len()==0{
                            render_files(file_path,ui,&mut self.prev_path,&mut self.path,frame,&self.type_to_search);
                        }else{
                            if file_path.contains(&self.search){
                                render_files(file_path,ui,&mut self.prev_path, &mut self.path, frame,&self.type_to_search);
                            }}
                }
            });
        TopBottomPanel::bottom("lalalaal aqui valo de escribir mensajes").show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| ui.text_edit_multiline(&mut self.search));
            });
        });
    }
}

pub fn run_window(app:MyFiles){
    let options = NativeOptions::default();
        run_native(
         APP_NAME,
         options,
         Box::new(|_cc| Box::new(app)),
      );
    }

fn is_dir(value:String)->bool{
    let md = metadata(value).unwrap();
    if md.is_dir(){
    true
    }else{
        false
    }
}

fn render_files(file_path:String, ui:&mut Ui, prev_path:&mut String, path:&mut String,frame: &mut eframe::Frame,extensions:&Vec<String>){
    if is_dir(file_path.to_owned()){
        if ui.add(Link::new(file_path.to_owned())).clicked(){
            *prev_path = path.to_owned();
            *path = file_path.to_owned();
        }
        ui.separator();
    }else{
        if extensions.len()==0{
                if ui.add(Link::new(file_path.to_owned())).clicked(){
                    println!("{}",file_path);
                    frame.close();
                }
                ui.separator();
        }else{
        for suffix in extensions{
             if file_path.ends_with(suffix){
                if ui.add(Link::new(file_path.to_owned())).clicked(){
                    println!("{}",file_path);
                    frame.close();
                }
                ui.separator();
            }
    }}}
}

 fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();
    println!("{:?}",&args);
    println!("{}",args_len);
    if args_len>2{
        let app = MyFiles::new(args[1..args.len()].to_vec());
        run_window(app);
    }else if args_len==2{
        let extensions:Vec<String>;
        match args[1].as_str() {
            "images" => extensions = vec!["png".to_string(),"jpg".to_string(),"jpeg".to_string(),"gif".to_string()],
           _ => extensions = vec![args[1].to_string()]  ,
        }
        let app = MyFiles::new(extensions);
        run_window(app);
    }else if args_len==1{
        let app = MyFiles::new(Vec::new());
        run_window(app);
    }
 }
