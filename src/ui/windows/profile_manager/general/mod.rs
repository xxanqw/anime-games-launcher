use adw::prelude::*;
use relm4::prelude::*;

use crate::prelude::*;

pub mod common;
pub mod windows;
pub mod linux;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneralSettingsPageInput {
    SetValues(GeneralProfileSettings),
    UpdateCommonValues(CommonGeneralProfileSettings)
}

#[derive(Debug)]
pub struct GeneralSettingsPage {
    common_page: AsyncController<common::CommonGeneralSettingsPage>,
    windows_page: AsyncController<windows::WindowsGeneralSettingsPage>,
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

            add = &model.windows_page.widget().clone() -> adw::PreferencesGroup {
                #[watch]
                set_visible: matches!(model.settings, GeneralProfileSettings::Windows { .. })
            },

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

            windows_page: windows::WindowsGeneralSettingsPage::builder()
                .launch(())
                .detach(),

            linux_page: linux::LinuxGeneralSettingsPage::builder()
                .launch(())
                .detach(),

            settings: GeneralProfileSettings::default()
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) {
        match message {
            GeneralSettingsPageInput::SetValues(settings) => {
                self.settings = settings;

                if let GeneralProfileSettings::Unknown(settings) = &self.settings {
                    self.common_page.emit(common::CommonGeneralSettingsPageInput::SetValues(settings.clone()));
                }
            }

            GeneralSettingsPageInput::UpdateCommonValues(settings) => {
                self.settings = GeneralProfileSettings::Unknown(settings);

                let _ = sender.output(self.settings.clone());
            }
        }
    }
}
