use adw::prelude::*;
use relm4::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProfileFactoryComponent(Profile);

#[relm4::factory(async)]
impl AsyncFactoryComponent for ProfileFactoryComponent {
    type Init = Profile;
    type Input = ProfilePageMsg;
    type Output = ProfilePageMsg;
    type ParentWidget = adw::PreferencesGroup;
    type CommandOutput = ();

    view! {
        #[root]
        adw::ActionRow {
            set_title: self.0.name(),
            set_subtitle: &self.0.target_platform().to_string(),

            set_activatable: true,

            add_suffix = &gtk::Button {
                set_align: gtk::Align::Center,

                add_css_class: "circular",
                set_icon_name: "user-trash-symbolic",

                set_tooltip_text: Some("Delete profile"),

                connect_clicked[sender, index] => move |_| {
                    let _ = sender.output(ProfilePageMsg::DeleteProfile(index.clone()));
                }
            },

            connect_activated[sender, index] => move |_| {
                let _ = sender.output(ProfilePageMsg::EditProfile(index.clone()));
            },
        }
    }

    #[inline]
    async fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: AsyncFactorySender<Self>) -> Self {
        Self(init)
    }
}

#[derive(Debug, Clone)]
pub enum ProfilePageMsg {
    UpdateProfiles,
    CreateProfile,
    EditProfile(DynamicIndex),
    DeleteProfile(DynamicIndex)
}

#[derive(Debug)]
pub struct ProfilePage {
    manager_window: AsyncController<ProfileManagerWindow>,
    profiles: AsyncFactoryVecDeque<ProfileFactoryComponent>
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for ProfilePage {
    type Init = ();
    type Input = ProfilePageMsg;
    type Output = ();

    view! {
        #[root]
        adw::PreferencesPage {
            add = &adw::PreferencesGroup {
                set_title: "Profiles",

                #[wrap(Some)]
                set_header_suffix = &gtk::Button {
                    set_align: gtk::Align::Center,

                    add_css_class: "flat",
                    set_icon_name: "list-add-symbolic",

                    set_tooltip_text: Some("Create new profile"),

                    connect_clicked => ProfilePageMsg::CreateProfile
                },

                model.profiles.widget(),
            }
        }
    }

    async fn init(_init: Self::Init, root: Self::Root, sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let model = Self {
            manager_window: ProfileManagerWindow::builder()
                .launch(())
                .detach(),

            profiles: AsyncFactoryVecDeque::builder()
                .launch_default()
                .forward(sender.input_sender(), std::convert::identity)
        };

        sender.input(ProfilePageMsg::UpdateProfiles);

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        match msg {
            ProfilePageMsg::UpdateProfiles => {
                let config = config::get();

                let store = ProfilesStore::new(config.profiles.store.path);

                match store.list().await {
                    Ok(profiles) => {
                        let mut guard = self.profiles.guard();

                        guard.clear();

                        for profile in profiles {
                            guard.push_back(profile);
                        }
                    }

                    Err(err) => tracing::error!(?err, "Failed to list profiles")
                }
            }

            ProfilePageMsg::CreateProfile => {
                self.manager_window.emit(ProfileManagerWindowMsg::OpenWindow(Profile::new("New profile")));
            }

            ProfilePageMsg::EditProfile(index) => {
                if let Some(profile) = self.profiles.guard().get(index.current_index()) {
                    self.manager_window.emit(ProfileManagerWindowMsg::OpenWindow(profile.0.clone()));
                }
            }

            ProfilePageMsg::DeleteProfile(index) => ()
        }
    }
}
