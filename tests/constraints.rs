use std::collections::HashMap;

use rustsat::{
    instances::SatInstance,
    lit, solvers,
    solvers::SolverResult,
    types::{
        constraints::{CardConstraint, PBConstraint},
        Lit,
    },
};

macro_rules! test_card {
    ( $constr:expr, $sat_assump:expr, $unsat_assump:expr ) => {{
        let mut inst: SatInstance = SatInstance::new();
        inst.add_card_constr($constr);
        let (cnf, _) = inst.as_cnf();
        println!("{:?}", cnf);
        let mut solver = solvers::new_default_inc_solver();
        solver.add_cnf(cnf);
        assert_eq!(
            solver.solve_assumps($sat_assump).unwrap(),
            SolverResult::SAT
        );
        assert_eq!(
            solver.solve_assumps($unsat_assump).unwrap(),
            SolverResult::UNSAT
        );
    }};
}

macro_rules! test_pb {
    ( $constr:expr, $sat_assump:expr, $unsat_assump:expr ) => {{
        let mut inst: SatInstance = SatInstance::new();
        inst.add_pb_constr($constr);
        let (cnf, _) = inst.as_cnf();
        println!("{:?}", cnf);
        let mut solver = solvers::new_default_inc_solver();
        solver.add_cnf(cnf);
        assert_eq!(
            solver.solve_assumps($sat_assump).unwrap(),
            SolverResult::SAT
        );
        assert_eq!(
            solver.solve_assumps($unsat_assump).unwrap(),
            SolverResult::UNSAT
        );
    }};
}

#[test]
fn card_ub() {
    let lits = vec![lit![0], lit![1], lit![2]];
    test_card!(
        CardConstraint::new_ub(lits.clone(), 1),
        vec![!lit![0], lit![1], !lit![2]],
        vec![lit![0], lit![1], !lit![2]]
    );
    test_card!(
        CardConstraint::new_ub(lits.clone(), 2),
        vec![lit![0], lit![1], !lit![2]],
        vec![lit![0], lit![1], lit![2]]
    );
}

#[test]
fn card_lb() {
    let lits = vec![lit![0], lit![1], lit![2]];
    test_card!(
        CardConstraint::new_lb(lits.clone(), 1),
        vec![lit![0], lit![1], !lit![2]],
        vec![!lit![0], !lit![1], !lit![2]]
    );
    test_card!(
        CardConstraint::new_lb(lits.clone(), 2),
        vec![lit![0], lit![1], !lit![2]],
        vec![!lit![0], !lit![1], !lit![2]]
    );
    test_card!(
        CardConstraint::new_lb(lits.clone(), 3),
        vec![lit![0], lit![1], lit![2]],
        vec![!lit![0], lit![1], !lit![2]]
    );
}

#[test]
fn card_eq() {
    let lits = vec![lit![0], lit![1], lit![2]];
    test_card!(
        CardConstraint::new_eq(lits.clone(), 1),
        vec![lit![0], !lit![1], !lit![2]],
        vec![!lit![0], lit![1], lit![2]]
    );
    test_card!(
        CardConstraint::new_eq(lits.clone(), 2),
        vec![lit![0], lit![1], !lit![2]],
        vec![!lit![0], !lit![1], !lit![2]]
    );
    test_card!(
        CardConstraint::new_eq(lits.clone(), 3),
        vec![lit![0], lit![1], lit![2]],
        vec![!lit![0], lit![1], !lit![2]]
    );
}

#[test]
fn pb_ub() {
    let mut lits = HashMap::new();
    lits.insert(lit![0], 1);
    lits.insert(lit![1], 2);
    lits.insert(lit![2], 3);
    test_pb!(
        PBConstraint::new_eq(lits.clone(), 1),
        vec![lit![0], !lit![1], !lit![2]],
        vec![!lit![0], lit![1], !lit![2]]
    );
    test_pb!(
        PBConstraint::new_eq(lits.clone(), 2),
        vec![!lit![0], lit![1], !lit![2]],
        vec![!lit![0], !lit![1], lit![2]]
    );
    test_pb!(
        PBConstraint::new_eq(lits.clone(), 3),
        vec![!lit![0], !lit![1], lit![2]],
        vec![lit![0], !lit![1], lit![2]]
    );
}

#[test]
fn pb_lb() {
    let mut lits = HashMap::new();
    lits.insert(lit![0], 1);
    lits.insert(lit![1], 2);
    lits.insert(lit![2], 3);
    test_pb!(
        PBConstraint::new_lb(lits.clone(), 1),
        vec![lit![0], lit![1], !lit![2]],
        vec![!lit![0], !lit![1], !lit![2]]
    );
    test_pb!(
        PBConstraint::new_lb(lits.clone(), 2),
        vec![lit![0], lit![1], !lit![2]],
        vec![lit![0], !lit![1], !lit![2]]
    );
    test_pb!(
        PBConstraint::new_lb(lits.clone(), 3),
        vec![!lit![0], !lit![1], lit![2]],
        vec![!lit![0], lit![1], !lit![2]]
    );
}

#[test]
fn pb_eq() {
    let mut lits = HashMap::new();
    lits.insert(lit![0], 1);
    lits.insert(lit![1], 2);
    lits.insert(lit![2], 3);
    test_pb!(
        PBConstraint::new_eq(lits.clone(), 1),
        vec![lit![0], !lit![1], !lit![2]],
        vec![!lit![0], lit![1], lit![2]]
    );
    test_pb!(
        PBConstraint::new_eq(lits.clone(), 2),
        vec![!lit![0], lit![1], !lit![2]],
        vec![!lit![0], !lit![1], !lit![2]]
    );
    test_pb!(
        PBConstraint::new_eq(lits.clone(), 3),
        vec![!lit![0], !lit![1], lit![2]],
        vec![!lit![0], lit![1], !lit![2]]
    );
}