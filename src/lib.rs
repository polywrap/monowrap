pub mod execute;
pub mod graph;
pub mod job;
pub mod manifest;
pub mod wrap;

pub use execute::*;
pub use graph::*;
pub use job::*;
pub use manifest::*;
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
    println!("Executing command: {}, {}", args.command, args.dependency);
    execute_graph(args.graph, args.dependency, args.command);
    return true;
}

pub fn main(args: ArgsMain) -> u8 {
    if args.args.len() < 3 {
        println!("Usage: pwr monowrap.eth <manifest> <dependency> <command>");
        return 1;
    }
    let (manifest, dependency, command) =
        (&args.args[0], &args.args[1], &args.args[2]);

    let manifest = get_manifest(ArgsGetManifest {
        path: manifest.to_string(),
    });
    println!("Manifest fetched successfully");
    let built_context_graphs = build_context_graphs(ArgsBuildContextGraphs {
        manifest: manifest.to_owned(),
    });
    println!("Context graphs built successfully");
    execute_command(ArgsExecuteCommand {
        graph: built_context_graphs.to_owned(),
        dependency: dependency.to_string(),
        command: command.to_string(),
    });
    println!("All Commands executed successfully");
    return 0;
}
