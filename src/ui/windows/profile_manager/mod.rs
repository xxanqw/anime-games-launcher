use adw::prelude::*;
use relm4::prelude::*;

pub mod component_page;
pub mod environment_page;

use component_page::*;
use environment_page::*;

use crate::prelude::*;

static mut WINDOW: Option<adw::PreferencesWindow> = None;

#[relm4::widget_template(pub)]
impl WidgetTemplate for ComboSwitchRow {
    view! {
        adw::ComboRow {
            add_suffix = &gtk::Switch {
                set_valign: gtk::Align::Center
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileManagerWindow {
    profile: Option<Profile>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileManagerWindowMsg {
    OpenWindow(Profile),
    CloseWindow
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for ProfileManagerWindow {
    type Init = ();
    type Input = ProfileManagerWindowMsg;
    type Output = ();

    view! {
        #[root]
        window = adw::PreferencesWindow {
            set_size_request: (700, 560),
            set_title: Some("Profile manager"),

            set_hide_on_close: true,
            set_modal: true,

            add_css_class?: crate::APP_DEBUG.then_some("devel"),

            add = &adw::PreferencesPage {
                set_title: "General",

                add = &adw::PreferencesGroup {
                    set_title: "Profile",

                    adw::EntryRow {
                        set_title: "Profile name",

                        #[watch]
                        set_text: model.profile.as_ref()
                            .map(|profile| profile.name())
                            .unwrap_or_default()
                    },

                    adw::ComboRow {
                        set_title: "Platform",
                        set_subtitle: "Environment emulated by this profile",

                        set_model: Some(&{
                            let list = gtk::StringList::new(&[]);

                            for platform in TargetPlatform::list() {
                                list.append(&platform.to_string());
                            }

                            list
                        }),

                        #[watch]
                        set_selected?: model.profile.as_ref()
                            .and_then(|profile| {
                                TargetPlatform::list()
                                    .iter()
                                    .position(|platform| platform == profile.target_platform())
                                    .map(|pos| pos as u32)
                            })
                    }
                }
            },

            add = &adw::PreferencesPage {
                set_title: "Runtime",

                add = &adw::PreferencesGroup {
                    set_title: "Profile info",

                    adw::EntryRow {
                        set_title: "Profile name"
                    },

                    adw::SwitchRow {
                        set_title: "Set default",
                        set_subtitle: "Use this profile by default with newly installed games"
                    }
                }
            }
        }
    }

    async fn init(_init: Self::Init, root: Self::Root, sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let model = Self {
            profile: None
        };

        let widgets = view_output!();

        unsafe {
            WINDOW = Some(widgets.window.clone());
        }

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        match msg {
            ProfileManagerWindowMsg::OpenWindow(profile) => unsafe {
                self.profile = Some(profile);

                if let Some(window) = WINDOW.as_ref() {
                    window.present();
                }
            }

            ProfileManagerWindowMsg::CloseWindow => unsafe {
                if let Some(window) = WINDOW.as_ref() {
                    window.close();
                }
            }
        }
    }
}
