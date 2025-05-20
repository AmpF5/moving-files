use std::{fs, path::Path};

use rfd::FileDialog;

use crate::{
    app::App,
    models::{
        file::{File, FileExtension},
        file_list::FileListType,
    },
};

pub fn match_file_extension(file_extension: &str) -> FileExtension {
    match file_extension {
        "webp" => FileExtension::Webp,
        "png" => FileExtension::Png,
        "jpg" => FileExtension::Jpg,
        "txt" => FileExtension::Txt,
        _ => FileExtension::NotImplemented,
    }
}

pub fn pick_folder(list_type: &FileListType) -> Option<std::path::PathBuf> {
    let title = match list_type {
        FileListType::FileListFrom => "Select folder to import files from",
        FileListType::FileListTo => "Select folder to import files to",
    };

    FileDialog::new().set_title(title).pick_folder()
}

pub fn move_selected_files(app: &mut App) -> color_eyre::Result<()> {
    let mut files_to_move: Vec<File> = app
        .files_from
        .items
        .iter()
        .filter(|f| f.is_selected)
        .cloned()
        .collect();

    for file in &mut files_to_move {
        let old_path = Path::new(&file.path);
        let new_path_string = format!("{}/{}.{}", app.files_to.path, file.name, file.extension);
        let new_path = Path::new(&new_path_string);
        fs::rename(old_path, new_path)?;
        file.path = new_path_string;
    }

    app.files_from.items.retain(|f| !f.is_selected);

    app.files_to.items.extend(files_to_move);

    Ok(())
}

pub fn load_files_via_file_explorer(app: &mut App, list_type: FileListType) {
    if let Some(folder_path) = pick_folder(&list_type) {
        let dir_path = folder_path.clone().into_os_string().into_string().unwrap();
        match fs::read_dir(folder_path) {
            std::result::Result::Ok(entries) => {
                let files: Vec<File> = entries
                    .filter_map(Result::ok)
                    .filter_map(|f| {
                        let path = f.path();

                        let path_string = path.to_string_lossy().to_string();

                        let file_name = path.file_name().and_then(|f| f.to_str())?;

                        let name: &str = file_name.rsplit_once('.')?.0;

                        let extension: String =
                            path.extension().and_then(|s| s.to_str())?.to_string();

                        Some(File::init(path_string, name.into(), extension))
                    })
                    .collect();

                match list_type {
                    FileListType::FileListFrom => {
                        app.files_from.items = files;
                        app.files_from.path = dir_path;
                        app.files_from.state.select(Some(0));
                    }
                    FileListType::FileListTo => {
                        app.files_to.items = files;
                        app.files_to.path = dir_path;
                    }
                }
            }
            Err(e) => eprint!("Error reading dir: {}", e),
        }
    }
}
