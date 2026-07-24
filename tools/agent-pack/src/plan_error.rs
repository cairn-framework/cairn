//! Actionable failures from agent-pack plan construction and rendering.

use crate::containment::ContainmentError;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) enum PlanError {
    SourceReadError {
        index: usize,
        path: PathBuf,
        details: String,
    },
    SourceContainment {
        index: usize,
        path: PathBuf,
        error: ContainmentError,
    },
    DestinationContainment {
        index: usize,
        destination: PathBuf,
        error: ContainmentError,
    },
    MissingCanonical {
        entry: String,
        mode: String,
    },
    DestinationIo {
        index: usize,
        destination: PathBuf,
        details: String,
    },
    DriftDetected {
        missing: Vec<PathBuf>,
        drifted: Vec<PathBuf>,
        manifest_path: PathBuf,
        repo_root: PathBuf,
    },
}

impl fmt::Display for PlanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let default_correction = "cargo run -p cairn-agent-pack -- --write";
        match self {
            Self::SourceReadError {
                index,
                path,
                details,
            } => write!(
                f,
                "row [[canonical]] index {index} source '{}': failed to read bytes: {details}; correct the source path",
                path.display()
            ),
            Self::SourceContainment { index, path, error } => write!(
                f,
                "row [[canonical]] index {index} source '{}': {error}; choose a source beneath the manifest directory and remove symlinked ancestors",
                path.display()
            ),
            Self::MissingCanonical { entry, mode } => write!(
                f,
                "entry '{entry}' mode '{mode}' has no canonical source; correct the manifest ownership rows"
            ),
            Self::DestinationContainment {
                index,
                destination,
                error,
            } => write!(
                f,
                "row [[adapters]] index {index} destination '{}': {error}; choose a destination beneath the repository root and remove symlinked ancestors",
                destination.display()
            ),
            Self::DestinationIo {
                index,
                destination,
                details,
            } => write!(
                f,
                "row [[adapters]] index {index} destination '{}': I/O failure: {details}. Run '{default_correction}'",
                destination.display()
            ),
            Self::DriftDetected {
                missing,
                drifted,
                manifest_path,
                repo_root,
            } => {
                writeln!(
                    f,
                    "render drift detected ({} missing, {} drifted). Run 'cargo run -p cairn-agent-pack -- --write --manifest <MANIFEST> --root <ROOT>' for manifest '{}' and root '{}'",
                    missing.len(),
                    drifted.len(),
                    manifest_path.display(),
                    repo_root.display()
                )?;
                for path in missing {
                    writeln!(f, "  missing: {}", path.display())?;
                }
                for path in drifted {
                    writeln!(f, "  drifted: {}", path.display())?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for PlanError {}
