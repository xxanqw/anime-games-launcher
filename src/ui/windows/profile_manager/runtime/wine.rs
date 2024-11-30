use adw::prelude::*;
use relm4::prelude::*;

use crate::prelude::*;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinuxWineRuntimeSettingsPageInput {
    SetValues(LinuxWineProfileRuntimeSettings)
}

#[derive(Debug, Clone)]
pub struct LinuxWineRuntimeSettingsPage {
    settings: LinuxWineProfileRuntimeSettings
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for LinuxWineRuntimeSettingsPage {
    type Init = ();
    type Input = LinuxWineRuntimeSettingsPageInput;
    type Output = LinuxWineProfileRuntimeSettings;

    view! {
        adw::PreferencesGroup {
            set_title: "Wine"
        }
    }

    async fn init(_init: Self::Init, root: Self::Root, _sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let model = Self {
            settings: LinuxWineProfileRuntimeSettings::default()
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, _sender: AsyncComponentSender<Self>) {
        match message {
            LinuxWineRuntimeSettingsPageInput::SetValues(settings) => {
                self.settings = settings;
            }
        }
    }
}
