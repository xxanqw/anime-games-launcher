use relm4::prelude::*;
use relm4::component::*;
use relm4::factory::*;

use gtk::prelude::*;
use adw::prelude::*;

mod dlc;

pub use dlc::*;

use crate::games::integrations::standards::dlc::{
    Dlc,
    DlcGroup
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DlcGroupComponent {
    pub info: DlcGroup
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DlcGroupComponentInput {

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DlcGroupComponentOutput {

}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for DlcGroupComponent {
    type Init = DlcGroup;
    type Input = DlcGroupComponentInput;
    type Output = DlcGroupComponentOutput;

    view! {
        #[root]
        adw::PreferencesGroup {
            set_title: &model.info.title
        }
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = Self {
            info: init
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        
    }
}
