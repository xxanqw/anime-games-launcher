use std::sync::Arc;

use adw::prelude::*;
use relm4::prelude::*;

use tokio::sync::mpsc::UnboundedSender;

use unic_langid::LanguageIdentifier;

use crate::prelude::*;

use super::*;

#[derive(Debug)]
pub enum GameLibraryDetailsMsg {
    SetGameInfo {
        manifest: Arc<GameManifest>,
        edition: GameEdition,
        listener: UnboundedSender<SyncGameCommand>
    },

    SetGameInstallationStatus(InstallationStatus),
    SetGameLaunchInfo(GameLaunchInfo)
}

#[derive(Debug)]
pub struct GameLibraryDetails {
    card: AsyncController<CardComponent>,
    background: AsyncController<LazyPictureComponent>,

    listener: Option<UnboundedSender<SyncGameCommand>>,

    title: String,
    developer: String,
    publisher: String,

    status: InstallationStatus,
    launch_info: GameLaunchInfo
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for GameLibraryDetails {
    type Init = ();
    type Input = GameLibraryDetailsMsg;
    type Output = ();

    view! {
        adw::Clamp {
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                set_margin_top: 16,
                set_spacing: 16,

                gtk::Label {
                    set_halign: gtk::Align::Start,

                    add_css_class: "title-1",

                    #[watch]
                    set_label: &model.title
                },

                model.background.widget() {
                    add_css_class: "card"
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,

                    set_spacing: 12,

                    // Play button
                    gtk::Button {
                        #[watch]
                        set_css_classes: match model.launch_info.status {
                            GameLaunchStatus::Normal    => &["pill", "suggested-action"],
                            GameLaunchStatus::Warning   => &["pill", "warning-action"],
                            GameLaunchStatus::Dangerous => &["pill", "destructive-action"],
                            GameLaunchStatus::Disabled  => &["pill"]
                        },

                        #[watch]
                        set_visible: [InstallationStatus::Installed, InstallationStatus::UpdateAvailable].contains(&model.status),

                        #[watch]
                        set_sensitive: model.launch_info.status != GameLaunchStatus::Disabled,

                        #[watch]
                        set_tooltip?: model.launch_info.hint.as_ref()
                            .map(|hint| {
                                // FIXME: IO-heavy thing (there's around 6 update calls each time)
                                let config = config::get();

                                let lang = config.general.language.parse::<LanguageIdentifier>();

                                match &lang {
                                    Ok(lang) => hint.translate(lang),
                                    Err(_) => hint.default_translation()
                                }
                            }),

                        adw::ButtonContent {
                            set_icon_name: "media-playback-start-symbolic",

                            set_label: "Play"
                        }
                    },

                    // Update / Install (execute diff) button
                    gtk::Button {
                        add_css_class: "pill",

                        #[watch]
                        set_visible: model.status != InstallationStatus::Installed,

                        adw::ButtonContent {
                            set_icon_name: "document-save-symbolic",

                            #[watch]
                            set_label: if model.status == InstallationStatus::NotInstalled {
                                "Install"
                            } else {
                                "Update"
                            }
                        }
                    }
                }
            }
        }
    }

    async fn init(_init: Self::Init, root: Self::Root, _sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let model = Self {
            card: CardComponent::builder()
                .launch(CardComponent::medium())
                .detach(),

            background: LazyPictureComponent::builder()
                .launch(LazyPictureComponent::default())
                .detach(),

            listener: None,

            title: String::new(),
            developer: String::new(),
            publisher: String::new(),

            status: InstallationStatus::NotInstalled,
            launch_info: GameLaunchInfo::default()
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            GameLibraryDetailsMsg::SetGameInfo { manifest, edition, listener } => {
                let config = config::get();

                let lang = config.general.language.parse::<LanguageIdentifier>();

                let title = match &lang {
                    Ok(lang) => manifest.game.title.translate(lang),
                    Err(_) => manifest.game.title.default_translation()
                };

                let developer = match &lang {
                    Ok(lang) => manifest.game.developer.translate(lang),
                    Err(_) => manifest.game.developer.default_translation()
                };

                let publisher = match &lang {
                    Ok(lang) => manifest.game.publisher.translate(lang),
                    Err(_) => manifest.game.publisher.default_translation()
                };

                self.listener = Some(listener.clone());

                self.title = title.to_string();
                self.developer = developer.to_string();
                self.publisher = publisher.to_string();

                self.card.emit(CardComponentInput::SetImage(Some(ImagePath::lazy_load(&manifest.game.images.poster))));

                // Little trolling. I think you can sorry me.
                let date = time::OffsetDateTime::now_utc();

                let image = if date.month() == time::Month::April && date.day() == 1 {
                    tracing::info!("＜( ￣︿￣)");

                    ImagePath::resource("images/april-fools.jpg")
                } else {
                    ImagePath::lazy_load(&manifest.game.images.background)
                };

                self.background.emit(LazyPictureComponentMsg::SetImage(Some(image)));

                // Request game installation status update.
                {
                    let listener = listener.clone();
                    let sender = sender.clone();
                    let edition_name = edition.name.clone();

                    tokio::spawn(async move {
                        let (send, recv) = tokio::sync::oneshot::channel();

                        if let Err(err) = listener.send(SyncGameCommand::GetStatus { edition: edition_name, listener: send }) {
                            tracing::error!(?err, "Failed to request game installation status");

                            return;
                        }

                        match recv.await {
                            Ok(Ok(status)) => {
                                sender.input(GameLibraryDetailsMsg::SetGameInstallationStatus(status));
                            }

                            Ok(Err(err)) => tracing::error!(?err, "Failed to request game installation status"),
                            Err(err) => tracing::error!(?err, "Failed to request game installation status")
                        }
                    });
                }

                // Request game launching info update.
                tokio::spawn(async move {
                    let (send, recv) = tokio::sync::oneshot::channel();

                    if let Err(err) = listener.send(SyncGameCommand::GetLaunchInfo { edition: edition.name, listener: send }) {
                        tracing::error!(?err, "Failed to request game launch info");

                        return;
                    }

                    match recv.await {
                        Ok(Ok(info)) => {
                            sender.input(GameLibraryDetailsMsg::SetGameLaunchInfo(info));
                        }

                        Ok(Err(err)) => tracing::error!(?err, "Failed to request game launch info"),
                        Err(err) => tracing::error!(?err, "Failed to request game launch info")
                    }
                });
            }

            GameLibraryDetailsMsg::SetGameInstallationStatus(status) => self.status = status,
            GameLibraryDetailsMsg::SetGameLaunchInfo(info) => self.launch_info = info
        }
    }
}
