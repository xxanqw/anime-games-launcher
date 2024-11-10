use adw::prelude::*;
use relm4::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommonGeneralSettingsPageInput {
    SetValues(CommonGeneralProfileSettings),
    Update
}

#[derive(Debug, Clone)]
pub struct CommonGeneralSettingsPage {
    settings: CommonGeneralProfileSettings,

    show_terminal: adw::SwitchRow
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for CommonGeneralSettingsPage {
    type Init = ();
    type Input = CommonGeneralSettingsPageInput;
    type Output = CommonGeneralProfileSettings;

    view! {
        adw::PreferencesGroup {
            set_title: "Common",

            #[local_ref]
            show_terminal -> adw::SwitchRow {
                set_title: "Open terminal window",
                set_subtitle: "Launch the game with a terminal window",

                #[watch]
                set_active: model.settings.show_terminal,

                connect_active_notify => CommonGeneralSettingsPageInput::Update
            }
        }
    }

    async fn init(_init: Self::Init, root: Self::Root, _sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let model = Self {
            settings: CommonGeneralProfileSettings::default(),

            show_terminal: adw::SwitchRow::new()
        };

        let show_terminal = &model.show_terminal;

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) {
        match message {
            CommonGeneralSettingsPageInput::SetValues(settings) => self.settings = settings,

            CommonGeneralSettingsPageInput::Update => {
                self.settings.show_terminal = self.show_terminal.is_active();

                let _ = sender.output(self.settings.clone());
            }
        }
    }
}
