use crate::games;

use crate::config;
use crate::config::games::prelude::*;

use crate::ui::components::game_card::CardInfo;

use crate::ui::components::tasks_queue::download_diff_task::{
    DownloadDiffQueuedTask,
    DiffOrigin
};

use crate::games::integrations::Game;
use crate::games::integrations::standards::diff::DiffInfo;

use crate::ui::windows::loading::check_addons::{
    AddonsListEntry,
    get_game_addons_downloads
};

use super::MainAppMsg;

type HeapResult<T> = Result<T, Box<MainAppMsg>>;

#[inline]
fn is_installed(game: &Game, game_path: &str, edition: &str) -> HeapResult<bool> {
    game.is_game_installed(game_path, edition)
        .map_err(|err| Box::new(MainAppMsg::ShowToast {
            title: format!("Unable to verify {} installation", game.manifest.game_title),
            message: Some(err.to_string())
        }))
}

#[inline]
fn get_diff(game: &Game, edition: impl AsRef<str>, game_path: impl AsRef<str>) -> HeapResult<DiffInfo> {
    game.get_game_diff(game_path.as_ref(), edition.as_ref())
        .map_err(|err| MainAppMsg::ShowToast {
            title: format!("Unable to find {} version diff", game.manifest.game_title),
            message: Some(err.to_string())
        })?
        .and_then(|diff| diff.diff)
        .ok_or_else(|| Box::new(MainAppMsg::ShowToast {
            title: format!("{} is not installed", game.manifest.game_title),
            message: None
        }))
}

#[inline]
fn get_download(game: &Game, edition: &str) -> HeapResult<DiffInfo> {
    game.get_game_download(edition.as_ref())
        .map_err(|err| Box::new(MainAppMsg::ShowToast {
            title: format!("Unable to find {} download info", game.manifest.game_title),
            message: Some(err.to_string())
        }))
        .map(|download| download.download)
}

#[inline]
fn get_diff_or_download(game: &Game, game_path: &str, edition: &str) -> HeapResult<DiffInfo> {
    is_installed(game, game_path, edition)?
        .then(|| get_diff(game, edition, game_path))
        .unwrap_or_else(|| get_download(game, edition))
}

#[inline]
fn get_settings(game: &Game, config: &config::Config) -> HeapResult<GameSettings> {
    config.games.get_game_settings(game)
        .map_err(|err| Box::new(MainAppMsg::ShowToast {
            title: format!("Unable to find {} settings", game.manifest.game_title),
            message: Some(err.to_string())
        }))
}

#[inline]
fn get_addons(game: &Game, game_info: &CardInfo, edition: &str, enabled_addons: &[GameEditionAddon]) -> HeapResult<Vec<AddonsListEntry>> {
    get_game_addons_downloads(game_info, game, edition, enabled_addons)
        .map_err(|err| Box::new(MainAppMsg::ShowToast {
            title: format!("Unable to get {} addons", game.manifest.game_title),
            message: Some(err.to_string())
        }))
}

pub struct DownloadGameResult {
    pub game_task: Box<DownloadDiffQueuedTask>,
    pub download_addons: Vec<AddonsListEntry>
}

#[inline]
pub fn get_download_game_task(game_info: &CardInfo, config: &config::Config) -> HeapResult<DownloadGameResult> {
    let game = unsafe {
        games::get_unsafe(game_info.get_name())
    };

    let settings = get_settings(game, config)?;

    // Game installation path
    let game_path = &settings.paths[game_info.get_edition()].game;

    // Enabled game addons
    let enabled_addons = &settings.addons[game_info.get_edition()];

    Ok(DownloadGameResult {
        game_task: Box::new(DownloadDiffQueuedTask {
            card_info: game_info.clone(),
            download_path: game_path.clone(),
            diff_info: get_diff_or_download(
                game,
                &game_path.to_string_lossy(),
                game_info.get_edition()
            )?,
            diff_origin: DiffOrigin::Game
        }),

        download_addons: get_addons(game, game_info, game_info.get_edition(), enabled_addons)?
    })
}
