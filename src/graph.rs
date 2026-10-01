use std::collections::{BTreeMap, BTreeSet};

use crate::Workflow;

/// Returns one lexical topological order or `None` for missing nodes/cycles.
pub(crate) fn topological_order(workflow: &Workflow) -> Option<Vec<String>> {
    let known: BTreeSet<_> = workflow.steps.iter().map(|step| step.id.as_str()).collect();
    if workflow.steps.iter().any(|step| {
        step.depends_on
            .iter()
            .any(|dependency| !known.contains(dependency.as_str()))
    }) {
        return None;
    }
    let mut indegree: BTreeMap<&str, usize> = workflow
        .steps
        .iter()
        .map(|step| (step.id.as_str(), step.depends_on.len()))
        .collect();
    let mut dependents: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for step in &workflow.steps {
        for dependency in &step.depends_on {
            dependents
                .entry(dependency.as_str())
                .or_default()
                .insert(step.id.as_str());
        }
    }
    let mut ready: BTreeSet<&str> = indegree
        .iter()
        .filter_map(|(id, count)| (*count == 0).then_some(*id))
        .collect();
    let mut order = Vec::with_capacity(workflow.steps.len());
    while let Some(next) = ready.pop_first() {
        order.push(next.to_owned());
        if let Some(children) = dependents.get(next) {
            for child in children {
                let count = indegree.get_mut(child).expect("known dependent");
                *count -= 1;
                if *count == 0 {
                    ready.insert(child);
                }
            }
        }
    }
    (order.len() == workflow.steps.len()).then_some(order)
}

#[cfg(test)]
mod tests {
    use super::topological_order;
    use crate::{ApprovalPolicy, IdempotencyPolicy, Workflow, WorkflowStep};

    fn step(id: &str, depends_on: &[&str]) -> WorkflowStep {
        WorkflowStep {
            id: id.to_owned(),
            action: id.to_owned(),
            depends_on: depends_on.iter().map(ToString::to_string).collect(),
            external_action: false,
            approval: ApprovalPolicy {
                required: false,
                gate_id: None,
            },
            idempotency: IdempotencyPolicy {
                required: false,
                key: None,
            },
        }
    }

    #[test]
    fn ties_are_lexical_and_cycles_have_no_order() {
        let workflow = Workflow {
            schema_version: "constillo.workflow/v1".to_owned(),
            workflow_id: "example".to_owned(),
            workflow_version: "1.0.0".to_owned(),
            steps: vec![step("b", &["a"]), step("c", &["a"]), step("a", &[])],
        };
        assert_eq!(topological_order(&workflow).expect("DAG"), ["a", "b", "c"]);
        let cyclic = Workflow {
            steps: vec![step("a", &["b"]), step("b", &["a"])],
            ..workflow
        };
        assert!(topological_order(&cyclic).is_none());
    }
}
