use hbb_common::tokio;
use std::sync::atomic::{AtomicBool, Ordering};

// Mock the static variables for testing
static TEST_MUST_LOGIN: AtomicBool = AtomicBool::new(false);
static TEST_MUST_LOGIN_PEER: AtomicBool = AtomicBool::new(true);

#[test]
fn test_must_login_peer_default_value() {
    // Test that MUST_LOGIN_PEER defaults to true (Y)
    assert_eq!(TEST_MUST_LOGIN_PEER.load(Ordering::SeqCst), true);
}

#[test]
fn test_must_login_peer_configuration() {
    // Test setting MUST_LOGIN_PEER to false (N)
    TEST_MUST_LOGIN_PEER.store(false, Ordering::SeqCst);
    assert_eq!(TEST_MUST_LOGIN_PEER.load(Ordering::SeqCst), false);
    
    // Test setting MUST_LOGIN_PEER to true (Y)
    TEST_MUST_LOGIN_PEER.store(true, Ordering::SeqCst);
    assert_eq!(TEST_MUST_LOGIN_PEER.load(Ordering::SeqCst), true);
}

#[test]
fn test_configuration_combinations() {
    // Test Case 1: MUST_LOGIN=N (total switch off)
    TEST_MUST_LOGIN.store(false, Ordering::SeqCst);
    TEST_MUST_LOGIN_PEER.store(true, Ordering::SeqCst);
    
    let should_verify_controller = TEST_MUST_LOGIN.load(Ordering::SeqCst);
    let should_verify_peer = TEST_MUST_LOGIN.load(Ordering::SeqCst) && TEST_MUST_LOGIN_PEER.load(Ordering::SeqCst);
    
    assert_eq!(should_verify_controller, false);
    assert_eq!(should_verify_peer, false);
    
    // Test Case 2: MUST_LOGIN=Y, MUST_LOGIN_PEER=Y (default behavior)
    TEST_MUST_LOGIN.store(true, Ordering::SeqCst);
    TEST_MUST_LOGIN_PEER.store(true, Ordering::SeqCst);
    
    let should_verify_controller = TEST_MUST_LOGIN.load(Ordering::SeqCst);
    let should_verify_peer = TEST_MUST_LOGIN.load(Ordering::SeqCst) && TEST_MUST_LOGIN_PEER.load(Ordering::SeqCst);
    
    assert_eq!(should_verify_controller, true);
    assert_eq!(should_verify_peer, true);
    
    // Test Case 3: MUST_LOGIN=Y, MUST_LOGIN_PEER=N (new feature)
    TEST_MUST_LOGIN.store(true, Ordering::SeqCst);
    TEST_MUST_LOGIN_PEER.store(false, Ordering::SeqCst);
    
    let should_verify_controller = TEST_MUST_LOGIN.load(Ordering::SeqCst);
    let should_verify_peer = TEST_MUST_LOGIN.load(Ordering::SeqCst) && TEST_MUST_LOGIN_PEER.load(Ordering::SeqCst);
    
    assert_eq!(should_verify_controller, true);
    assert_eq!(should_verify_peer, false);
}

#[test]
fn test_environment_variable_parsing_logic() {
    // Test the logic for parsing MUST_LOGIN_PEER environment variable
    // This simulates the logic in rendezvous_server.rs
    
    // Case 1: Empty string should use default (true)
    let env_value = "";
    let should_set_false = env_value.to_uppercase() == "N" || 
        (env_value == "" && "".to_uppercase() == "N");
    assert_eq!(should_set_false, false); // Should remain true (default)
    
    // Case 2: "N" should set to false
    let env_value = "N";
    let should_set_false = env_value.to_uppercase() == "N" || 
        (env_value == "" && "".to_uppercase() == "N");
    assert_eq!(should_set_false, true); // Should set to false
    
    // Case 3: "Y" should remain true (default)
    let env_value = "Y";
    let should_set_false = env_value.to_uppercase() == "N" || 
        (env_value == "" && "".to_uppercase() == "N");
    assert_eq!(should_set_false, false); // Should remain true
    
    // Case 4: Case insensitive
    let env_value = "n";
    let should_set_false = env_value.to_uppercase() == "N" || 
        (env_value == "" && "".to_uppercase() == "N");
    assert_eq!(should_set_false, true); // Should set to false
}

#[tokio::test]
async fn test_connection_scenarios() {
    // This test simulates different connection scenarios
    
    // Scenario 1: Controller has token, peer login not required
    TEST_MUST_LOGIN.store(true, Ordering::SeqCst);
    TEST_MUST_LOGIN_PEER.store(false, Ordering::SeqCst);
    
    let controller_has_token = true;
    let peer_has_login = false;
    
    let controller_should_pass = !TEST_MUST_LOGIN.load(Ordering::SeqCst) || controller_has_token;
    let peer_should_pass = !TEST_MUST_LOGIN.load(Ordering::SeqCst) || 
                          !TEST_MUST_LOGIN_PEER.load(Ordering::SeqCst) || 
                          peer_has_login;
    
    assert_eq!(controller_should_pass, true);
    assert_eq!(peer_should_pass, true); // Connection should succeed
    
    // Scenario 2: Controller has token, peer login required but peer not logged in
    TEST_MUST_LOGIN.store(true, Ordering::SeqCst);
    TEST_MUST_LOGIN_PEER.store(true, Ordering::SeqCst);
    
    let controller_has_token = true;
    let peer_has_login = false;
    
    let controller_should_pass = !TEST_MUST_LOGIN.load(Ordering::SeqCst) || controller_has_token;
    let peer_should_pass = !TEST_MUST_LOGIN.load(Ordering::SeqCst) || 
                          !TEST_MUST_LOGIN_PEER.load(Ordering::SeqCst) || 
                          peer_has_login;
    
    assert_eq!(controller_should_pass, true);
    assert_eq!(peer_should_pass, false); // Connection should fail
    
    // Scenario 3: Controller has no token
    TEST_MUST_LOGIN.store(true, Ordering::SeqCst);
    TEST_MUST_LOGIN_PEER.store(false, Ordering::SeqCst);
    
    let controller_has_token = false;
    let peer_has_login = false;
    
    let controller_should_pass = !TEST_MUST_LOGIN.load(Ordering::SeqCst) || controller_has_token;
    let peer_should_pass = !TEST_MUST_LOGIN.load(Ordering::SeqCst) || 
                          !TEST_MUST_LOGIN_PEER.load(Ordering::SeqCst) || 
                          peer_has_login;
    
    assert_eq!(controller_should_pass, false); // Connection should fail at controller level
    assert_eq!(peer_should_pass, true);
}
