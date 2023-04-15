use serde::Deserialize;
#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Key {
    key: String,
    permissions: Vec<Permission>,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub enum Permission {
    VolumeControl((f32, f32)),
    Seek,
    Add,
    Download,
    PlayPause,
    Info,
    All,
}

impl Key {
    pub fn convert_all(&self) -> Self {
        let key = self.key.to_owned();
        let permissions = match self.permissions.contains(&Permission::All) {
            true => {
                vec![
                    Permission::VolumeControl((0.0, 10.0)),
                    Permission::Seek,
                    Permission::All,
                    Permission::Download,
                    Permission::PlayPause,
                    Permission::Info,
                ]
            }
            false => self.permissions.to_owned(),
        };
        Key { key, permissions }
    }
    pub fn is_allowed(&self, permissions: &[Permission]) -> bool {
        permissions
            .iter()
            .fold(true, |acc, x| self.permissions.contains(x) && acc)
    }
}
