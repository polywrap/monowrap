pub mod execute;
pub mod graph;
pub mod job;
pub mod manifest;
pub mod wrap;
pub mod arg_parse;
pub mod logger;

pub use execute::*;
pub use graph::*;
pub use job::*;
pub use manifest::*;
pub use wrap::*;
pub use arg_parse::*;
pub use logger::*;

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
    execute_command_in_scope(args.graph, args.scope, args.commands, args.log_level);
    return true;
}

pub fn main(args: ArgsMain) -> u8 {
    let parsed_args = match arg_parse(args.args) {
        ArgParseResult::Args(args) => args,
        ArgParseResult::Help(help) => {
            print(help);
            return 0;
        },
        ArgParseResult::Error(err) => {
            print_error(err);
            return 1;
        }
    };

    let logger = Logger::new("monowrap".to_string(), parsed_args.log_level);

    logger.debug(format!("Args: {:?}", parsed_args));

    let manifest = get_manifest(ArgsGetManifest {
        path: parsed_args.manifest.clone(),
    });

    logger.info(format!("ℹ️ Using manifest: {}", parsed_args.manifest));

    let built_context_graphs = build_context_graphs(ArgsBuildContextGraphs {
        manifest: manifest.clone(),
    });
    logger.info("✅ Context graphs built successfully!".to_string());

    execute_command(ArgsExecuteCommand {
        graph: built_context_graphs.clone(),
        scope: parsed_args.scope.clone(),
        commands: parsed_args.commands.clone(),
        log_level: parsed_args.log_level.clone(),
    });
    logger.info("✅ All Commands executed successfully!".to_string());
    return 0;
}
