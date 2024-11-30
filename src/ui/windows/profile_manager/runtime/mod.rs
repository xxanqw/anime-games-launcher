use adw::prelude::*;
use relm4::prelude::*;

use crate::prelude::*;

pub mod wine;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeSettingsPageInput {
    SetValues(RuntimeProfileSettings),
    UpdateWineValues(LinuxWineProfileRuntimeSettings)
}

#[derive(Debug)]
pub struct RuntimeSettingsPage {
    wine_page: AsyncController<wine::LinuxWineRuntimeSettingsPage>,

    settings: RuntimeProfileSettings
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for RuntimeSettingsPage {
    type Init = ();
    type Input = RuntimeSettingsPageInput;
    type Output = RuntimeProfileSettings;

    view! {
        adw::PreferencesPage {
            set_title: "Runtime",

            add = &model.wine_page.widget().clone() -> adw::PreferencesGroup {
                #[watch]
                set_visible: matches!(model.settings, RuntimeProfileSettings::LinuxWine { .. })
            }
        }
    }

    async fn init(_init: Self::Init, root: Self::Root, sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let model = Self {
            wine_page: wine::LinuxWineRuntimeSettingsPage::builder()
                .launch(())
                .forward(sender.input_sender(), RuntimeSettingsPageInput::UpdateWineValues),

            settings: RuntimeProfileSettings::default()
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) {
        match message {
            RuntimeSettingsPageInput::SetValues(settings) => {
                self.settings = settings;

                if let RuntimeProfileSettings::LinuxWine(wine) = &self.settings {
                    self.wine_page.emit(wine::LinuxWineRuntimeSettingsPageInput::SetValues(*wine));
                }
            }

            RuntimeSettingsPageInput::UpdateWineValues(settings) => {
                if let RuntimeProfileSettings::LinuxWine(wine) = &mut self.settings {
                    *wine = settings;
                }

                let _ = sender.output(self.settings.clone());
            }
        }
    }
}
