use std::path::PathBuf;

use crate::{
    error::{Error, ErrorKind, Result},
    util::registry,
};

pub fn get_manifests_path() -> PathBuf {
    PathBuf::from("C:\\ProgramData\\GOG.com\\Galaxy\\storage\\galaxy-2.0.db")
}

pub fn get_launcher_executable() -> Result<PathBuf> {
    let launcher_executable = registry::get_local_machine_reg_key("GOG.com\\GalaxyClient\\paths")
        .and_then(|launcher_paths_reg| registry::get_value(&launcher_paths_reg, "client"))
        .map(PathBuf::from)
        .and_then(|launcher_path| {
            registry::get_local_machine_reg_key("GOG.com\\GalaxyClient")
                .and_then(|launcher_reg| registry::get_value(&launcher_reg, "clientExecutable"))
                .map(|launcher_filename| launcher_path.join(launcher_filename))
        })
        .map_err(|_error| {
            Error::new(
                ErrorKind::LauncherNotFound,
                "Invalid GOG path, maybe this launcher is not installed",
            )
        })
        .unwrap();

    if !launcher_executable.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid GOG path, maybe this launcher is not installed: {}",
                launcher_executable.display().to_string()
            ),
        ));
    }

    Ok(launcher_executable)
}
