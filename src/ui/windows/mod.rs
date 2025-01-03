pub mod main_window;
pub mod profile_wizard;
pub mod profile_manager;
pub mod download_manager;

pub mod prelude {
    pub use super::main_window::{
        MainWindow,
        MainWindowMsg,
        WINDOW as MAIN_WINDOW
    };

    pub use super::main_window::library_page::SyncGameCommand;

    pub use super::profile_manager::{
        ProfileManagerWindow,
        ProfileManagerWindowMsg
    };

    pub use super::profile_manager::builder::{
        ProfileBuilderWindow,
        ProfileBuilderWindowInput
    };

    pub use super::download_manager::{
        DownloadManagerWindow,
        DownloadManagerWindowMsg
    };
}
