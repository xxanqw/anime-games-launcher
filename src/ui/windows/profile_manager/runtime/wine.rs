use adw::prelude::*;
use relm4::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct LinuxWineRuntimeSettingsPage {
    settings: LinuxWineProfileRuntimeSettings
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for LinuxWineRuntimeSettingsPage {
    type Init = ();
    type Input = ();
    type Output = ();

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
}
