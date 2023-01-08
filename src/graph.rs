use polywrap_wasm_rs::{BigInt, Map};
use std::collections::BTreeSet;

use crate::wrap::*;

pub fn build_command_graph(manifest: &MonowrapManifest) -> CommandGraph {
    let mut map = Map::<String, BTreeSet<&String>>::new();
    let mut command_map = Map::<String, CommandNode>::new();

    for command in manifest.commands.iter() {
        let dependent = &command.alias;
        command_map.insert(
            command.alias.clone(),
            CommandNode {
                alias: command.alias.clone(),
                uri: command.uri.clone(),
                method: command.method.clone(),
                args: command.args.clone(),
                deps: BigInt::from(command.requires.clone().unwrap_or(vec![]).len()),
                visited: false,
            },
        );
        match &command.requires {
            Some(requires) => {
                for require in requires.iter() {
                    if let Some(dependents) = map.get_mut(require) {
                        dependents.insert(&dependent);
                    } else {
                        let mut st: BTreeSet<&String> = BTreeSet::new();
                        st.insert(&dependent);
                        map.insert(require.to_owned(), st);
                    }
                }
            }
            None => (),
        }
    }

    let mut adj_list = Map::<String, Vec<String>>::new();

    for (k, v) in map.iter_mut() {
        let arr: Vec<String> = v.to_owned().into_iter().map(|x| x.to_owned()).collect();
        adj_list.insert(k.to_owned(), arr);
    }

    CommandGraph {
        vertices: command_map.to_owned(),
        adj_list: adj_list.to_owned(),
    }
}

pub fn build_dependency_graph(manifest: &MonowrapManifest) -> DependencyGraph {
    let mut map = Map::<String, BTreeSet<&String>>::new();
    let mut dependency_nodes_map = Map::<String, DependencyNode>::new();

    for dependency in manifest.dependencies.iter() {
        let dependent = &dependency.name;
        dependency_nodes_map.insert(
            dependency.name.clone(),
            DependencyNode {
                name: dependency.name.clone(),
                path: dependency.path.clone(),
                deps: BigInt::from(dependency.requires.clone().unwrap_or(vec![]).len()),
                visited: false,
            },
        );
        match &dependency.requires {
            Some(requires) => {
                for require in requires.iter() {
                    if let Some(dependents) = map.get_mut(require) {
                        dependents.insert(&dependent);
                    } else {
                        let mut st: BTreeSet<&String> = BTreeSet::new();
                        st.insert(&dependent);
                        map.insert(require.to_owned(), st);
                    }
                }
            }
            None => (),
        }
    }

    let mut adj_list = Map::<String, Vec<String>>::new();

    for (k, v) in map.iter_mut() {
        let arr: Vec<String> = v.to_owned().into_iter().map(|x| x.to_owned()).collect();
        adj_list.insert(k.to_owned(), arr);
    }

    DependencyGraph {
        vertices: dependency_nodes_map.to_owned(),
        adj_list: adj_list.to_owned(),
    }
}
