use std::collections::BTreeSet;

use polywrap_wasm_rs::Map;
use serde_json::json;

use crate::wrap::imported::concurrent_module::{
    ArgsResult as ConcurrentArgsResult, 
    ArgsSchedule as ConcurrentArgsSchedule,
};
use crate::wrap::*;
use crate::JobGraphBuilder;

fn get_ready_jobs(jobs: &mut Map<(String, String), u32>) -> Option<BTreeSet<&(String, String)>> {
    let mut ready_jobs: BTreeSet<&(String, String)> = BTreeSet::new();

    for (job, count) in jobs.iter_mut() {
        if *count == 0 {
            *count = u32::MAX; // Marks job as visited
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
        })
    }
}

fn execute_tasks(tasks: Vec<ConcurrentTask>) -> () {
    match ConcurrentModule::schedule(&ConcurrentArgsSchedule{
        tasks
    }) {
        Ok(task_ids) => match ConcurrentModule::result(&ConcurrentArgsResult {
            task_ids,
            return_when: ConcurrentReturnWhen::ALL_COMPLETED
        }) {
            Err(e) => panic!("{}", e),
            _ => ()
        },
        Err(e) => panic!("{}", e)
    };
}

pub fn execute_graph(context_graph: BuiltContextGraphs, dep: String, cmd: String) -> () {
    let mut builder = JobGraphBuilder::new(context_graph.clone());
    let mut job_graph = builder.build();

    while let Some(ready_jobs) = get_ready_jobs(&mut job_graph.vertices) {
        let mut tasks: Vec<ConcurrentTask> = vec![];
        if ready_jobs.contains(&(dep.clone(), cmd.clone())) {
            tasks.push(create_task(&context_graph, &dep, &cmd));
            execute_tasks(tasks);
            break;
        }
        for (ready_dep, ready_cmd) in ready_jobs {
            tasks.push(create_task(&context_graph, &ready_dep, &ready_cmd));
        }
        execute_tasks(tasks);
    }
}
