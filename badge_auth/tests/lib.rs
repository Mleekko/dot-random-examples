use std::env;

use scrypto_unit::*;
use transaction::prelude::*;

use dot_random_test_utils::{deploy_random_component};

#[test]
fn test_request_mint_badge_auth() {
    // dir is different in Debug mode
    let root_dir = env::current_dir().ok().unwrap().ends_with("dot-random-examples");
    let dir_example = if root_dir { "./badge_auth" } else { "../badge_auth" };
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();

    // Deploy RandomComponent
    let test_util = deploy_random_component(&mut test_runner, "d23b065");

    // Deploy ExampleCaller
    let package_address2 = test_runner.publish_retain_blueprints(
        dir_example,
        |blueprint, _| blueprint.eq("ExampleCallerBadgeAuth"),
    );
    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_fee_from_faucet()
            .call_function(
                package_address2,
                "ExampleCallerBadgeAuth",
                "instantiate",
                manifest_args!(),
            )
            .build(), vec![]);

    let result = receipt.expect_commit_success();
    let example_component = result.new_component_addresses()[0];

    // Act
    // 1. Request mint - should return callback id: 1
    let receipt = test_runner.execute_manifest_ignoring_fee(
        ManifestBuilder::new()
            .call_method(
                example_component,
                "request_mint",
                manifest_args!(),
            )
            .build(), vec![]);
    let result = receipt.expect_commit_success();
    let out = result.outcome.expect_success();
    out[1].expect_return_value(&1u32);

    // 2. Watcher calls RandomComponent.process() to do the actual mint - should mint an NFT
    test_util.process_num(&mut test_runner, 100);

    // Assert
}
