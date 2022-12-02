use eframe::{egui::{CentralPanel,ScrollArea,Context}, run_native, NativeOptions};
use std::fs;
use std::fs::metadata;
use egui::widgets::Link;

const APP_NAME: &str = "Archivos";

struct MyFiles {
    path:String,
    prev_path:String
}

impl MyFiles {
    fn new()-> MyFiles {
        if cfg!(windows) {
            Self {
            path:String::from("/"),
            prev_path:String::from("/")
            }
        }else if cfg!(unix){
            Self {
            path:String::from("/"),
            prev_path:String::from("/")
            }
        }else if cfg!(target_os = "macos"){
            Self {
            path:String::from("/"),
            prev_path:String::from("/")
            }
        }else{
            Self {
            path:String::from("/"),
            prev_path:String::from("/")
            }

        }
    }
}

impl eframe::App for MyFiles{
    fn update(&mut self, ctx:  &Context, _frame: &mut eframe::Frame){
        CentralPanel::default().show(ctx, |ui| {
            if ui.button("return").clicked(){
                self.path=self.prev_path.to_owned();
            }
            ScrollArea::vertical().show(ui, |ui| {
                for file in fs::read_dir(&self.path).unwrap() {
                    let file_path =file.as_ref().unwrap().path().to_str().unwrap().to_string().to_owned();
                    let linku = Link::new(file_path.to_owned());

                    if is_dir(file_path.to_owned()){
                        if ui.link(file_path.to_owned()).clicked(){
                            self.prev_path=self.path.to_owned();
                            self.path = file_path.to_owned();
                        }
                    ui.separator();
                    }else{
                        if ui.add(linku).clicked(){
                            println!("{}",file_path);
                        }
                    ui.separator();
                }}
            });
        });
    }
}

pub fn run_window(){

    let options = NativeOptions::default();
    let app =MyFiles::new();

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
