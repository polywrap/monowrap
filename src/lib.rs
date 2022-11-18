pub mod manifest;
pub mod graph;

pub use manifest::*;
pub use graph::*;
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

pub fn build_context_graphs(args: ArgsBuildContextGraphs) -> BuiltContextGraphs {
    let command_graph = build_command_graph(&args.manifest);
    let dependency_graph = build_dependency_graph(&args.manifest);

    BuiltContextGraphs {
        id: "N/A".to_string(),
        manifest: args.manifest.to_owned(),
        dependency_graph: dependency_graph,
        command_graph: command_graph,
    }
}
