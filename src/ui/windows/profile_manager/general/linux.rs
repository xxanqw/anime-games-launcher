use adw::prelude::*;
use relm4::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinuxGeneralSettingsPageInput {
    SetValues(LinuxGeneralProfileSettings),
    Update
}

#[derive(Debug, Clone)]
pub struct LinuxGeneralSettingsPage {
    settings: LinuxGeneralProfileSettings,

    gamemode: adw::SwitchRow
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for LinuxGeneralSettingsPage {
    type Init = ();
    type Input = LinuxGeneralSettingsPageInput;
    type Output = LinuxGeneralProfileSettings;

    view! {
        adw::PreferencesGroup {
            set_title: "Linux",

            #[local_ref]
            gamemode -> adw::SwitchRow {
                set_title: "Gamemode",
                set_subtitle: "Prioritize game process and enable system optimizations",

                #[watch]
                set_active: model.settings.gamemode,

                connect_active_notify => LinuxGeneralSettingsPageInput::Update
            }
        }
    }

    async fn init(_init: Self::Init, root: Self::Root, _sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let model = Self {
            settings: LinuxGeneralProfileSettings::default(),

            gamemode: adw::SwitchRow::new()
        };

        let gamemode = &model.gamemode;

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) {
        match message {
            LinuxGeneralSettingsPageInput::SetValues(settings) => self.settings = settings,

            LinuxGeneralSettingsPageInput::Update => {
                self.settings.gamemode = self.gamemode.is_active();

                let _ = sender.output(self.settings.clone());
            }
        }
    }
}
