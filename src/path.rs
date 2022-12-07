use crate::root::Root;
use crate::url::Url;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Path<'a> {
    root: &'a Root,
    host: String,
    owner: String,
    repo: String,
}

impl<'a> Path<'a> {
    pub fn new(
        root: &'a Root,
        host: impl Into<String>,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> Self {
        Self {
            root,
            host: host.into(),
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    pub fn resolve(root: &'a Root, url: &Url) -> Self {
        Self {
            root,
            host: url.host.to_string(),
            owner: url.owner.clone(),
            repo: url.repo.clone(),
        }
    }
}

impl<'a> Display for Path<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}/{}", self.host, self.owner, self.repo)
    }
}

impl<'a> From<&Path<'a>> for PathBuf {
    fn from(p: &Path<'a>) -> Self {
        p.root.path().join(&p.host).join(&p.owner).join(&p.repo)
    }
}

impl<'a> From<Path<'a>> for PathBuf {
    fn from(p: Path<'a>) -> Self {
        (&p).into()
    }
}

pub struct PartialPath<'a> {
    pub root: &'a Root,
    pub host: Option<String>,
    pub owner: Option<String>,
    pub repo: Option<String>,
}

impl<'a> From<&PartialPath<'a>> for PathBuf {
    fn from(p: &PartialPath<'a>) -> Self {
        let mut path = p.root.path().to_owned();

        match p.host.as_deref() {
            Some(h) => path = path.join(h),
            _ => return path,
        }

        match p.owner.as_deref() {
            Some(o) => path = path.join(o),
            _ => return path,
        }

        match p.repo.as_deref() {
            Some(r) => path.join(r),
            _ => path,
        }
    }
}

impl<'a> From<PartialPath<'a>> for PathBuf {
    fn from(p: PartialPath<'a>) -> Self {
        (&p).into()
    }
}
