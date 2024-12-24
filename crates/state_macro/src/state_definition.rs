use std::collections::{HashMap, HashSet};

use helpers::get_item_attribute;
use proc_macro2::Span;
use quote::quote;
use syn::Ident;

use crate::{items::item_state::ItemState, resource::Resource, state_attribute::StateAttribute};

pub(crate) struct StateDefintion {
    resources: Vec<Resource>,
}

impl StateDefintion {
    pub(crate) fn expand(self) -> proc_macro2::TokenStream {
        let resources_impls = self.resources.into_iter().map(|r| r.expand());

        quote! {
            #(#resources_impls)*
        }
    }

    fn get_resources(item_state: ItemState) -> Result<Vec<Resource>, syn::Error> {
        let resources = item_state
            .item_resources()
            .filter_map(|item_resource| {
                match get_item_attribute::<StateAttribute>(&item_resource) {
                    Ok(Some(state_attribute)) => match state_attribute {
                        StateAttribute::Resource(resource_field) => {
                            Some(Ok(Resource::from(item_resource, resource_field)))
                        }
                    },
                    Ok(None) => None,
                    Err(err) => Some(Err(err)),
                }
            })
            .collect::<Result<Vec<Resource>, syn::Error>>()?;

        Ok(resources)
    }

    fn resolve_dependencies(resources: Vec<Resource>) -> Result<Vec<Resource>, syn::Error> {
        let independents = resources
            .iter()
            .filter(|r| r.get_dependencies().is_empty())
            .map(|r| r.name())
            .collect::<Vec<String>>();

        let dependents = resources
            .iter()
            .filter(|r| !r.get_dependencies().is_empty())
            .map(|r| (r.name(), r.get_dependencies()))
            .collect::<Vec<(String, Vec<Ident>)>>();

        let mut graph: HashMap<String, Vec<Ident>> = HashMap::new();

        for (item, deps) in dependents {
            graph.insert(item, deps);
        }

        let mut results: Vec<Resource> = Vec::new();
        let mut visited = HashSet::new();
        let mut visiting = HashSet::new();

        fn dfs(
            dep: String,
            graph: &HashMap<String, Vec<Ident>>,
            visited: &mut HashSet<String>,
            visiting: &mut HashSet<String>,
            results: &mut Vec<Resource>,
            resources: &Vec<Resource>,
        ) -> Result<(), syn::Error> {
            if visited.contains(&dep) {
                return Ok(());
            }
            if visiting.contains(&dep) {
                return Err(syn::Error::new(
                    Span::call_site(),
                    format!("Cycle detected at '{}'", dep),
                ));
            }

            visiting.insert(dep.to_string());
            if let Some(dependencies) = graph.get(&dep) {
                for dep in dependencies {
                    dfs(
                        dep.to_string(),
                        graph,
                        visited,
                        visiting,
                        results,
                        resources,
                    )?;
                }
            }
            visiting.remove(&dep);
            visited.insert(dep.clone());

            let result = resources
                .iter()
                .find(|r| r.name().as_str() == &dep)
                .ok_or_else(|| {
                    syn::Error::new(
                        Span::call_site(),
                        format!("Resource {} not found in resource list", dep.clone()),
                    )
                })?;

            results.push(result.clone());

            Ok(())
        }

        for dep in independents.iter().chain(graph.keys()) {
            dfs(
                dep.to_string(),
                &graph,
                &mut visited,
                &mut visiting,
                &mut results,
                &resources,
            )?;
        }

        Ok(results)
    }
}

impl TryFrom<ItemState> for StateDefintion {
    type Error = syn::Error;

    fn try_from(value: ItemState) -> Result<Self, Self::Error> {
        let resources = Self::get_resources(value)?;
        let resources = Self::resolve_dependencies(resources)?;

        Ok(Self { resources })
    }
}
