pub mod graph;
pub mod manifest;
pub mod job;
pub mod execute;
pub mod wrap;

pub use graph::*;
pub use manifest::*;
pub use job::*;
pub use execute::*;
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
        dependency_graph: dependency_graph.to_owned(),
        command_graph: command_graph.to_owned(),
        sub_deps_execute_after: args.manifest.sub_deps_execute_after.clone(),
    }
}

pub fn execute_command(args: ArgsExecuteCommand) -> bool {
    execute_graph(args.graph, args.dependency, args.command);
    return true;
}
