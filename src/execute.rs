use std::collections::BTreeSet;

use glob_match::glob_match;
use polywrap_wasm_rs::Map;
use serde_json::json;

use crate::logger::*;
use crate::wrap::imported::concurrent_module::{
    ArgsResult as ConcurrentArgsResult, ArgsSchedule as ConcurrentArgsSchedule,
};
use crate::wrap::*;
use crate::JobGraphBuilder;

fn get_ready_jobs(jobs: &Map<(String, String), u32>) -> Option<BTreeSet<&(String, String)>> {
    let mut ready_jobs: BTreeSet<&(String, String)> = BTreeSet::new();

    for (job, count) in jobs.iter() {
        if *count == 0 {
            ready_jobs.insert(job);
        }
    }

    if ready_jobs.len() > 0 {
        Some(ready_jobs)
    } else {
        None
    }
}

fn create_task(context_graph: &BuiltContextGraphs, dep: &String, cmd: &String) -> ConcurrentTask {
    let cmd_node = context_graph.command_graph.vertices.get(cmd).unwrap();
    let dep_node = context_graph.dependency_graph.vertices.get(dep).unwrap();

    ConcurrentTask {
        uri: cmd_node.uri.clone(),
        method: cmd_node.method.clone(),
        args: cmd_node.args.clone(),
        env: json!({
            "cwd": dep_node.path.clone(),
            "processId": dep_node.name.clone(),
        }),
    }
}

fn execute_tasks(tasks: Vec<ConcurrentTask>) -> () {
    match ConcurrentModule::schedule(&ConcurrentArgsSchedule { tasks }) {
        Ok(task_ids) => match ConcurrentModule::result(&ConcurrentArgsResult {
            task_ids,
            return_when: ConcurrentReturnWhen::ALL_COMPLETED,
        }) {
            Err(e) => panic!("{}", e),
            _ => (),
        },
        Err(e) => panic!("{}", e),
    };
}

pub fn execute_command_in_scope(
    context_graph: BuiltContextGraphs,
    scope: Vec<String>,
    commands: Vec<String>,
    log_level: LoggerLogLevel,
) -> () {
    let logger = Logger::new("monowrap".to_string(), log_level);
    let mut deps: BTreeSet<String> = BTreeSet::new();
    for dep in context_graph.dependency_graph.vertices.keys().clone() {
        for glob in scope.clone() {
            if glob_match(&glob, &dep) {
                deps.insert(dep.clone());
            }
        }
    }
    let dep_cmds_counter: Map<String, u32> = deps
        .iter()
        .map(|dep| (dep.clone(), commands.len().to_string().parse().unwrap()))
        .collect();
    logger.info("🛠️ Building job graph...".to_string());

    let mut builder = JobGraphBuilder::new(
        context_graph.clone(),
        dep_cmds_counter,
        commands,
        logger.clone(),
    );
    logger.debug(format!("JobGraphBuilder: {:?}", builder));
    let mut job_graph = builder.build();

    logger.info("✅ job graph built successfully!".to_string());
    logger.debug(format!("{:?}", job_graph));

    while let Some(ready_jobs) = get_ready_jobs(&job_graph.vertices.clone()) {
        let mut tasks: Vec<ConcurrentTask> = vec![];

        for (ready_dep, ready_cmd) in ready_jobs {
            let count = job_graph
                .vertices
                .get_mut(&(ready_dep.clone(), ready_cmd.clone()))
                .unwrap();
            *count = u32::MAX; // Marks job as visited

            for job in job_graph
                .adj_list
                .get_mut(&(ready_dep.clone(), ready_cmd.clone()))
                .unwrap()
            {
                let count = job_graph.vertices.get_mut(job).unwrap();
                *count -= 1;
            }

            tasks.push(create_task(&context_graph, &ready_dep, &ready_cmd));
            logger.info(format!(
                "🔄 Scheduling command: {} in {}",
                ready_dep, ready_cmd
            ));
        }
        logger.info("🚀 Executing jobs...".to_string());
        logger.debug(format!("Tasks: {:?}", tasks));
        execute_tasks(tasks);
        logger.info("✅ Jobs executed successfully!".to_string());
    }
}
