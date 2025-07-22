use directories::BaseDirs;
use eframe::{
    egui::{self, CentralPanel, Context, Layout, ScrollArea, TopBottomPanel, Ui},
    run_native, NativeOptions,
};
use egui::widgets::Link;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const APP_NAME: &str = "Files";

pub struct MyFiles {
    path: PathBuf,
    search: String,
    type_to_search: Vec<String>,
    // CAMBIO: Almacenamos los bytes de la imagen directamente.
    // 'static es porque los incluimos en el binario con include_bytes!
    directory_image_bytes: &'static [u8],
    file_cache: Vec<PathBuf>,
}

impl MyFiles {
    fn new(ext_vec: Vec<String>) -> MyFiles {
        let home_dir = BaseDirs::new()
            .map(|dirs| dirs.home_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("/"));

        let mut app = Self {
            path: home_dir.clone(),
            search: String::new(),
            type_to_search: ext_vec,
            // CAMBIO: Simplemente asignamos los bytes de la imagen.
            // La macro include_bytes! devuelve &'static [u8].
            directory_image_bytes: include_bytes!("../directory.png"),
            file_cache: Vec::new(),
        };
        app.refresh_cache();
        app
    }

    fn refresh_cache(&mut self) {
        self.file_cache.clear();
        if let Ok(entries) = fs::read_dir(&self.path) {
            let mut paths: Vec<PathBuf> = entries
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| {
                    p.file_name()
                        .and_then(|n| n.to_str())
                        .map(|s| !s.starts_with('.'))
                        .unwrap_or(false)
                })
                .collect();
            paths.sort();
            self.file_cache = paths;
        }
    }
}

// CAMBIO: El parámetro 'frame' ya no es necesario aquí, pero lo mantenemos por la firma.
impl eframe::App for MyFiles {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Home").clicked() {
                    if let Some(dirs) = BaseDirs::new() {
                        self.path = dirs.home_dir().to_path_buf();
                        self.refresh_cache();
                    }
                }
                if ui.button("Return").clicked() {
                    if let Some(parent) = self.path.parent() {
                        self.path = parent.to_path_buf();
                        self.refresh_cache();
                    }
                }
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Current Path: {}", self.path.display()));
            ui.separator();

            ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui| {
                    let search_lower = self.search.to_lowercase();
                    let mut new_path: Option<PathBuf> = None;
                    let mut close_window = false;
                    for file_path in &self.file_cache {
                        let file_name =
                            file_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                        if !search_lower.is_empty()
                            && !file_name.to_lowercase().contains(&search_lower)
                        {
                            continue;
                        }
                        if render_file_or_dir(
                            file_path,
                            ui,
                            // CAMBIO: Ya no pasamos el contexto, no es necesario para la imagen.
                            self.directory_image_bytes,
                            &self.type_to_search,
                        ) {
                            if file_path.is_dir() {
                                new_path = Some(file_path.to_path_buf());
                            } else {
                                close_window = true;
                            }
                        }
                    }
                    if let Some(path) = new_path {
                        self.path = path;
                        self.refresh_cache();
                    }
                    if close_window {
                        // CAMBIO: Así es como se cierra la ventana ahora.
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        TopBottomPanel::bottom("search_bar").show(ctx, |ui| {
            ui.with_layout(Layout::top_down_justified(egui::Align::Center), |ui| {
                if ui.text_edit_singleline(&mut self.search).changed() {
                    // No es necesario refrescar aquí si ya lo haces al mostrar los archivos
                }
            });
        });
    }
}

// CAMBIO: La firma de la función se actualiza.
fn render_file_or_dir(
    file_path: &Path,
    ui: &mut Ui,
    img_bytes: &'static [u8],
    extensions: &[String],
) -> bool {
    let is_dir = match fs::metadata(file_path) {
        Ok(md) => md.is_dir(),
        Err(_) => return false,
    };

    let mut clicked = false;
    ui.horizontal(|ui| {
        if is_dir {
            // CAMBIO: Se crea la imagen desde los bytes y se ajusta el tamaño.
            let image = egui::Image::new(egui::ImageSource::Bytes {
                uri: "bytes://directory.png".into(), // URI único para el cacheo de textura
                bytes: img_bytes.into(),
            })
            .fit_to_exact_size(egui::vec2(20.0, 12.0));
            ui.add(image);
        } else {
            ui.label("📄"); // Icono simple de archivo
        }

        if ui
            .add(Link::new(
                file_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown"),
            ))
            .clicked()
        {
            if !is_dir {
                if extensions.is_empty()
                    || extensions
                        .iter()
                        .any(|ext| file_path.to_str().unwrap_or("").ends_with(ext))
                {
                    if let Err(e) = open_file(file_path) {
                        eprintln!("Failed to open file: {}", e);
                    }
                }
            }
            clicked = true;
        }
    });
    ui.separator();
    clicked
}

fn open_file(path: &Path) -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(["/C", "start", "", path.to_str().unwrap()])
        .spawn()?;

    #[cfg(target_os = "macos")]
    Command::new("open").arg(path).spawn()?;

    #[cfg(target_os = "linux")]
    Command::new("xdg-open").arg(path).spawn()?;

    Ok(())
}

pub fn run_window(app: MyFiles) {
    let options = NativeOptions::default();
    // CAMBIO: La clausura de creación ahora debe devolver un Result.
    // Simplemente envolvemos el resultado en Ok().
    run_native(APP_NAME, options, Box::new(|_cc| Ok(Box::new(app))))
        .expect("Failed to run egui app"); // Es buena práctica manejar el Result de run_native
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let extensions = match args.len() {
        1 => Vec::new(),
        2 => match args[1].as_str() {
            "images" => vec![
                "png".to_string(),
                "jpg".to_string(),
                "jpeg".to_string(),
                "gif".to_string(),
            ],
            "videos" => vec!["mp4".to_string(), "webm".to_string()],
            "documents" => vec![
                "doc".to_string(),
                "docx".to_string(),
                "pdf".to_string(),
                "ppt".to_string(),
            ],
            _ => vec![args[1].clone()],
        },
        _ => args[1..].to_vec(),
    };

    let app = MyFiles::new(extensions);
    run_window(app);
}
