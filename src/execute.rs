use std::collections::BTreeSet;

use polywrap_wasm_rs::{Map, wrap_debug_log};
use serde_json::json;

use crate::wrap::imported::concurrent_module::{
    ArgsResult as ConcurrentArgsResult, 
    ArgsSchedule as ConcurrentArgsSchedule,
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
    wrap_debug_log("Building job graph...");
    let mut builder = JobGraphBuilder::new(context_graph.clone());
    let mut job_graph = builder.build();
    wrap_debug_log(format!("Job graph built! ({} vertices)", job_graph.vertices.len()).as_str());
    wrap_debug_log(format!("{:?}", job_graph.clone()).as_str());

    while let Some(ready_jobs) = get_ready_jobs(&job_graph.vertices.clone()) {
        let mut tasks: Vec<ConcurrentTask> = vec![];
        if ready_jobs.contains(&(dep.clone(), cmd.clone())) {
            tasks.push(create_task(&context_graph, &dep, &cmd));
            wrap_debug_log(format!("Scheduling and Executing final task: {} {}", dep, cmd).as_str());
            execute_tasks(tasks);
            break;
        }
        for (ready_dep, ready_cmd) in ready_jobs {
            let count = job_graph.vertices.get_mut(&(ready_dep.clone(), ready_cmd.clone())).unwrap();
            *count = u32::MAX; // Marks job as visited

            for job in job_graph.adj_list.get_mut(&(ready_dep.clone(), ready_cmd.clone())).unwrap() {
                let count = job_graph.vertices.get_mut(job).unwrap();
                *count -= 1;
            }

            tasks.push(create_task(&context_graph, &ready_dep, &ready_cmd));
            wrap_debug_log(format!("Scheduling task: {} {}", ready_dep, ready_cmd).as_str());
        }
        wrap_debug_log("Executing tasks...");
        execute_tasks(tasks);
    }
}
