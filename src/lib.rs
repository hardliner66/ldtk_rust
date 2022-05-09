//! This library parses JSON generated by LDtk (Level Designers ToolKit) for use in Rust.
//! It is designed to be usable in any Rust program, including all game frameworks.
//!
//! Most users will want to start by reviewing the top level Project struct and
//! in particular the Project::new() method. Calling this method will load in
//! all of your LDtk data. See the library's /examples subdirectory for more
//! detailed examples.
//!
//! ```ignore
//! Project::new(f: Path) --- loads all the data
//! Project::load_project(f: Path) --- loads only the project file
//! Level::new(f: Path) --- loads a single external level file
//! ```

mod json_1_1_3;

pub use json_1_1_3::*;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

// this struct name has to match the auto-generated top-level struct.
// Currently mirroring the LDTK Haxe API as best I can figure out.
impl Project {
    pub fn new<P: AsRef<Path>>(f: P) -> Self {
        let mut o = Project::load_project(&f);
        if o.external_levels {
            o.load_external_levels(f);
        }
        o
    }

    // Read in an LDTK project file
    pub fn load_project<P: AsRef<Path>>(f: P) -> Self {
        let file = File::open(f).expect("project file not found");
        let o: Project = serde_json::from_reader(file).expect("error while reading");
        o
    }

    // Remove any items in the project.levels Vec ... useful when you
    // get external file info and want to replace the items with more
    // complete data extrated from the files.
    pub fn clear_levels(&mut self) {
        self.levels = Vec::new();
    }

    // Read in ALL the external level files referred to in an LDTK Project
    pub fn load_external_levels<P: AsRef<Path>>(&mut self, f: P) {
        // check to make sure there ARE separate levels
        // if not, then likely the call to this method
        // should do nothing because you already have
        // the levels.
        if self.external_levels {
            // get all the file names
            let mut all_level_files: Vec<PathBuf> = Vec::new();
            for level in self.levels.iter_mut() {
                let level_file_path = level.external_rel_path.as_ref().expect("missing level");
                all_level_files.push(level_file_path.into());
            }

            // get rid of existing levels (which don't have much data)
            self.clear_levels();

            // now add each of them to our struct
            for file in all_level_files.iter() {
                let mut full_path = PathBuf::new();
                let parent = f.as_ref().parent().unwrap().to_str().unwrap();
                full_path.push(parent);
                full_path.push("/");
                full_path.push(&file);
                println!("opening {:#?}", full_path);
                let level_ldtk = Level::new(full_path);
                self.levels.push(level_ldtk);
            }
        }
    }

    pub fn get_level(&self, uid: i64) -> Option<&Level> {
        for level in self.levels.iter() {
            if level.uid == uid {
                return Some(level);
            }
        }
        None
    }
}

impl Level {
    // Read in a single external LDTK level file
    pub fn new<P: AsRef<Path>>(f: P) -> Self {
        let file = File::open(f).expect("level file not found");
        let o: Level = serde_json::from_reader(file).expect("error while reading");
        o
    }
}

#[deprecated = "Use Project instead of LdtkJson to match LDtk documentation."]
pub struct LdtkJson;

// supports legacy name from 0.2.0 as well as
// some QuickType examples over at LDTK.
#[allow(deprecated)]
impl LdtkJson {
    pub fn new(f: String) -> Project {
        Project::new(f)
    }
}
