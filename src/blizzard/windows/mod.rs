use crate::{
    blizzard::db,
    blizzard::windows::utils::{get_launcher_executable, get_manifests_path},
    error::Result,
    prelude::Game,
};

mod utils;

pub fn games() -> Result<Vec<Game>> {
    let launcher_executable = get_launcher_executable().unwrap();

    return db::read_all(&get_manifests_path(), &launcher_executable);
}

pub fn find(id: &str) -> Result<Game> {
    let launcher_executable = get_launcher_executable().unwrap();

    return db::read(id, &get_manifests_path(), &launcher_executable);
}
