use crate::Error;
use gates_vocabulary_language_core_dependencies::decoder::Decoder;
use gates_vocabulary_language_core_dependencies::manifest::{Manifest, ManifestText};
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub(in crate::gate) struct ManifestFile<'directory> {
    file: Box<dyn Read + Send>,
    manifest_path: PathBuf,
    directory: &'directory Path,
}

impl Layer for ManifestFile<'_> {
    type Layer = ApplicationLayer;
}

impl ManifestFile<'_> {
    pub(in crate::gate) fn try_decode_manifest(
        self,
        decoder: &impl Decoder,
    ) -> Result<Manifest, Error> {
        let Self {
            file,
            manifest_path,
            directory,
        } = self;
        let file_error = |error: io::Error| {
            let path = manifest_path.clone();
            Error::File { path, error }
        };
        let manifest_error = |error: io::Error| {
            let path = manifest_path.clone();
            Error::Manifest { path, error }
        };
        let text = io::read_to_string(file).map_err(file_error)?;
        let file_text = text.as_str();
        let manifest_text = ManifestText::from(file_text);
        let decoder_manifest = decoder
            .try_decode_manifest(&manifest_text)
            .map_err(manifest_error)?;
        let manifest = Manifest::from((&decoder_manifest, directory));
        Ok(manifest)
    }
}

impl<'directory> From<(Box<dyn Read + Send>, PathBuf, &'directory Path)>
    for ManifestFile<'directory>
{
    fn from(
        (file, manifest_path, directory): (Box<dyn Read + Send>, PathBuf, &'directory Path),
    ) -> Self {
        Self {
            file,
            manifest_path,
            directory,
        }
    }
}
