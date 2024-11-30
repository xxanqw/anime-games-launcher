use adw::prelude::*;
use relm4::prelude::*;

pub mod builder;
pub mod general;
pub mod runtime;

use crate::prelude::*;

#[derive(Debug)]
pub struct ProfileManagerWindow {
    window: Option<adw::PreferencesWindow>,

    general_page: AsyncController<general::GeneralSettingsPage>,
    runtime_page: AsyncController<runtime::RuntimeSettingsPage>,

    profile: Option<Profile>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileManagerWindowMsg {
    OpenWindow(Profile),
    CloseWindow,

    UpdateGeneralSettings(GeneralProfileSettings),
    UpdateRuntimeSettings(RuntimeProfileSettings)
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for ProfileManagerWindow {
    type Init = ();
    type Input = ProfileManagerWindowMsg;
    type Output = Profile;

    view! {
        #[root]
        window = adw::PreferencesWindow {
            set_size_request: (700, 560),
            set_title: Some("Profile manager"),

            set_hide_on_close: true,
            set_modal: true,

            add_css_class?: crate::APP_DEBUG.then_some("devel"),

            add = model.general_page.widget(),
            add = model.runtime_page.widget(),
        }
    }

    async fn init(_init: Self::Init, root: Self::Root, sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let mut model = Self {
            window: None,

            general_page: general::GeneralSettingsPage::builder()
                .launch(())
                .forward(sender.input_sender(), ProfileManagerWindowMsg::UpdateGeneralSettings),

            runtime_page: runtime::RuntimeSettingsPage::builder()
                .launch(())
                .forward(sender.input_sender(), ProfileManagerWindowMsg::UpdateRuntimeSettings),

            profile: None
        };

        let widgets = view_output!();

        model.window = Some(widgets.window.clone());

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        match msg {
            ProfileManagerWindowMsg::OpenWindow(profile) => {
                self.general_page.emit(general::GeneralSettingsPageInput::SetValues(profile.general.clone()));
                self.runtime_page.emit(runtime::RuntimeSettingsPageInput::SetValues(profile.runtime.clone()));

                self.profile = Some(profile);

                if let Some(window) = self.window.as_ref() {
                    window.present();
                }
            }

            ProfileManagerWindowMsg::CloseWindow => {
                if let Some(window) = self.window.as_ref() {
                    window.close();
                }
            }

            ProfileManagerWindowMsg::UpdateGeneralSettings(settings) => {
                if let Some(profile) = &mut self.profile {
                    profile.general = settings;
                }
            }

            ProfileManagerWindowMsg::UpdateRuntimeSettings(settings) => {
                if let Some(profile) = &mut self.profile {
                    profile.runtime = settings;
                }
            }
        }
    }
}
