use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

use crate::error::FatalError;

fn do_call(
    command: impl IntoIterator<Item = impl Into<String>>,
    path: Option<&Path>,
    envs: Option<BTreeMap<&OsStr, &OsStr>>,
    dry_run: bool,
) -> Result<bool, FatalError> {
    let command: Vec<_> = command.into_iter().map(|s| s.into()).collect();
    if dry_run {
        if path.is_some() {
            log::trace!("cd {}", path.unwrap().display());
        }
        log::trace!("{}", command.join(" "));
        return Ok(true);
    }
    let mut iter = command.iter();
    let cmd_name = iter.next().unwrap();

    let mut cmd = Command::new(cmd_name);

    if let Some(p) = path {
        cmd.current_dir(p);
    }

    if let Some(e) = envs {
        cmd.envs(e.iter());
    }

    for arg in iter {
        if !arg.is_empty() {
            cmd.arg(arg);
        }
    }

    let mut child = cmd.spawn().map_err(FatalError::from)?;
    let result = child.wait().map_err(FatalError::from)?;

    Ok(result.success())
}

pub fn call(
    command: impl IntoIterator<Item = impl Into<String>>,
    dry_run: bool,
) -> Result<bool, FatalError> {
    do_call(command, None, None, dry_run)
}

pub fn call_on_path(
    command: impl IntoIterator<Item = impl Into<String>>,
    path: &Path,
    dry_run: bool,
) -> Result<bool, FatalError> {
    do_call(command, Some(path), None, dry_run)
}

pub fn call_with_env(
    command: impl IntoIterator<Item = impl Into<String>>,
    envs: BTreeMap<&OsStr, &OsStr>,
    path: &Path,
    dry_run: bool,
) -> Result<bool, FatalError> {
    do_call(command, Some(path), Some(envs), dry_run)
}
