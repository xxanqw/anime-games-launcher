mod main_window;
mod profile_manager;
mod download_manager;

pub use main_window::{
    MainWindow,
    MainWindowMsg,
    WINDOW as MAIN_WINDOW
};

pub use main_window::library_page::SyncGameCommand;

pub use profile_manager::{
    ProfileManagerWindow,
    ProfileManagerWindowMsg
};

pub use profile_manager::builder::{
    ProfileBuilderWindow,
    ProfileBuilderWindowInput,
    ProfileBuilderWindowOutput
};

pub use download_manager::{
    DownloadManagerWindow,
    DownloadManagerWindowMsg
};
