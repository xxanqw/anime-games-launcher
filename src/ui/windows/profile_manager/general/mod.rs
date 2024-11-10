use adw::prelude::*;
use relm4::prelude::*;

use crate::prelude::*;

pub mod common;
pub mod linux;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneralSettingsPageInput {
    SetValues(GeneralProfileSettings),
    UpdateCommonValues(CommonGeneralProfileSettings),
    UpdateLinuxValues(LinuxGeneralProfileSettings)
}

#[derive(Debug)]
pub struct GeneralSettingsPage {
    common_page: AsyncController<common::CommonGeneralSettingsPage>,
    linux_page: AsyncController<linux::LinuxGeneralSettingsPage>,

    settings: GeneralProfileSettings
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for GeneralSettingsPage {
    type Init = ();
    type Input = GeneralSettingsPageInput;
    type Output = GeneralProfileSettings;

    view! {
        adw::PreferencesPage {
            set_title: "General",

            add = model.common_page.widget(),

            add = &model.linux_page.widget().clone() -> adw::PreferencesGroup {
                #[watch]
                set_visible: matches!(model.settings, GeneralProfileSettings::Linux { .. })
            }
        }
    }

    async fn init(_init: Self::Init, root: Self::Root, sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let model = Self {
            common_page: common::CommonGeneralSettingsPage::builder()
                .launch(())
                .forward(sender.input_sender(), GeneralSettingsPageInput::UpdateCommonValues),

            linux_page: linux::LinuxGeneralSettingsPage::builder()
                .launch(())
                .forward(sender.input_sender(), GeneralSettingsPageInput::UpdateLinuxValues),

            settings: GeneralProfileSettings::default()
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) {
        match message {
            GeneralSettingsPageInput::SetValues(settings) => {
                self.settings = settings;

                match self.settings.clone() {
                    GeneralProfileSettings::Unknown(settings) => {
                        self.common_page.emit(common::CommonGeneralSettingsPageInput::SetValues(settings));
                    }

                    GeneralProfileSettings::Linux { common, linux } => {
                        self.common_page.emit(common::CommonGeneralSettingsPageInput::SetValues(common));
                        self.linux_page.emit(linux::LinuxGeneralSettingsPageInput::SetValues(linux));
                    }

                    _ => ()
                }
            }

            GeneralSettingsPageInput::UpdateCommonValues(settings) => {
                match &mut self.settings {
                    GeneralProfileSettings::Windows { common, .. } |
                    GeneralProfileSettings::Linux { common, .. } |
                    GeneralProfileSettings::Unknown(common) => *common = settings
                }

                let _ = sender.output(self.settings.clone());
            }

            GeneralSettingsPageInput::UpdateLinuxValues(settings) => {
                if let GeneralProfileSettings::Linux { linux, .. } = &mut self.settings {
                    *linux = settings;
                }

                let _ = sender.output(self.settings.clone());
            }
        }
    }
}
