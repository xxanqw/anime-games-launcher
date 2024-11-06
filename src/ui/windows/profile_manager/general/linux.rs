use adw::prelude::*;
use relm4::prelude::*;

#[derive(Debug, Clone)]
pub struct LinuxGeneralSettingsPage;

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for LinuxGeneralSettingsPage {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        adw::PreferencesGroup {
            set_title: "Linux"
        }
    }

    async fn init(_init: Self::Init, root: Self::Root, _sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let model = Self;

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }
}
