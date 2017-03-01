use dev_prefix::*;
use core::types::*;

lazy_static!{
    pub static ref ART_LOC: Regex = Regex::new(
        &format!(r"(?i)(?:#({}))|(\n)", ART_VALID_STR)).unwrap();
}

pub fn find_locs_text(path: &Path, text: &str, locs: &mut HashMap<ArtName, Loc>) -> Result<()> {
    let mut line = 1;
    for cap in ART_LOC.captures_iter(text) {
        //debug_assert_eq!(cap.len(), 2);
        if let Some(m) = cap.get(1) {
            debug_assert!(cap.get(2).is_none());
            let name = ArtName::from_str(m.as_str()).expect("regex validated");
            let loc = Loc {
                path: path.to_path_buf(),
                line: line,
            };
            if let Some(first) = locs.insert(name, loc) {
                warn!("locations found twice. first: {}({}), \
                      second: {}({})",
                      first.path.display(),
                      first.line,
                      path.display(),
                      line);
            }
        } else {
            debug_assert!(cap.get(2).is_some());
            line += 1;
        }
    }
    Ok(())
}

/// given text, the path to the text, and the locations to add onto
/// extract all the locations from the text and return whether there
/// was an error
pub fn find_locs_file(path: &Path, locs: &mut HashMap<ArtName, Loc>) -> Result<()> {
    debug!("resolving locs at: {:?}", path);
    let mut text = String::new();
    let mut f = fs::File::open(path).chain_err(|| format!("opening file: {}", path.display()))?;
    if let Err(e) = f.read_to_string(&mut text) {
        if e.kind() == io::ErrorKind::InvalidData {
            warn!("non-utf8 file: {}", path.display());
            return Ok(());
        } else {
            Err(e).chain_err(|| format!("reading file: {}", path.display()))?;
        }
    }
    find_locs_text(path, &text, locs)
}

/// recursively find all locs given a directory
fn find_locs_dir(path: &PathBuf,
                 loaded_dirs: &mut HashSet<PathBuf>,
                 locs: &mut HashMap<ArtName, Loc>)
                 -> Result<()> {
    loaded_dirs.insert(path.to_path_buf());
    let read_dir = fs::read_dir(path).chain_err(|| format!("loading dir {}", path.display()))?;
    let mut dirs_to_load: Vec<PathBuf> = Vec::new(); // TODO: use references
    for entry in read_dir.filter_map(|e| e.ok()) {
        let fpath = entry.path();
        // don't parse .toml files for locations
        // TODO: make general instead
        match fpath.extension() {
            None => {}
            Some(ext) => {
                if ext == "toml" {
                    continue;
                }
            }
        }
        let ftype = entry.file_type().chain_err(|| format!("{}", fpath.display()))?;
        if ftype.is_dir() {
            dirs_to_load.push(fpath.clone());
        } else if ftype.is_file() {
            find_locs_file(&fpath, locs)?
        }
    }

    for d in dirs_to_load {
        if !loaded_dirs.contains(&d) {
            find_locs_dir(&d, loaded_dirs, locs)?;
        }
    }
    Ok(())
}

/// search through the `code_paths` in settings to find all valid locs
pub fn find_locs(settings: &Settings) -> Result<HashMap<ArtName, Loc>> {
    info!("parsing code files for artifacts...");
    let mut locs: HashMap<ArtName, Loc> = HashMap::new();
    let mut loaded_dirs: HashSet<PathBuf> =
        HashSet::from_iter(settings.exclude_code_paths.iter().map(|p| p.to_path_buf()));
    debug!("excluded code paths: {:?}", loaded_dirs);
    for dir in &settings.code_paths {
        if loaded_dirs.contains(dir) {
            continue;
        }
        debug!("Loading from code: {:?}", dir);
        find_locs_dir(dir, &mut loaded_dirs, &mut locs)?;
    }
    Ok(locs)
}

/// attach the locations to the artifacts, returning locations that were not used.
pub fn attach_locs(artifacts: &mut Artifacts,
                   mut locs: HashMap<ArtName, Loc>)
                   -> Result<HashMap<ArtName, Loc>> {
    let mut dne: HashMap<ArtName, Loc> = HashMap::new();
    for (lname, loc) in locs.drain() {
        let artifact = match artifacts.get_mut(&lname) {
            Some(a) => a,
            None => {
                dne.insert(lname, loc);
                continue;
            }
        };
        if let Done::Defined(_) = artifact.done {
            return Err(ErrorKind::DoneTwice(lname.to_string()).into());
        }
        artifact.done = Done::Code(loc);
    }
    Ok(dne)
}
