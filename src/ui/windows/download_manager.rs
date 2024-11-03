use adw::prelude::*;
use relm4::prelude::*;

use crate::prelude::*;

static mut WINDOW: Option<adw::Window> = None;

#[derive(Debug)]
pub struct DownloadManagerWindow {
    graph: AsyncController<Graph>
}

#[derive(Debug, Clone)]
pub enum DownloadManagerWindowMsg {
    Show,
    Hide
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for DownloadManagerWindow {
    type Init = ();
    type Input = DownloadManagerWindowMsg;
    type Output = ();

    view! {
        window = adw::Window {
            set_size_request: (700, 560),
            set_title: Some("Download manager"),

            set_hide_on_close: true,
            set_modal: true,

            add_css_class?: APP_DEBUG.then_some("devel"),

            adw::Clamp {
                set_maximum_size: 600,

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    model.graph.widget(),
                }
            }

            // adw::PreferencesPage {
            //     adw::PreferencesGroup {
            //         model.graph.widget(),
            //     },

            //     adw::PreferencesGroup {
            //         gtk::Box {
            //             set_orientation: gtk::Orientation::Horizontal,
            //             set_spacing: 16,

            //             // adw::PreferencesGroup {
            //             //     adw::ActionRow {
            //             //         set_title: "Current speed",
            //             //         #[watch]
            //             //         set_subtitle: &format!("{}/s", pretty_bytes(model.speed).0),
            //             //     }
            //             // },

            //             // adw::PreferencesGroup {
            //             //     adw::ActionRow {
            //             //         set_title: "Average speed",
            //             //         #[watch]
            //             //         set_subtitle: &format!("{}/s", pretty_bytes(model.avg_speed).0),
            //             //     }
            //             // },

            //             // adw::PreferencesGroup {
            //             //     adw::ActionRow {
            //             //         set_title: "Time elapsed",
            //             //         #[watch]
            //             //         set_subtitle: &pretty_seconds(model.elapsed),
            //             //     }
            //             // },

            //             // adw::PreferencesGroup {
            //             //     adw::ActionRow {
            //             //         set_title: "Current ETA",
            //             //         set_subtitle: "amogus",
            //             //     }
            //             // },

            //             // adw::PreferencesGroup {
            //             //     adw::ActionRow {
            //             //         #[watch]
            //             //         set_title: match model.state {
            //             //             DownloadsAppState::None => "",
            //             //             DownloadsAppState::Downloading => "Total download",
            //             //             DownloadsAppState::Extracting => "Total extracted",
            //             //             DownloadsAppState::StreamUnpacking => "Total unpacked",
            //             //             DownloadsAppState::Verifying => "Total verified",
            //             //         },
    
            //             //         #[watch]
            //             //         set_subtitle: &pretty_bytes(model.total).0.to_string(),
            //             //     }
            //             // }
            //         }
            //     }
            // }
        }
    }

    async fn init(_init: Self::Init, root: Self::Root, _sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let model = Self {
            graph: Graph::builder()
                .launch(GraphInit {
                    width: 600,
                    height: 300,
                    window_size: 200,
                    color: (1.0, 0.0, 0.0)
                })
                .detach()
        };

        let widgets = view_output!();

        unsafe {
            WINDOW = Some(widgets.window.clone());
        }

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        match msg {
            DownloadManagerWindowMsg::Show => unsafe {
                if let Some(window) = WINDOW.as_ref() {
                    if let Some(main_window) = MAIN_WINDOW.as_ref() {
                        let main_window = main_window.upcast_ref::<gtk::Window>();

                        window.set_transient_for(Some(main_window));
                    }

                    window.present();
                }
            }

            DownloadManagerWindowMsg::Hide => unsafe {
                if let Some(window) = WINDOW.as_ref() {
                    window.close();
                }
            }
        }
    }
}
