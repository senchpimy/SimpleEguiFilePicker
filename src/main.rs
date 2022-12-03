use eframe::{egui::{CentralPanel,ScrollArea,TopBottomPanel,Ui,Context}, run_native, NativeOptions};
use std::fs;
use std::fs::metadata;
use egui::widgets::Link;
use directories::BaseDirs;

const APP_NAME: &str = "Files";

struct MyFiles {
    path:String,
    prev_path:String,
    search:String,
}

impl MyFiles {
    fn new()-> MyFiles {
            Self {
            path:BaseDirs::new().unwrap().home_dir().to_str().unwrap().to_string(),
            prev_path:String::from("/"),
            search:"".to_owned(),
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
                    let file_path =file.as_ref().unwrap().path().to_str().unwrap().to_string().to_owned();
                    let linku = Link::new(file_path.to_owned());

                    if self.search.len()==0{
                        render_files(file_path,linku,ui,&mut self.prev_path,&mut self.path,frame);
                    }else{
                        if file_path.contains(&self.search){
                        render_files(file_path,linku,ui,&mut self.prev_path, &mut self.path, frame);
                        }
                    }}
            });
        TopBottomPanel::bottom("lalalaal aqui valo de escribir mensajes").show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| ui.text_edit_multiline(&mut self.search));
            });
        });
    }
}

pub fn run_window(){
    
    let options = NativeOptions::default();
    let app = MyFiles::new();
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

fn render_files(file_path:String, linku:Link, ui:&mut Ui, prev_path:&mut String, path:&mut String,frame: &mut eframe::Frame){
    if is_dir(file_path.to_owned()){
        if ui.add(linku).clicked(){
            *prev_path = path.to_owned();
            *path = file_path.to_owned();
        }
        ui.separator();
    }else{
        if ui.add(linku).clicked(){
            print!("{}",file_path);
            frame.close();
        }
        ui.separator();
    }
}

 fn main() {
     run_window()
 }
