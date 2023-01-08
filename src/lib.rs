pub mod graph;
pub mod manifest;
pub mod job;

pub use graph::*;
pub use manifest::*;
pub use job::*;
pub mod wrap;
use polywrap_wasm_rs::{BigInt, Map};
pub use wrap::imported::concurrent_module::serialization::ArgsSchedule;
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
    let mut builder = JobGraphBuilder::new(args.graph);
    let job_graph = builder.build();
    return true;
}
