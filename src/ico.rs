use std::{borrow::Cow, env::temp_dir, path::PathBuf};

use pop_launcher::IconSource;
use sha2::{Digest, Sha256};
use tokio::io::AsyncWriteExt;

#[inline]
fn load_ico_b64(content: &str) -> Option<&str> {
    content.split("base64,").nth(1)
}

#[inline]
fn load_ico_svg(content: &str) -> Result<Vec<u8>, base64::DecodeError> {
    base64::decode(content)
}

fn genpath(content: &str) -> PathBuf {
    let mut sha = Sha256::new();
    sha.update(content);
    let hash = sha.finalize();
    let filename = format!("{hash:x}.png");
    temp_dir().join("firefox-tabs").join(filename)
}

#[derive(Default)]
pub struct Cache;

impl Cache {
    pub async fn load(&self, content: &str) -> Result<IconSource, &'static str> {
        let path = genpath(content);
        let as_name = || IconSource::Name(Cow::Owned(path.to_string_lossy().into_owned()));
        if path.exists() {
            log::trace!("Loading favico from cache: {path:?}");
            return Ok(as_name());
        }
        let b64 = load_ico_b64(content).ok_or("Unexpected icon format")?;
        let svg = load_ico_svg(b64).map_err(|_err| "Icon base64 decode error")?;
        if let Some(dirname) = path.parent() {
            tokio::fs::create_dir_all(dirname)
                .await
                .map_err(|_e| "Failed to create dirs")?;
        }
        let mut f = tokio::fs::File::create(&path)
            .await
            .map_err(|_e| "Failed to create file")?;
        f.write_all(&svg)
            .await
            .map_err(|_e| "Failed to write to new image source")?;
        log::trace!("Saved favico: {path:?}");
        Ok(as_name())
    }
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::Read, path::Path};

    use super::*;

    fn read_content(p: impl AsRef<Path>) -> String {
        let mut f = File::open(p).expect("Failed to open file");
        let mut c = String::new();
        f.read_to_string(&mut c).expect("Failed to read file");
        c
    }

    #[test]
    fn as_svg() {
        let txt = read_content("assets/ico.txt");
        let loaded_b64 = load_ico_b64(&txt).unwrap();

        let b64 = read_content("assets/ico.b64");
        assert!(loaded_b64 == b64);

        let loaded_svg = load_ico_svg(loaded_b64).unwrap();

        let svg = read_content("assets/ico.png");
        assert!(loaded_svg == svg.as_bytes())
    }

    #[test]
    fn path() {
        let expected = PathBuf::from("/tmp/firefox-tabs/0b7c33a8e1510a8a972bfb610f2ff623c08e37d973ffd559ef24732ffd55730c.png");
        let content = read_content("assets/ico.txt");
        let path = genpath(&content);
        assert_eq!(expected, path)
    }
}
