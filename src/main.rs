use eframe::{egui::{CentralPanel,ScrollArea,Separator,Context}, epi::App, run_native, NativeOptions};
use std::fs;
use std::fs::metadata;

const APP_NAME: &str = "Archivos";

struct MyFiles {
    path:String,
    prev_path:String
}

impl MyFiles {
    fn new()-> MyFiles {
        Self {
        path:String::from("/"),
        prev_path:String::from("/")
        }
    }
}

impl App for MyFiles{
    fn update(&mut self, ctx:  &Context, _frame: &eframe::epi::Frame){
        CentralPanel::default().show(ctx, |ui| {
            if ui.button("return").clicked(){
                self.path=self.prev_path.to_owned();
            }
            ScrollArea::vertical().show(ui, |ui| {
                for file in fs::read_dir(&self.path).unwrap() {
                    let file_path =file.as_ref().unwrap().path().to_str().unwrap().to_string().to_owned();
                    if ui.button(file_path.to_owned()).clicked(){
                        if is_dir(file_path.to_owned()){
                            self.prev_path=self.path.to_owned();
                            self.path = file_path.to_owned();
                        }else{
                            println!("{}",file_path);
                        }
                    }
                        ui.separator();
                }
            });
        });
    }

    fn name(&self)->&str{
        &APP_NAME
    }
}


fn main(){

    let options = NativeOptions::default();
    let app =MyFiles::new();

    run_native(
        Box::new(app),
        options,
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
