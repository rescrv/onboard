//! onboard is a linter for objective graphs.

use onboard::ObjectiveGraph;

fn main() {
    let mut objective_graph = ObjectiveGraph::default();
    for arg in std::env::args().skip(1) {
        let og = ObjectiveGraph::load(&arg).unwrap();
        objective_graph.merge(og);
    }
    let lint = objective_graph.report().unwrap();

    // first rule
    if lint
        .iter()
        .any(|lint| matches!(lint, (onboard::Lint::GraphNotADag(_), _)))
    {
        println!("[X] graph is not a DAG");
    } else {
        println!("[✓] graph is a DAG");
    }

    // second rule
    if lint
        .iter()
        .any(|lint| matches!(lint, (onboard::Lint::InvalidOwnership(_), _)))
    {
        println!("[X] ownership is invalid");
    } else {
        println!("[✓] ownership is valid");
    }

    // third rule
    if lint
        .iter()
        .any(|lint| matches!(lint, (onboard::Lint::TooMuchWork(_, _), _)))
    {
        println!("[X] work scheduling is invalid");
    } else {
        println!("[✓] work scheduling is valid");
    }

    // fourth rule
    if lint
        .iter()
        .any(|lint| matches!(lint, (onboard::Lint::HigherPriority(_, _), _)))
    {
        println!("[X] priority inversion");
    } else {
        println!("[✓] priority monotonicity");
    }

    if !lint.is_empty() {
        // details
        println!("\nlint detected:");
        for lint in lint.into_iter() {
            match lint.0 {
                onboard::Lint::GraphNotADag(p) => {
                    println!("- graph is not a DAG: {p}");
                }
                onboard::Lint::InvalidOwnership(p) => {
                    println!("- invalid ownership: {p}");
                }
                onboard::Lint::TooMuchWork(work, owner) => {
                    println!("- too much work for {owner}: {work}");
                }
                onboard::Lint::HigherPriority(p, c) => {
                    println!("- priority inversion: {p} is higher priority than {c}");
                }
            }
        }
        std::process::exit(1);
    }
}
