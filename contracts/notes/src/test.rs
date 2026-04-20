#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_study_planner() {
    let env = Env::default();
    env.mock_all_auths(); // Memalsukan autentikasi wallet untuk keperluan testing

    let contract_id = env.register_contract(None, StudyPlannerContract);
    let client = StudyPlannerContractClient::new(&env, &contract_id);

    // Buat akun tiruan untuk testing
    let user1 = Address::generate(&env);

    // 1. Test Create Plan
    let subject = String::from_str(&env, "Matematika");
    let topic = String::from_str(&env, "Aljabar Linear");
    let response = client.create_plan(&user1, &subject, &topic);
    assert_eq!(response, String::from_str(&env, "Study plan successfully added"));

    // 2. Test Get Plans
    let plans = client.get_plans(&user1);
    assert_eq!(plans.len(), 1);
    assert_eq!(plans.get(0).unwrap().subject, subject);
    assert_eq!(plans.get(0).unwrap().topic, topic);

    // 3. Test Delete Plan
    let plan_id = plans.get(0).unwrap().id;
    let delete_response = client.delete_plan(&user1, &plan_id);
    assert_eq!(delete_response, String::from_str(&env, "Study plan successfully deleted"));

    // Pastikan plan sudah kosong
    let plans_after_delete = client.get_plans(&user1);
    assert_eq!(plans_after_delete.len(), 0);
}