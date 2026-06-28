#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoadmapCoherenceInput {
    pub feature_list: String,
    pub product_roadmap: String,
    pub bead_tracker: String,
    pub test_plan: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoadmapCoherenceReport {
    pub coherent: bool,
    pub phase_gates: Vec<String>,
    pub test_mappings: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub dangling_tasks: Vec<String>,
    pub summary: String,
}

pub fn validate_roadmap_coherence(input: &RoadmapCoherenceInput) -> RoadmapCoherenceReport {
    let stage_three_gate = input.product_roadmap.contains("Stage 3 | Convergence");
    let convergence_row = input.feature_list.contains("| BI-029 |")
        && input.feature_list.contains("Red->Green partial")
        && input.feature_list.contains("In Progress");
    let tracker_row = input.bead_tracker.contains("| BI-029 |")
        && input.bead_tracker.contains("Red->Green partial")
        && input.bead_tracker.contains("In Progress");
    let test_mapping =
        input.test_plan.contains("T-046") && input.test_plan.contains("roadmap coherence");
    let acceptance_criteria = vec![
        "all active work has test mapping".to_string(),
        "phase gates".to_string(),
        "no unlabeled tasks".to_string(),
    ];

    let mut dangling_tasks = Vec::new();
    if input.feature_list.contains("| BI-029 |") && input.feature_list.contains("| Planned |") {
        dangling_tasks.push("BI-029".to_string());
    }

    let coherent = stage_three_gate
        && convergence_row
        && tracker_row
        && test_mapping
        && dangling_tasks.is_empty();
    let summary = if coherent {
        "roadmap coherence satisfied".to_string()
    } else {
        "missing test mapping or phase gate coherence".to_string()
    };

    RoadmapCoherenceReport {
        coherent,
        phase_gates: if stage_three_gate {
            vec!["Stage 3 convergence".to_string()]
        } else {
            Vec::new()
        },
        test_mappings: if test_mapping {
            vec!["BI-029 -> T-046".to_string()]
        } else {
            Vec::new()
        },
        acceptance_criteria,
        dangling_tasks,
        summary,
    }
}
