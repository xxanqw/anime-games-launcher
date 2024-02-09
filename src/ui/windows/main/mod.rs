use gtk::prelude::*;
use adw::prelude::*;
use relm4::prelude::*;

pub mod store_page;
pub mod library_page;
pub mod profile_page;

pub use store_page::{StorePageApp, StorePageAppMsg};
pub use library_page::{LibraryPageApp, LibraryPageAppMsg};
pub use profile_page::{ProfilePageApp, ProfilePageAppMsg};

pub static mut WINDOW: Option<adw::Window> = None;

#[derive(Debug)]
pub struct MainApp {
    store_page: AsyncController<StorePageApp>,
    library_page: AsyncController<LibraryPageApp>,
    profile_page: AsyncController<ProfilePageApp>
}

#[derive(Debug, Clone)]
pub enum MainAppMsg {
    
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for MainApp {
    type Init = ();
    type Input = MainAppMsg;
    type Output = ();

    view! {
        window = adw::Window {
            set_size_request: (1200, 800),
            set_title: Some("Anime Games Launcher"),

            add_css_class?: crate::APP_DEBUG.then_some("devel"),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    add_css_class: "flat",

                    #[wrap(Some)]
                    set_title_widget = &adw::ViewSwitcher {
                        set_policy: adw::ViewSwitcherPolicy::Wide,

                        set_stack: Some(&view_stack)
                    }
                },

                #[name = "view_stack"]
                adw::ViewStack {
                    add = &adw::Clamp {
                        model.store_page.widget(),
                    } -> {
                        set_title: Some("Store"),
                        set_icon_name: Some("folder-download-symbolic")
                    },

                    add = &adw::Clamp {
                        model.library_page.widget(),
                    } -> {
                        set_title: Some("Library"),
                        set_icon_name: Some("applications-games-symbolic")
                    },

                    add = &adw::Clamp {
                        model.profile_page.widget(),
                    } -> {
                        set_title: Some("Profile"),
                        set_icon_name: Some("person-symbolic")
                    }
                }
            }
        }
    }

    async fn init(_init: Self::Init, root: Self::Root, sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let model = Self {
            store_page: StorePageApp::builder()
                .launch(())
                .detach(),

            library_page: LibraryPageApp::builder()
                .launch(())
                .detach(),

            profile_page: ProfilePageApp::builder()
                .launch(())
                .detach()
        };

        let widgets = view_output!();

        unsafe {
            WINDOW = Some(widgets.window.clone());
        }

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            
        }
    }
}
