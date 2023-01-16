use polywrap_wasm_rs::{wrap_debug_log, BigInt, Map};
use serde::{Deserialize, Serialize};

use crate::wrap::*;

fn get_ready_deps(deps: &Map<String, DependencyNode>) -> Option<Vec<String>> {
    let ready_deps: Vec<String> = deps
        .values()
        .filter(|dep| dep.deps == BigInt::from(0) && dep.visited == false)
        .map(|dep| dep.name.clone())
        .collect();

    if ready_deps.len() > 0 {
        Some(ready_deps)
    } else {
        None
    }
}

fn get_ready_commands(cmds: &Map<String, CommandNode>) -> Option<Vec<String>> {
    let ready_cmds: Vec<String> = cmds
        .values()
        .filter(|cmd| cmd.deps == BigInt::from(0) && cmd.visited == false)
        .map(|cmd| cmd.alias.clone())
        .collect();

    if ready_cmds.len() > 0 {
        Some(ready_cmds)
    } else {
        None
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JobGraph {
    pub vertices: Map<(String, String), u32>,
    pub adj_list: Map<(String, String), Vec<(String, String)>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JobGraphBuilder {
    job_graph: JobGraph,
    context_graph: BuiltContextGraphs,
}

impl JobGraphBuilder {
    pub fn new(context_graph: BuiltContextGraphs) -> JobGraphBuilder {
        JobGraphBuilder {
            job_graph: JobGraph {
                vertices: Map::<(String, String), u32>::new(),
                adj_list: Map::<(String, String), Vec<(String, String)>>::new(),
            },
            context_graph,
        }
    }

    pub fn build(&mut self) -> JobGraph {
        wrap_debug_log("Building job graph");
        while let Some(mut ready_deps) =
            get_ready_deps(&self.context_graph.dependency_graph.vertices)
        {
            for dep in ready_deps.iter_mut() {
                wrap_debug_log(format!("dependency: {}", dep).as_str());
                self.context_graph
                    .dependency_graph
                    .vertices
                    .get_mut(dep)
                    .unwrap()
                    .visited = true;
                self.add_dependency(&dep);
            }
        }

        self.job_graph.to_owned()
    }

    pub fn add_dependency(&mut self, dep: &String) -> () {
        let mut command_graph = self.context_graph.command_graph.clone();
        let dependency_graph = &mut self.context_graph.dependency_graph;
        let sub_deps_execute_after = self.context_graph.sub_deps_execute_after.clone();

        while let Some(mut ready_cmds) = get_ready_commands(&command_graph.vertices) {
            for cmd in ready_cmds.iter_mut() {
                wrap_debug_log(format!("command: {}", cmd).as_str());
                command_graph
                    .vertices
                    .get_mut(cmd)
                    .unwrap()
                    .visited = true;
                if !self
                    .job_graph
                    .vertices
                    .contains_key(&(dep.clone(), cmd.clone()))
                {
                    self.job_graph
                        .vertices
                        .insert((dep.clone(), cmd.clone()), 0);
                }

                if !self
                    .job_graph
                    .adj_list
                    .contains_key(&(dep.clone(), cmd.clone()))
                {
                    self.job_graph
                        .adj_list
                        .insert((dep.clone(), cmd.clone()), vec![]);
                }

                let job_adj_list = self
                    .job_graph
                    .adj_list
                    .get_mut(&(dep.clone(), cmd.clone()))
                    .unwrap();

                match command_graph.adj_list.get(&cmd.clone()) {
                    Some(sub_cmds) => {
                        for sub_cmd_alias in sub_cmds.clone() {
                            let deps = *self
                                .job_graph
                                .vertices
                                .get(&(dep.clone(), sub_cmd_alias.clone()))
                                .unwrap_or(&0);
                            self.job_graph
                                .vertices
                                .insert((dep.clone(), sub_cmd_alias.clone()), deps + 1);

                            job_adj_list.push((dep.clone(), sub_cmd_alias.clone()));
                            command_graph.vertices.get_mut(&sub_cmd_alias).unwrap().deps -= 1;
                        }
                    }
                    None => (),
                }

                if cmd.clone() == sub_deps_execute_after {
                    match dependency_graph.adj_list.get(&dep.clone()) {
                        Some(sub_deps) => {
                            for sub_dep in sub_deps {
                                match get_ready_commands(&self.context_graph.command_graph.vertices)
                                {
                                    Some(ready_cmds) => {
                                        for ready_cmd in ready_cmds {
                                            let deps = *self
                                                .job_graph
                                                .vertices
                                                .get(&(sub_dep.clone(), ready_cmd.clone()))
                                                .unwrap_or(&0);
                                            self.job_graph.vertices.insert(
                                                (sub_dep.clone(), ready_cmd.clone()),
                                                deps + 1,
                                            );

                                            job_adj_list
                                                .push((sub_dep.clone(), ready_cmd.clone()));
                                            dependency_graph
                                                .vertices
                                                .get_mut(&sub_dep.clone())
                                                .unwrap()
                                                .deps -= 1;
                                        }
                                    }
                                    None => break,
                                }
                            }
                        }
                        None => (),
                    }
                }
            }
        }
    }
}
