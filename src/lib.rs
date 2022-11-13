pub mod manifest;
pub use manifest::*;
pub mod wrap;
pub use wrap::*;

use wrap::imported::ArgsReadFileAsString;

pub fn get_manifest(args: ArgsGetManifest) -> MonowrapManifest {
    let manifest = FsModule::read_file_as_string(&ArgsReadFileAsString {
        path: args.path,
        encoding: Some(FsFileSystemEncoding::UTF8),
    })
    .unwrap();

    deserialize_manifest(manifest)
}
