use adw::prelude::*;
use relm4::prelude::*;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CommonGeneralSettingsPage(CommonGeneralProfileSettings);

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for CommonGeneralSettingsPage {
    type Init = CommonGeneralProfileSettings;
    type Input = ();
    type Output = ();

    view! {
        adw::PreferencesGroup {
            set_title: "Common",

            adw::SwitchRow {
                set_title: "Open terminal window",
                set_subtitle: "Launch the game with a terminal window"
            }
        }
    }

    async fn init(init: Self::Init, root: Self::Root, _sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let model = Self(init);

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }
}
