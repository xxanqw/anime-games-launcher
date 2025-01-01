use std::collections::HashMap;

use adw::prelude::*;
use relm4::prelude::*;

#[derive(Debug, Clone)]
pub struct DynamicSettingsComponentGroup {
    pub group: adw::PreferencesGroup,
    pub components: HashMap<String, DynamicSettingsComponentRow>
}

impl DynamicSettingsComponentGroup {
    pub fn new(
        title: Option<impl AsRef<str>>,
        description: Option<impl AsRef<str>>,
        components: impl IntoIterator<Item = (String, DynamicSettingsComponentRow)>
    ) -> Self {
        let group = adw::PreferencesGroup::new();
        let components = HashMap::from_iter(components);

        if let Some(title) = title {
            group.set_title(title.as_ref());
        }

        if let Some(description) = description {
            group.set_description(Some(description.as_ref()));
        }

        for component in components.values() {
            match component {
                DynamicSettingsComponentRow::Switch(switch)        => group.add(switch),
                DynamicSettingsComponentRow::Text(text)             => group.add(text),
                DynamicSettingsComponentRow::Enum { component, .. } => group.add(component)
            }
        }

        Self {
            group,
            components
        }
    }
}

#[derive(Debug, Clone)]
pub enum DynamicSettingsComponentRow {
    Switch(adw::SwitchRow),
    Text(adw::EntryRow),

    Enum {
        component: adw::ComboRow,
        values: HashMap<String, String>
    }
}

impl DynamicSettingsComponentRow {
    pub fn new_switch(title: impl AsRef<str>, description: Option<impl AsRef<str>>, default: bool) -> Self {
        let switch = adw::SwitchRow::new();

        switch.set_title(title.as_ref());
        switch.set_active(default);

        if let Some(description) = description {
            switch.set_subtitle(description.as_ref());
        }

        Self::Switch(switch)
    }

    pub fn new_text(title: impl AsRef<str>, description: Option<impl AsRef<str>>, default: impl AsRef<str>) -> Self {
        let text = adw::EntryRow::new();

        text.set_title(title.as_ref());
        text.set_text(default.as_ref());

        if let Some(description) = description {
            text.set_tooltip(description.as_ref());
        }

        Self::Text(text)
    }

    pub fn new_enum(
        title: impl AsRef<str>,
        description: Option<impl AsRef<str>>,
        values: impl IntoIterator<Item = (String, String)>,
        default: impl AsRef<str>
    ) -> Self {
        let component = adw::ComboRow::new();
        let values = HashMap::from_iter(values);

        component.set_title(title.as_ref());

        if let Some(description) = description {
            component.set_subtitle(description.as_ref());
        }

        let model = gtk::StringList::new(&[]);
        let default = default.as_ref();

        for item in values.values() {
            model.append(item);
        }

        component.set_model(Some(&model));

        if let Some((k, _)) = values.keys().enumerate().find(|(_, k)| k == &default) {
            component.set_selected(k as u32);
        }

        Self::Enum {
            component,
            values
        }
    }
}

#[derive(Debug, Clone)]
pub enum DynamicSettingsComponentInput {
    AddGroup(DynamicSettingsComponentGroup),
    Clear
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DynamicSettingsComponentOutput {

}

#[derive(Debug)]
pub struct DynamicSettingsComponent {
    preferences_page: adw::PreferencesPage,
    groups: Vec<DynamicSettingsComponentGroup>
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for DynamicSettingsComponent {
    type Init = ();
    type Input = DynamicSettingsComponentInput;
    type Output = DynamicSettingsComponentOutput;

    view! {
        #[root]
        gtk::Box {
            #[local_ref]
            preferences_page -> adw::PreferencesPage,
        }
    }

    #[inline]
    async fn init(_init: Self::Init, root: Self::Root, _sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let model = Self {
            preferences_page: adw::PreferencesPage::new(),
            groups: Vec::new()
        };

        let preferences_page = &model.preferences_page;

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        match msg {
            DynamicSettingsComponentInput::AddGroup(group) => {
                self.preferences_page.add(&group.group);
                self.groups.push(group);
            }

            DynamicSettingsComponentInput::Clear => {
                for group in self.groups.drain(..) {
                    self.preferences_page.remove(&group.group);
                }
            }
        }
    }
}
