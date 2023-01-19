use std::{
    env::temp_dir,
    fs::{create_dir_all, remove_dir_all},
    path::{Path, PathBuf},
};

use hemtt_bin_config::project::Configuration;
use hemtt_bin_error::Error;
use vfs::{AltrootFS, MemoryFS, OverlayFS, PhysicalFS, VfsPath};

use crate::addons::Addon;

pub struct Context {
    config: Configuration,
    addons: Vec<Addon>,
    vfs: VfsPath,
    project_folder: PathBuf,
    hemtt_folder: PathBuf,
    out_folder: PathBuf,
    tmp: PathBuf,
}

impl Context {
    pub fn new(folder: &str) -> Result<Self, Error> {
        let tmp = temp_dir().join("hemtt").join(
            std::env::current_dir()
                .unwrap()
                .components()
                .skip(2)
                .collect::<PathBuf>()
                .to_str()
                .unwrap()
                .replace(['\\', '/'], "_"),
        );
        let project_folder = std::env::current_dir().expect("Unable to get current directory");
        let hemtt_folder = project_folder.join(".hemtt");
        let out_folder = project_folder.join(".hemttout");
        create_dir_all(&out_folder)?;
        let build_folder = out_folder.join(folder);
        if build_folder.exists() {
            remove_dir_all(&build_folder)?;
        }
        create_dir_all(&build_folder)?;
        Ok(Self {
            config: {
                let path = Path::new(".hemtt").join("project.toml");
                if !path.exists() {
                    return Err(Error::ConfigNotFound);
                }
                Configuration::from_file(&path)?
            },
            vfs: OverlayFS::new(&{
                let mut layers = vec![AltrootFS::new(MemoryFS::new().into()).into()];
                if cfg!(target_os = "windows") {
                    layers.push(AltrootFS::new(PhysicalFS::new(tmp.join("output")).into()).into());
                }
                layers.push(AltrootFS::new(PhysicalFS::new(".").into()).into());
                layers
            })
            .into(),
            project_folder,
            hemtt_folder,
            out_folder: build_folder,
            addons: Addon::scan()?,
            tmp,
        })
    }

    pub fn filter<F>(self, mut filter: F) -> Self
    where
        F: FnMut(&Addon, &Configuration) -> bool,
    {
        let config = self.config.clone();
        Self {
            addons: self
                .addons
                .into_iter()
                .filter(|a| filter(a, &config))
                .collect(),
            ..self
        }
    }

    pub const fn config(&self) -> &Configuration {
        &self.config
    }

    pub fn addons(&self) -> &[Addon] {
        &self.addons
    }

    pub const fn vfs(&self) -> &VfsPath {
        &self.vfs
    }

    /// The project folder
    pub const fn project_folder(&self) -> &PathBuf {
        &self.project_folder
    }

    /// The .hemtt folder
    pub const fn hemtt_folder(&self) -> &PathBuf {
        &self.hemtt_folder
    }

    /// The .hemttout folder
    pub const fn out_folder(&self) -> &PathBuf {
        &self.out_folder
    }

    /// %temp%/hemtt/project
    pub const fn tmp(&self) -> &PathBuf {
        &self.tmp
    }
}
