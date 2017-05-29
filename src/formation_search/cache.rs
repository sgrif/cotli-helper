use byteorder::*;
use std::env;
use std::fs::*;
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::path::*;
use ordermap::OrderMap;

use crusader::*;
use formation::Formation;
use super::Node;

pub struct Cache {
    directory: PathBuf,
}

impl Cache {
    pub fn new(key: u64) -> Self {
        let digest = format!("{:x}", key);
        Cache {
            directory: env::temp_dir().join("cotli_helper").join(&digest),
        }
    }

    pub(super) fn write_to_cache(&self, formation: &Formation, node: &Node) {
        let path = self.path_to_cache_file(formation);
        self.try_write_to_cache(&path, node).unwrap();
    }

    pub(super) fn load_from_cache<'a>(
        &self,
        formation: &Formation<'a>,
        crusaders: &'a [Crusader],
    ) -> Option<Node<'a>> {
        let file_path = self.path_to_cache_file(formation);
        if !file_path.join("search_data.dat").exists() {
            return None;
        }
        match self.try_load_from_cache(&file_path, crusaders) {
            Ok(x) => Some(x),
            Err(e) => {
                println!("Could not load from cache. Deleting directory. Error: {}", e);
                remove_dir_all(file_path).unwrap();
                None
            }
        }
    }

    fn path_to_cache_file(&self, formation: &Formation) -> PathBuf {
        formation.placements().fold(self.directory.clone(), |path, (idx, crusader)| {
            let as_dir = format!("{}{:?}", idx, crusader.name);
            path.join(&as_dir)
        })
    }

    fn try_write_to_cache(&self, path: &Path, node: &Node) -> io::Result<()> {
        create_dir_all(path)?;
        let mut file = BufWriter::new(File::create(path.join("search_data.dat"))?);
        file.write_u8(1)?;
        write_node(node, &mut file)?;
        Ok(())
    }

    fn try_load_from_cache<'a>(
        &self,
        path: &Path,
        crusaders: &'a [Crusader],
    ) -> io::Result<Node<'a>> {
        let mut file = BufReader::new(File::open(path.join("search_data.dat"))?);
        let v = file.read_u8()?;
        if v == 1 {
            read_node(crusaders, &mut file)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Invalid cache version"))
        }
    }
}

fn write_node<W: Write>(node: &Node, out: &mut W) -> io::Result<()> {
    out.write_f64::<NativeEndian>(node.highest_score_seen.into())?;
    out.write_f64::<NativeEndian>(node.total_score_seen.into())?;
    out.write_u32::<NativeEndian>(node.times_checked)?;
    out.write_u64::<NativeEndian>(node.children.len() as u64)?;
    for (&(idx, crusader), child) in &node.children {
        out.write_u8(idx as u8)?;
        out.write_u8(crusader.name as u8)?;
        write_node(child, out)?;
    }
    Ok(())
}

fn read_node<'a, R: Read>(crusaders: &'a [Crusader], read: &mut R) -> io::Result<Node<'a>> {
    use std::mem::transmute;
    let highest_score_seen = read.read_f64::<NativeEndian>()?.into();
    let total_score_seen = read.read_f64::<NativeEndian>()?.into();
    let times_checked = read.read_u32::<NativeEndian>()?;
    let num_children = read.read_u64::<NativeEndian>()? as usize;
    let mut children = OrderMap::with_capacity(num_children);
    for _ in 0..num_children {
        let idx = read.read_u8()? as usize;
        let name = unsafe { transmute(read.read_u8()?) };
        let crusader = crusaders.iter().find(|c| c.name == name).unwrap();
        let node = read_node(crusaders, read)?;
        children.insert((idx, crusader), node);
    }
    Ok(Node {
        progress: Default::default(),
        highest_score_seen,
        total_score_seen,
        times_checked,
        children,
    })
}
