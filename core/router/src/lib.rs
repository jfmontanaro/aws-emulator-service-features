// SPDX-License-Identifier: CC0-1.0

#![forbid(unsafe_code)]
#![doc(hidden)]
//! Router — assembles all services into the multi-service emulator.
//!
//! # Writing integration tests
//!
//! Tests live in `router/tests/all/<name>.rs`, each declared in `router/tests/all.rs`.
//! Every router test target links all services, so the router has exactly one; a file
//! directly under `router/tests/` fails the build. `build_router(engine)` returns the
//! `MultiServiceRouter` the production entrypoint serves; drive requests through
//! `router.dispatch(req)` and assert on the response. The framework handles
//! deserialization, routing, and engine injection.
//!
//! ## Pattern
//!
//! ```rust,ignore
//! use std::sync::Arc;
//!
//! #[tokio::test]
//! async fn test_create_thing() {
//!     let engine = Arc::new(engine::InMemoryEngine::new());
//!     let router = build_router(engine);
//!     let req = http::Request::builder()
//!         .method("POST")
//!         .uri("http://localhost/2021-01-01/things")
//!         .header("authorization", /* a SigV4 header naming the service */ "...")
//!         .body(bytes::Bytes::from_static(br#"{"Name":"my-thing"}"#))
//!         .unwrap();
//!     let resp = router.dispatch(req).await;
//!     // 501 until you implement the operation — a FRESHLY SCAFFOLDED operation
//!     // answers `NotImplemented`, so asserting 200 here fails until the `handle`
//!     // body is written. Change this to 200 as part of implementing it.
//!     assert_eq!(resp.status().as_u16(), 501);
//! }
//! ```
//!
//! ## Key points
//!
//! - **Direct construction**: each service is built from the shared engine via
//!   `<service>::EmulatedService::new(engine.clone())`, which constructs each
//!   operation's authored `impl Operation` struct through the engine-as-constructor
//!   seam. There is no builder and no per-operation registration step.
//! - **Routing**: `MultiServiceRouter` picks the service whose `can_handle` claims
//!   the request (seeded from the model's SigV4 signing name), then the service's
//!   protocol names the operation.
//! - **Deps**: a router integration test needs `service-foo`, `engine`, `http`,
//!   `bytes` and `tokio`. Put the test in the SERVICE crate rather than here:
//!   `router/Cargo.toml` is re-rendered on every service add (see the banner at the
//!   top of this file), so a dev-dependency added there is overwritten.

// @generated — RE-RENDERED ON EVERY SERVICE ADD. Edits below are OVERWRITTEN.
//
// Put hand-written code in a SERVICE crate, or in `router/src/main.rs`. Those are
// yours; this file, `router/Cargo.toml` and the workspace `Cargo.toml` are replaced
// whenever a service is added, so an edit here is lost.

use smithy_server_runtime::MultiServiceRouter;
use std::sync::Arc;
pub use engine::InMemoryEngine;

pub fn new_engine() -> Arc<InMemoryEngine> {
    Arc::new(InMemoryEngine::new())
}

/// Assemble the multi-service router the production entrypoint serves.
///
/// Registrations are DERIVED from the scaffolded service set, not hand-maintained:
/// adding a service re-renders this whole file. `router/tests/all/boot_serve.rs` exercises
/// this function.
///
/// Keep this a plain `fn` whose registrations stay in the `#[inline(never)]`
/// `mount_chunk_NN` helpers below, one frame at a time. Collapsing them into a single
/// chained expression holds every service's `new` frame at once and overflows
/// the stack here, aborting before either listener binds.
pub fn build_router(engine: Arc<InMemoryEngine>) -> MultiServiceRouter {
    let router = MultiServiceRouter::new();
    let router = mount_chunk_00(router, &engine);
    let router = mount_chunk_01(router, &engine);
    let router = mount_chunk_02(router, &engine);
    let router = mount_chunk_03(router, &engine);
    let router = mount_chunk_04(router, &engine);
    let router = mount_chunk_05(router, &engine);
    let router = mount_chunk_06(router, &engine);
    let router = mount_chunk_07(router, &engine);
    let router = mount_chunk_08(router, &engine);
    let router = mount_chunk_09(router, &engine);
    let router = mount_chunk_10(router, &engine);
    let router = mount_chunk_11(router, &engine);
    let router = mount_chunk_12(router, &engine);
    let router = mount_chunk_13(router, &engine);
    let router = mount_chunk_14(router, &engine);
    let router = mount_chunk_15(router, &engine);
    let router = mount_chunk_16(router, &engine);
    let router = mount_chunk_17(router, &engine);
    let router = mount_chunk_18(router, &engine);
    let router = mount_chunk_19(router, &engine);
    let router = mount_chunk_20(router, &engine);
    let router = mount_chunk_21(router, &engine);
    let router = mount_chunk_22(router, &engine);
    let router = mount_chunk_23(router, &engine);
    let router = mount_chunk_24(router, &engine);
    let router = mount_chunk_25(router, &engine);
    let router = mount_chunk_26(router, &engine);
    let router = mount_chunk_27(router, &engine);
    let router = mount_chunk_28(router, &engine);
    let router = mount_chunk_29(router, &engine);
    let router = mount_chunk_30(router, &engine);
    let router = mount_chunk_31(router, &engine);
    let router = mount_chunk_32(router, &engine);
    let router = mount_chunk_33(router, &engine);
    let router = mount_chunk_34(router, &engine);
    let router = mount_chunk_35(router, &engine);
    let router = mount_chunk_36(router, &engine);
    let router = mount_chunk_37(router, &engine);
    let router = mount_chunk_38(router, &engine);
    mount_chunk_39(router, &engine)
}

/// Register services 00 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_00(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "accessanalyzer")]
    router.register_service(service_com_amazonaws_accessanalyzer_accessanalyzer::EmulatedService::new(engine.clone()));
    #[cfg(feature = "account")]
    router.register_service(service_com_amazonaws_account_account::EmulatedService::new(engine.clone()));
    #[cfg(feature = "accountaccess")]
    router.register_service(service_com_amazonaws_accountaccess_awsaccountaccess::EmulatedService::new(engine.clone()));
    #[cfg(feature = "accountaccessruntime")]
    router.register_service(service_com_amazonaws_accountaccessruntime_awsaccountaccessruntime::EmulatedService::new(engine.clone()));
    #[cfg(feature = "acm")]
    router.register_service(service_com_amazonaws_acm_certificatemanager::EmulatedService::new(engine.clone()));
    #[cfg(feature = "acmpca")]
    router.register_service(service_com_amazonaws_acmpca_acmprivateca::EmulatedService::new(engine.clone()));
    #[cfg(feature = "agentregistry")]
    router.register_service(service_com_amazonaws_agentregistry_agentregistry::EmulatedService::new(engine.clone()));
    #[cfg(feature = "agentregistrycontrol")]
    router.register_service(service_com_amazonaws_agentregistrycontrol_agentregistrycontrol::EmulatedService::new(engine.clone()));
    #[cfg(feature = "aiops")]
    router.register_service(service_com_amazonaws_aiops_aiops::EmulatedService::new(engine.clone()));
    #[cfg(feature = "amp")]
    router.register_service(service_com_amazonaws_amp_amazonprometheusservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "amplify")]
    router.register_service(service_com_amazonaws_amplify_amplify::EmulatedService::new(engine.clone()));
    router
}

/// Register services 01 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_01(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "amplifybackend")]
    router.register_service(service_com_amazonaws_amplifybackend_amplifybackend::EmulatedService::new(engine.clone()));
    #[cfg(feature = "amplifyuibuilder")]
    router.register_service(service_com_amazonaws_amplifyuibuilder_amplifyuibuilder::EmulatedService::new(engine.clone()));
    #[cfg(feature = "apigateway")]
    router.register_service(service_com_amazonaws_apigateway_backplanecontrolservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "apigatewaymanagementapi")]
    router.register_service(service_com_amazonaws_apigatewaymanagementapi_apigatewaymanagementapi::EmulatedService::new(engine.clone()));
    #[cfg(feature = "apigatewayv2")]
    router.register_service(service_com_amazonaws_apigatewayv2_apigatewayv2::EmulatedService::new(engine.clone()));
    #[cfg(feature = "appconfig")]
    router.register_service(service_com_amazonaws_appconfig_amazonappconfig::EmulatedService::new(engine.clone()));
    #[cfg(feature = "appconfigdata")]
    router.register_service(service_com_amazonaws_appconfigdata_appconfigdata::EmulatedService::new(engine.clone()));
    #[cfg(feature = "appfabric")]
    router.register_service(service_com_amazonaws_appfabric_fabricfrontendservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "appflow")]
    router.register_service(service_com_amazonaws_appflow_sandstoneconfigurationservicelambda::EmulatedService::new(engine.clone()));
    #[cfg(feature = "appintegrations")]
    router.register_service(service_com_amazonaws_appintegrations_amazonappintegrationservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "applicationautoscaling")]
    router.register_service(service_com_amazonaws_applicationautoscaling_anyscalefrontendservice::EmulatedService::new(engine.clone()));
    router
}

/// Register services 02 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_02(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "applicationcostprofiler")]
    router.register_service(service_com_amazonaws_applicationcostprofiler_awsapplicationcostprofiler::EmulatedService::new(engine.clone()));
    #[cfg(feature = "applicationdiscoveryservice")]
    router.register_service(service_com_amazonaws_applicationdiscoveryservice_awsposeidonservice_v2015_11_01::EmulatedService::new(engine.clone()));
    #[cfg(feature = "applicationinsights")]
    router.register_service(service_com_amazonaws_applicationinsights_ec2windowsbarleyservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "applicationsignals")]
    router.register_service(service_com_amazonaws_applicationsignals_applicationsignals::EmulatedService::new(engine.clone()));
    #[cfg(feature = "appmesh")]
    router.register_service(service_com_amazonaws_appmesh_appmesh::EmulatedService::new(engine.clone()));
    #[cfg(feature = "apprunner")]
    router.register_service(service_com_amazonaws_apprunner_apprunner::EmulatedService::new(engine.clone()));
    #[cfg(feature = "appstream")]
    router.register_service(service_com_amazonaws_appstream_photonadminproxyservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "appsync")]
    router.register_service(service_com_amazonaws_appsync_awsdeepdishcontrolplaneservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "arcregionswitch")]
    router.register_service(service_com_amazonaws_arcregionswitch_arcregionswitch::EmulatedService::new(engine.clone()));
    #[cfg(feature = "arczonalshift")]
    router.register_service(service_com_amazonaws_arczonalshift_percdataplane::EmulatedService::new(engine.clone()));
    #[cfg(feature = "artifact")]
    router.register_service(service_com_amazonaws_artifact_artifact::EmulatedService::new(engine.clone()));
    router
}

/// Register services 03 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_03(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "athena")]
    router.register_service(service_com_amazonaws_athena_amazonathena::EmulatedService::new(engine.clone()));
    #[cfg(feature = "auditmanager")]
    router.register_service(service_com_amazonaws_auditmanager_bedrockassessmentmanagerlambda::EmulatedService::new(engine.clone()));
    #[cfg(feature = "autoscaling")]
    router.register_service(service_com_amazonaws_autoscaling_autoscaling_2011_01_01::EmulatedService::new(engine.clone()));
    #[cfg(feature = "autoscalingplans")]
    router.register_service(service_com_amazonaws_autoscalingplans_anyscalescalingplannerfrontendservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "b2bi")]
    router.register_service(service_com_amazonaws_b2bi_b2bi::EmulatedService::new(engine.clone()));
    #[cfg(feature = "backup")]
    router.register_service(service_com_amazonaws_backup_cryocontrollerusermanager::EmulatedService::new(engine.clone()));
    #[cfg(feature = "backupgateway")]
    router.register_service(service_com_amazonaws_backupgateway_backuponpremises_v20210101::EmulatedService::new(engine.clone()));
    #[cfg(feature = "backupsearch")]
    router.register_service(service_com_amazonaws_backupsearch_cryobackupsearchservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "batch")]
    router.register_service(service_com_amazonaws_batch_awsbatchv20160810::EmulatedService::new(engine.clone()));
    #[cfg(feature = "bcmdashboards")]
    router.register_service(service_com_amazonaws_bcmdashboards_awsbcmdashboardsservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "bcmdataexports")]
    router.register_service(service_com_amazonaws_bcmdataexports_awsbillingandcostmanagementdataexports::EmulatedService::new(engine.clone()));
    router
}

/// Register services 04 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_04(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "bcmpricingcalculator")]
    router.register_service(service_com_amazonaws_bcmpricingcalculator_awsbcmpricingcalculator::EmulatedService::new(engine.clone()));
    #[cfg(feature = "bcmrecommendedactions")]
    router.register_service(service_com_amazonaws_bcmrecommendedactions_awsbillingandcostmanagementrecommendedactions::EmulatedService::new(engine.clone()));
    #[cfg(feature = "bedrock")]
    router.register_service(service_com_amazonaws_bedrock_amazonbedrockcontrolplaneservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "bedrockagent")]
    router.register_service(service_com_amazonaws_bedrockagent_amazonbedrockagentbuildtimelambda::EmulatedService::new(engine.clone()));
    #[cfg(feature = "bedrockagentcore")]
    router.register_service(service_com_amazonaws_bedrockagentcore_amazonbedrockagentcore::EmulatedService::new(engine.clone()));
    #[cfg(feature = "bedrockagentcorecontrol")]
    router.register_service(service_com_amazonaws_bedrockagentcorecontrol_amazonbedrockagentcorecontrol::EmulatedService::new(engine.clone()));
    #[cfg(feature = "bedrockagentruntime")]
    router.register_service(service_com_amazonaws_bedrockagentruntime_amazonbedrockagentruntimeservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "bedrockdataautomation")]
    router.register_service(service_com_amazonaws_bedrockdataautomation_amazonbedrockkeystonebuildtimeservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "bedrockdataautomationruntime")]
    router.register_service(service_com_amazonaws_bedrockdataautomationruntime_amazonbedrockkeystoneruntimeservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "bedrockruntime")]
    router.register_service(service_com_amazonaws_bedrockruntime_amazonbedrockfrontendservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "billing")]
    router.register_service(service_com_amazonaws_billing_awsbilling::EmulatedService::new(engine.clone()));
    router
}

/// Register services 05 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_05(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "billingconductor")]
    router.register_service(service_com_amazonaws_billingconductor_awsbillingconductor::EmulatedService::new(engine.clone()));
    #[cfg(feature = "braket")]
    router.register_service(service_com_amazonaws_braket_braket::EmulatedService::new(engine.clone()));
    #[cfg(feature = "budgets")]
    router.register_service(service_com_amazonaws_budgets_awsbudgetservicegateway::EmulatedService::new(engine.clone()));
    #[cfg(feature = "chatbot")]
    router.register_service(service_com_amazonaws_chatbot_wheatleyorchestration_20171011::EmulatedService::new(engine.clone()));
    #[cfg(feature = "chime")]
    router.register_service(service_com_amazonaws_chime_ucbuzzconsoleservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "chimesdkidentity")]
    router.register_service(service_com_amazonaws_chimesdkidentity_chimeidentityservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "chimesdkmediapipelines")]
    router.register_service(service_com_amazonaws_chimesdkmediapipelines_chimesdkmediapipelinesservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "chimesdkmeetings")]
    router.register_service(service_com_amazonaws_chimesdkmeetings_chimemeetingssdkservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "chimesdkmessaging")]
    router.register_service(service_com_amazonaws_chimesdkmessaging_chimemessagingservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "chimesdkvoice")]
    router.register_service(service_com_amazonaws_chimesdkvoice_chimesdktelephonyservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "cleanrooms")]
    router.register_service(service_com_amazonaws_cleanrooms_awsbastioncontrolplaneservicelambda::EmulatedService::new(engine.clone()));
    router
}

/// Register services 06 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_06(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "cleanroomsml")]
    router.register_service(service_com_amazonaws_cleanroomsml_awsstarkcontrolservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "cloud9")]
    router.register_service(service_com_amazonaws_cloud9_awscloud9workspacemanagementservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "cloudcontrol")]
    router.register_service(service_com_amazonaws_cloudcontrol_cloudapiservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "clouddirectory")]
    router.register_service(service_com_amazonaws_clouddirectory_amazonclouddirectory_20170111::EmulatedService::new(engine.clone()));
    #[cfg(feature = "cloudformation")]
    router.register_service(service_com_amazonaws_cloudformation_cloudformation::EmulatedService::new(engine.clone()));
    #[cfg(feature = "cloudfront")]
    router.register_service(service_com_amazonaws_cloudfront_cloudfront2020_05_31::EmulatedService::new(engine.clone()));
    #[cfg(feature = "cloudfrontkeyvaluestore")]
    router.register_service(service_com_amazonaws_cloudfrontkeyvaluestore_cloudfrontkeyvaluestore::EmulatedService::new(engine.clone()));
    #[cfg(feature = "cloudhsm")]
    router.register_service(service_com_amazonaws_cloudhsm_cloudhsmfrontendservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "cloudhsmv2")]
    router.register_service(service_com_amazonaws_cloudhsmv2_baldrapiservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "cloudsearch")]
    router.register_service(service_com_amazonaws_cloudsearch_a9searchcloudconfigservice2013::EmulatedService::new(engine.clone()));
    #[cfg(feature = "cloudsearchdomain")]
    router.register_service(service_com_amazonaws_cloudsearchdomain_amazoncloudsearch2013::EmulatedService::new(engine.clone()));
    router
}

/// Register services 07 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_07(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "cloudtrail")]
    router.register_service(service_com_amazonaws_cloudtrail_cloudtrail_20131101::EmulatedService::new(engine.clone()));
    #[cfg(feature = "cloudtraildata")]
    router.register_service(service_com_amazonaws_cloudtraildata_cloudtraildataservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "cloudwatch")]
    router.register_service(service_com_amazonaws_cloudwatch_graniteserviceversion20100801::EmulatedService::new(engine.clone()));
    #[cfg(feature = "cloudwatchevents")]
    router.register_service(service_com_amazonaws_cloudwatchevents_awsevents::EmulatedService::new(engine.clone()));
    #[cfg(feature = "cloudwatchlogs")]
    router.register_service(service_com_amazonaws_cloudwatchlogs_logs_20140328::EmulatedService::new(engine.clone()));
    #[cfg(feature = "cloudwatchomni")]
    router.register_service(service_com_amazonaws_cloudwatchomni_cloudwatchomnifrontend::EmulatedService::new(engine.clone()));
    #[cfg(feature = "codeartifact")]
    router.register_service(service_com_amazonaws_codeartifact_codeartifactcontrolplaneservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "codebuild")]
    router.register_service(service_com_amazonaws_codebuild_codebuild_20161006::EmulatedService::new(engine.clone()));
    #[cfg(feature = "codecatalyst")]
    router.register_service(service_com_amazonaws_codecatalyst_codecatalyst::EmulatedService::new(engine.clone()));
    #[cfg(feature = "codecommit")]
    router.register_service(service_com_amazonaws_codecommit_codecommit_20150413::EmulatedService::new(engine.clone()));
    #[cfg(feature = "codeconnections")]
    router.register_service(service_com_amazonaws_codeconnections_codeconnections_20231201::EmulatedService::new(engine.clone()));
    router
}

/// Register services 08 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_08(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "codedeploy")]
    router.register_service(service_com_amazonaws_codedeploy_codedeploy_20141006::EmulatedService::new(engine.clone()));
    #[cfg(feature = "codeguruprofiler")]
    router.register_service(service_com_amazonaws_codeguruprofiler_codeguruprofiler::EmulatedService::new(engine.clone()));
    #[cfg(feature = "codegurureviewer")]
    router.register_service(service_com_amazonaws_codegurureviewer_awsgurufrontendservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "codegurusecurity")]
    router.register_service(service_com_amazonaws_codegurusecurity_awscodegurusecurity::EmulatedService::new(engine.clone()));
    #[cfg(feature = "codepipeline")]
    router.register_service(service_com_amazonaws_codepipeline_codepipeline_20150709::EmulatedService::new(engine.clone()));
    #[cfg(feature = "codestarconnections")]
    router.register_service(service_com_amazonaws_codestarconnections_codestar_connections_20191201::EmulatedService::new(engine.clone()));
    #[cfg(feature = "codestarnotifications")]
    router.register_service(service_com_amazonaws_codestarnotifications_codestarnotifications_20191015::EmulatedService::new(engine.clone()));
    #[cfg(feature = "cognitoidentity")]
    router.register_service(service_com_amazonaws_cognitoidentity_awscognitoidentityservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "cognitoidentityprovider")]
    router.register_service(service_com_amazonaws_cognitoidentityprovider_awscognitoidentityproviderservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "cognitosync")]
    router.register_service(service_com_amazonaws_cognitosync_awscognitosyncservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "comprehend")]
    router.register_service(service_com_amazonaws_comprehend_comprehend_20171127::EmulatedService::new(engine.clone()));
    router
}

/// Register services 09 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_09(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "comprehendmedical")]
    router.register_service(service_com_amazonaws_comprehendmedical_comprehendmedical_20181030::EmulatedService::new(engine.clone()));
    #[cfg(feature = "computeoptimizer")]
    router.register_service(service_com_amazonaws_computeoptimizer_computeoptimizerservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "computeoptimizerautomation")]
    router.register_service(service_com_amazonaws_computeoptimizerautomation_computeoptimizerautomationservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "configservice")]
    router.register_service(service_com_amazonaws_configservice_starlingdoveservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "connect")]
    router.register_service(service_com_amazonaws_connect_amazonconnectservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "connectcampaigns")]
    router.register_service(service_com_amazonaws_connectcampaigns_amazonconnectcampaignservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "connectcampaignsv2")]
    router.register_service(service_com_amazonaws_connectcampaignsv2_amazonconnectcampaignservicev2::EmulatedService::new(engine.clone()));
    #[cfg(feature = "connectcases")]
    router.register_service(service_com_amazonaws_connectcases_amazonconnectcases::EmulatedService::new(engine.clone()));
    #[cfg(feature = "connectcontactlens")]
    router.register_service(service_com_amazonaws_connectcontactlens_amazonconnectcontactlens::EmulatedService::new(engine.clone()));
    #[cfg(feature = "connecthealth")]
    router.register_service(service_com_amazonaws_connecthealth_connecthealth::EmulatedService::new(engine.clone()));
    #[cfg(feature = "connectparticipant")]
    router.register_service(service_com_amazonaws_connectparticipant_amazonconnectparticipantservicelambda::EmulatedService::new(engine.clone()));
    router
}

/// Register services 10 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_10(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "continuum")]
    router.register_service(service_com_amazonaws_continuum_continuumcontrolplane::EmulatedService::new(engine.clone()));
    #[cfg(feature = "controlcatalog")]
    router.register_service(service_com_amazonaws_controlcatalog_controlcatalog::EmulatedService::new(engine.clone()));
    #[cfg(feature = "controltower")]
    router.register_service(service_com_amazonaws_controltower_awscontroltowerapis::EmulatedService::new(engine.clone()));
    #[cfg(feature = "costandusagereportservice")]
    router.register_service(service_com_amazonaws_costandusagereportservice_awsorigamiservicegatewayservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "costexplorer")]
    router.register_service(service_com_amazonaws_costexplorer_awsinsightsindexservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "costoptimizationhub")]
    router.register_service(service_com_amazonaws_costoptimizationhub_costoptimizationhubservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "customerprofiles")]
    router.register_service(service_com_amazonaws_customerprofiles_customerprofiles_20200815::EmulatedService::new(engine.clone()));
    #[cfg(feature = "databasemigrationservice")]
    router.register_service(service_com_amazonaws_databasemigrationservice_amazondmsv20160101::EmulatedService::new(engine.clone()));
    #[cfg(feature = "databrew")]
    router.register_service(service_com_amazonaws_databrew_awsgluedatabrew::EmulatedService::new(engine.clone()));
    #[cfg(feature = "dataexchange")]
    router.register_service(service_com_amazonaws_dataexchange_dataexchange::EmulatedService::new(engine.clone()));
    #[cfg(feature = "datapipeline")]
    router.register_service(service_com_amazonaws_datapipeline_datapipeline::EmulatedService::new(engine.clone()));
    router
}

/// Register services 11 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_11(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "datasync")]
    router.register_service(service_com_amazonaws_datasync_fmrsservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "datazone")]
    router.register_service(service_com_amazonaws_datazone_datazone::EmulatedService::new(engine.clone()));
    #[cfg(feature = "dax")]
    router.register_service(service_com_amazonaws_dax_amazondaxv3::EmulatedService::new(engine.clone()));
    #[cfg(feature = "deadline")]
    router.register_service(service_com_amazonaws_deadline_deadline::EmulatedService::new(engine.clone()));
    #[cfg(feature = "detective")]
    router.register_service(service_com_amazonaws_detective_amazondetective::EmulatedService::new(engine.clone()));
    #[cfg(feature = "devicefarm")]
    router.register_service(service_com_amazonaws_devicefarm_devicefarm_20150623::EmulatedService::new(engine.clone()));
    #[cfg(feature = "devopsagent")]
    router.register_service(service_com_amazonaws_devopsagent_devopsagent::EmulatedService::new(engine.clone()));
    #[cfg(feature = "devopsguru")]
    router.register_service(service_com_amazonaws_devopsguru_capstonecontrolplaneservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "directconnect")]
    router.register_service(service_com_amazonaws_directconnect_overtureservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "directoryservice")]
    router.register_service(service_com_amazonaws_directoryservice_directoryservice_20150416::EmulatedService::new(engine.clone()));
    #[cfg(feature = "directoryservicedata")]
    router.register_service(service_com_amazonaws_directoryservicedata_directoryservicedata::EmulatedService::new(engine.clone()));
    router
}

/// Register services 12 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_12(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "dlm")]
    router.register_service(service_com_amazonaws_dlm_dlm_20180112::EmulatedService::new(engine.clone()));
    #[cfg(feature = "docdb")]
    router.register_service(service_com_amazonaws_docdb_amazonrdsv19::EmulatedService::new(engine.clone()));
    #[cfg(feature = "docdbelastic")]
    router.register_service(service_com_amazonaws_docdbelastic_chimeradblionfishservicelambda::EmulatedService::new(engine.clone()));
    #[cfg(feature = "drs")]
    router.register_service(service_com_amazonaws_drs_elasticdisasterrecoveryservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "dsql")]
    router.register_service(service_com_amazonaws_dsql_dsql::EmulatedService::new(engine.clone()));
    #[cfg(feature = "dynamodb")]
    router.register_service(service_com_amazonaws_dynamodb_dynamodb_20120810::EmulatedService::new(engine.clone()));
    #[cfg(feature = "dynamodbstreams")]
    router.register_service(service_com_amazonaws_dynamodbstreams_dynamodbstreams_20120810::EmulatedService::new(engine.clone()));
    #[cfg(feature = "ebs")]
    router.register_service(service_com_amazonaws_ebs_ebs::EmulatedService::new(engine.clone()));
    #[cfg(feature = "ec2")]
    router.register_service(service_com_amazonaws_ec2_amazonec2::EmulatedService::new(engine.clone()));
    #[cfg(feature = "ec2instanceconnect")]
    router.register_service(service_com_amazonaws_ec2instanceconnect_awsec2instanceconnectservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "ecr")]
    router.register_service(service_com_amazonaws_ecr_amazonec2containerregistry_v20150921::EmulatedService::new(engine.clone()));
    router
}

/// Register services 13 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_13(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "ecrpublic")]
    router.register_service(service_com_amazonaws_ecrpublic_spencerfrontendservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "ecs")]
    router.register_service(service_com_amazonaws_ecs_amazonec2containerservicev20141113::EmulatedService::new(engine.clone()));
    #[cfg(feature = "efs")]
    router.register_service(service_com_amazonaws_efs_magnolioapiservice_v20150201::EmulatedService::new(engine.clone()));
    #[cfg(feature = "eks")]
    router.register_service(service_com_amazonaws_eks_awswesleyfrontend::EmulatedService::new(engine.clone()));
    #[cfg(feature = "eksauth")]
    router.register_service(service_com_amazonaws_eksauth_eksauthfrontend::EmulatedService::new(engine.clone()));
    #[cfg(feature = "elasticache")]
    router.register_service(service_com_amazonaws_elasticache_amazonelasticachev9::EmulatedService::new(engine.clone()));
    #[cfg(feature = "elasticbeanstalk")]
    router.register_service(service_com_amazonaws_elasticbeanstalk_awselasticbeanstalkservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "elasticloadbalancing")]
    router.register_service(service_com_amazonaws_elasticloadbalancing_elasticloadbalancing_v7::EmulatedService::new(engine.clone()));
    #[cfg(feature = "elasticloadbalancingv2")]
    router.register_service(service_com_amazonaws_elasticloadbalancingv2_elasticloadbalancing_v10::EmulatedService::new(engine.clone()));
    #[cfg(feature = "elasticsearchservice")]
    router.register_service(service_com_amazonaws_elasticsearchservice_amazonelasticsearchservice2015::EmulatedService::new(engine.clone()));
    #[cfg(feature = "elementalinference")]
    router.register_service(service_com_amazonaws_elementalinference_elementalinference::EmulatedService::new(engine.clone()));
    router
}

/// Register services 14 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_14(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "emr")]
    router.register_service(service_com_amazonaws_emr_elasticmapreduce::EmulatedService::new(engine.clone()));
    #[cfg(feature = "emrcontainers")]
    router.register_service(service_com_amazonaws_emrcontainers_awschicagowebservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "emrserverless")]
    router.register_service(service_com_amazonaws_emrserverless_awstoledowebservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "endusermessaging")]
    router.register_service(service_com_amazonaws_endusermessaging_endusermessaging::EmulatedService::new(engine.clone()));
    #[cfg(feature = "entityresolution")]
    router.register_service(service_com_amazonaws_entityresolution_awsveniceservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "eventbridge")]
    router.register_service(service_com_amazonaws_eventbridge_awsevents::EmulatedService::new(engine.clone()));
    #[cfg(feature = "eventbridgev2")]
    router.register_service(service_com_amazonaws_eventbridgev2_awseventsv2::EmulatedService::new(engine.clone()));
    #[cfg(feature = "evs")]
    router.register_service(service_com_amazonaws_evs_amazonelasticvmwareservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "firehose")]
    router.register_service(service_com_amazonaws_firehose_firehose_20150804::EmulatedService::new(engine.clone()));
    #[cfg(feature = "fis")]
    router.register_service(service_com_amazonaws_fis_faultinjectionsimulator::EmulatedService::new(engine.clone()));
    #[cfg(feature = "fms")]
    router.register_service(service_com_amazonaws_fms_awsfms_20180101::EmulatedService::new(engine.clone()));
    router
}

/// Register services 15 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_15(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "forecast")]
    router.register_service(service_com_amazonaws_forecast_amazonforecast::EmulatedService::new(engine.clone()));
    #[cfg(feature = "forecastquery")]
    router.register_service(service_com_amazonaws_forecastquery_amazonforecastruntime::EmulatedService::new(engine.clone()));
    #[cfg(feature = "frauddetector")]
    router.register_service(service_com_amazonaws_frauddetector_awshawksnestservicefacade::EmulatedService::new(engine.clone()));
    #[cfg(feature = "freetier")]
    router.register_service(service_com_amazonaws_freetier_awsfreetierservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "fsx")]
    router.register_service(service_com_amazonaws_fsx_awssimbaapiservice_v20180301::EmulatedService::new(engine.clone()));
    #[cfg(feature = "gamelift")]
    router.register_service(service_com_amazonaws_gamelift_gamelift::EmulatedService::new(engine.clone()));
    #[cfg(feature = "gameliftstreams")]
    router.register_service(service_com_amazonaws_gameliftstreams_gameliftstreams::EmulatedService::new(engine.clone()));
    #[cfg(feature = "geomaps")]
    router.register_service(service_com_amazonaws_geomaps_mapsservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "geoplaces")]
    router.register_service(service_com_amazonaws_geoplaces_placesservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "georoutes")]
    router.register_service(service_com_amazonaws_georoutes_routesservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "glacier")]
    router.register_service(service_com_amazonaws_glacier_glacier::EmulatedService::new(engine.clone()));
    router
}

/// Register services 16 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_16(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "globalaccelerator")]
    router.register_service(service_com_amazonaws_globalaccelerator_globalaccelerator_v20180706::EmulatedService::new(engine.clone()));
    #[cfg(feature = "glue")]
    router.register_service(service_com_amazonaws_glue_awsglue::EmulatedService::new(engine.clone()));
    #[cfg(feature = "grafana")]
    router.register_service(service_com_amazonaws_grafana_awsgrafanacontrolplane::EmulatedService::new(engine.clone()));
    #[cfg(feature = "greengrass")]
    router.register_service(service_com_amazonaws_greengrass_greengrass::EmulatedService::new(engine.clone()));
    #[cfg(feature = "greengrassv2")]
    router.register_service(service_com_amazonaws_greengrassv2_greengrassv2::EmulatedService::new(engine.clone()));
    #[cfg(feature = "groundstation")]
    router.register_service(service_com_amazonaws_groundstation_groundstation::EmulatedService::new(engine.clone()));
    #[cfg(feature = "guardduty")]
    router.register_service(service_com_amazonaws_guardduty_guarddutyapiservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "health")]
    router.register_service(service_com_amazonaws_health_awshealth_20160804::EmulatedService::new(engine.clone()));
    #[cfg(feature = "healthlake")]
    router.register_service(service_com_amazonaws_healthlake_healthlake::EmulatedService::new(engine.clone()));
    #[cfg(feature = "iam")]
    router.register_service(service_com_amazonaws_iam_awsidentitymanagementv20100508::EmulatedService::new(engine.clone()));
    #[cfg(feature = "iamtoolbox")]
    router.register_service(service_com_amazonaws_iamtoolbox_authrequestservice::EmulatedService::new(engine.clone()));
    router
}

/// Register services 17 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_17(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "identitystore")]
    router.register_service(service_com_amazonaws_identitystore_awsidentitystore::EmulatedService::new(engine.clone()));
    #[cfg(feature = "imagebuilder")]
    router.register_service(service_com_amazonaws_imagebuilder_imagebuilder::EmulatedService::new(engine.clone()));
    #[cfg(feature = "inspector")]
    router.register_service(service_com_amazonaws_inspector_inspectorservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "inspector2")]
    router.register_service(service_com_amazonaws_inspector2_inspector2::EmulatedService::new(engine.clone()));
    #[cfg(feature = "inspectorscan")]
    router.register_service(service_com_amazonaws_inspectorscan_inspectorscan::EmulatedService::new(engine.clone()));
    #[cfg(feature = "interconnect")]
    router.register_service(service_com_amazonaws_interconnect_interconnect::EmulatedService::new(engine.clone()));
    #[cfg(feature = "internetmonitor")]
    router.register_service(service_com_amazonaws_internetmonitor_internetmonitor20210603::EmulatedService::new(engine.clone()));
    #[cfg(feature = "invoicing")]
    router.register_service(service_com_amazonaws_invoicing_invoicing::EmulatedService::new(engine.clone()));
    #[cfg(feature = "iot")]
    router.register_service(service_com_amazonaws_iot_awsiotservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "iotdataplane")]
    router.register_service(service_com_amazonaws_iotdataplane_iotmoonrakerservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "iotdeviceadvisor")]
    router.register_service(service_com_amazonaws_iotdeviceadvisor_iotsenateservice::EmulatedService::new(engine.clone()));
    router
}

/// Register services 18 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_18(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "iotfleetwise")]
    router.register_service(service_com_amazonaws_iotfleetwise_iotautobahncontrolplane::EmulatedService::new(engine.clone()));
    #[cfg(feature = "iotjobsdataplane")]
    router.register_service(service_com_amazonaws_iotjobsdataplane_iotlaserthingjobmanagerexternalservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "iotmanagedintegrations")]
    router.register_service(service_com_amazonaws_iotmanagedintegrations_iotmanagedintegrations::EmulatedService::new(engine.clone()));
    #[cfg(feature = "iotsecuretunneling")]
    router.register_service(service_com_amazonaws_iotsecuretunneling_iotsecuredtunneling::EmulatedService::new(engine.clone()));
    #[cfg(feature = "iotsitewise")]
    router.register_service(service_com_amazonaws_iotsitewise_awsiotsitewise::EmulatedService::new(engine.clone()));
    #[cfg(feature = "iotthingsgraph")]
    router.register_service(service_com_amazonaws_iotthingsgraph_iotthingsgraphfrontendservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "iottwinmaker")]
    router.register_service(service_com_amazonaws_iottwinmaker_awsiottwinmaker::EmulatedService::new(engine.clone()));
    #[cfg(feature = "iotwireless")]
    router.register_service(service_com_amazonaws_iotwireless_iotwireless::EmulatedService::new(engine.clone()));
    #[cfg(feature = "ivs")]
    router.register_service(service_com_amazonaws_ivs_amazoninteractivevideoservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "ivschat")]
    router.register_service(service_com_amazonaws_ivschat_amazoninteractivevideoservicechat::EmulatedService::new(engine.clone()));
    #[cfg(feature = "ivsrealtime")]
    router.register_service(service_com_amazonaws_ivsrealtime_amazoninteractivevideoservicerealtime::EmulatedService::new(engine.clone()));
    router
}

/// Register services 19 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_19(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "kafka")]
    router.register_service(service_com_amazonaws_kafka_kafka::EmulatedService::new(engine.clone()));
    #[cfg(feature = "kafkaconnect")]
    router.register_service(service_com_amazonaws_kafkaconnect_kafkaconnect::EmulatedService::new(engine.clone()));
    #[cfg(feature = "kendra")]
    router.register_service(service_com_amazonaws_kendra_awskendrafrontendservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "kendraranking")]
    router.register_service(service_com_amazonaws_kendraranking_awskendrarerankingfrontendservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "keyspaces")]
    router.register_service(service_com_amazonaws_keyspaces_keyspacesservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "keyspacesstreams")]
    router.register_service(service_com_amazonaws_keyspacesstreams_keyspacesstreams::EmulatedService::new(engine.clone()));
    #[cfg(feature = "kinesis")]
    router.register_service(service_com_amazonaws_kinesis_kinesis_20131202::EmulatedService::new(engine.clone()));
    #[cfg(feature = "kinesisanalytics")]
    router.register_service(service_com_amazonaws_kinesisanalytics_kinesisanalytics_20150814::EmulatedService::new(engine.clone()));
    #[cfg(feature = "kinesisanalyticsv2")]
    router.register_service(service_com_amazonaws_kinesisanalyticsv2_kinesisanalytics_20180523::EmulatedService::new(engine.clone()));
    #[cfg(feature = "kinesisvideo")]
    router.register_service(service_com_amazonaws_kinesisvideo_kinesisvideo_20170930::EmulatedService::new(engine.clone()));
    #[cfg(feature = "kinesisvideoarchivedmedia")]
    router.register_service(service_com_amazonaws_kinesisvideoarchivedmedia_awsacuityreader::EmulatedService::new(engine.clone()));
    router
}

/// Register services 20 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_20(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "kinesisvideomedia")]
    router.register_service(service_com_amazonaws_kinesisvideomedia_awsacuityinletservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "kinesisvideosignaling")]
    router.register_service(service_com_amazonaws_kinesisvideosignaling_awsacuitysignalingservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "kinesisvideowebrtcstorage")]
    router.register_service(service_com_amazonaws_kinesisvideowebrtcstorage_awsacuityroutingservicelambda::EmulatedService::new(engine.clone()));
    #[cfg(feature = "kms")]
    router.register_service(service_com_amazonaws_kms_trentservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "lakeformation")]
    router.register_service(service_com_amazonaws_lakeformation_awslakeformation::EmulatedService::new(engine.clone()));
    #[cfg(feature = "lambda")]
    router.register_service(service_com_amazonaws_lambda_awsgirapiservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "lambdacore")]
    router.register_service(service_com_amazonaws_lambdacore_lambdacoreapiservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "lambdamicrovms")]
    router.register_service(service_com_amazonaws_lambdamicrovms_lambdamicrovms::EmulatedService::new(engine.clone()));
    #[cfg(feature = "lambdaweb")]
    router.register_service(service_com_amazonaws_lambdaweb_lambdaweb::EmulatedService::new(engine.clone()));
    #[cfg(feature = "launchwizard")]
    router.register_service(service_com_amazonaws_launchwizard_launchwizard::EmulatedService::new(engine.clone()));
    #[cfg(feature = "lexmodelbuildingservice")]
    router.register_service(service_com_amazonaws_lexmodelbuildingservice_awsdeepsensemodelbuildingservice::EmulatedService::new(engine.clone()));
    router
}

/// Register services 21 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_21(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "lexmodelsv2")]
    router.register_service(service_com_amazonaws_lexmodelsv2_lexmodelbuildingservicev2::EmulatedService::new(engine.clone()));
    #[cfg(feature = "lexruntimeservice")]
    router.register_service(service_com_amazonaws_lexruntimeservice_awsdeepsenseruntimeservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "lexruntimev2")]
    router.register_service(service_com_amazonaws_lexruntimev2_awsdeepsenseruntimeserviceapi2_0::EmulatedService::new(engine.clone()));
    #[cfg(feature = "licensemanager")]
    router.register_service(service_com_amazonaws_licensemanager_awslicensemanager::EmulatedService::new(engine.clone()));
    #[cfg(feature = "licensemanagerlinuxsubscriptions")]
    router.register_service(service_com_amazonaws_licensemanagerlinuxsubscriptions_licensemanagerlinuxsubscriptions::EmulatedService::new(engine.clone()));
    #[cfg(feature = "licensemanagerusersubscriptions")]
    router.register_service(service_com_amazonaws_licensemanagerusersubscriptions_licensemanagerusersubscriptions::EmulatedService::new(engine.clone()));
    #[cfg(feature = "lightsail")]
    router.register_service(service_com_amazonaws_lightsail_lightsail_20161128::EmulatedService::new(engine.clone()));
    #[cfg(feature = "location")]
    router.register_service(service_com_amazonaws_location_locationservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "lookoutequipment")]
    router.register_service(service_com_amazonaws_lookoutequipment_awslookoutequipmentfrontendservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "m2")]
    router.register_service(service_com_amazonaws_m2_awssupernovacontrolplaneservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "machinelearning")]
    router.register_service(service_com_amazonaws_machinelearning_amazonml_20141212::EmulatedService::new(engine.clone()));
    router
}

/// Register services 22 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_22(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "macie2")]
    router.register_service(service_com_amazonaws_macie2_macie2::EmulatedService::new(engine.clone()));
    #[cfg(feature = "mailmanager")]
    router.register_service(service_com_amazonaws_mailmanager_mailmanagersvc::EmulatedService::new(engine.clone()));
    #[cfg(feature = "managedblockchain")]
    router.register_service(service_com_amazonaws_managedblockchain_taigawebservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "managedblockchainquery")]
    router.register_service(service_com_amazonaws_managedblockchainquery_tietonchainqueryservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "marketplaceagreement")]
    router.register_service(service_com_amazonaws_marketplaceagreement_awsmpcommerceservice_v20200301::EmulatedService::new(engine.clone()));
    #[cfg(feature = "marketplacecatalog")]
    router.register_service(service_com_amazonaws_marketplacecatalog_awsmpseymour::EmulatedService::new(engine.clone()));
    #[cfg(feature = "marketplacecommerceanalytics")]
    router.register_service(service_com_amazonaws_marketplacecommerceanalytics_marketplacecommerceanalytics20150701::EmulatedService::new(engine.clone()));
    #[cfg(feature = "marketplacedeployment")]
    router.register_service(service_com_amazonaws_marketplacedeployment_awsmpdeploymentparametersservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "marketplacediscovery")]
    router.register_service(service_com_amazonaws_marketplacediscovery_awsmarketplacediscovery::EmulatedService::new(engine.clone()));
    #[cfg(feature = "marketplaceentitlementservice")]
    router.register_service(service_com_amazonaws_marketplaceentitlementservice_awsmpentitlementservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "marketplacemetering")]
    router.register_service(service_com_amazonaws_marketplacemetering_awsmpmeteringservice::EmulatedService::new(engine.clone()));
    router
}

/// Register services 23 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_23(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "marketplacereporting")]
    router.register_service(service_com_amazonaws_marketplacereporting_awsmarketplacereporting::EmulatedService::new(engine.clone()));
    #[cfg(feature = "mediaconnect")]
    router.register_service(service_com_amazonaws_mediaconnect_mediaconnect::EmulatedService::new(engine.clone()));
    #[cfg(feature = "mediaconvert")]
    router.register_service(service_com_amazonaws_mediaconvert_mediaconvert::EmulatedService::new(engine.clone()));
    #[cfg(feature = "medialive")]
    router.register_service(service_com_amazonaws_medialive_medialive::EmulatedService::new(engine.clone()));
    #[cfg(feature = "mediapackage")]
    router.register_service(service_com_amazonaws_mediapackage_mediapackage::EmulatedService::new(engine.clone()));
    #[cfg(feature = "mediapackagev2")]
    router.register_service(service_com_amazonaws_mediapackagev2_mediapackagev2::EmulatedService::new(engine.clone()));
    #[cfg(feature = "mediapackagevod")]
    router.register_service(service_com_amazonaws_mediapackagevod_mediapackagevod::EmulatedService::new(engine.clone()));
    #[cfg(feature = "mediastore")]
    router.register_service(service_com_amazonaws_mediastore_mediastore_20170901::EmulatedService::new(engine.clone()));
    #[cfg(feature = "mediastoredata")]
    router.register_service(service_com_amazonaws_mediastoredata_mediastoreobject_20170901::EmulatedService::new(engine.clone()));
    #[cfg(feature = "mediatailor")]
    router.register_service(service_com_amazonaws_mediatailor_mediatailor::EmulatedService::new(engine.clone()));
    #[cfg(feature = "medicalimaging")]
    router.register_service(service_com_amazonaws_medicalimaging_ahigatewayservice::EmulatedService::new(engine.clone()));
    router
}

/// Register services 24 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_24(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "memorydb")]
    router.register_service(service_com_amazonaws_memorydb_amazonmemorydb::EmulatedService::new(engine.clone()));
    #[cfg(feature = "mgn")]
    router.register_service(service_com_amazonaws_mgn_applicationmigrationservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "migrationhub")]
    router.register_service(service_com_amazonaws_migrationhub_awsmigrationhub::EmulatedService::new(engine.clone()));
    #[cfg(feature = "migrationhubconfig")]
    router.register_service(service_com_amazonaws_migrationhubconfig_awsmigrationhubmultiaccountservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "migrationhuborchestrator")]
    router.register_service(service_com_amazonaws_migrationhuborchestrator_awsmigrationhuborchestrator::EmulatedService::new(engine.clone()));
    #[cfg(feature = "migrationhubrefactorspaces")]
    router.register_service(service_com_amazonaws_migrationhubrefactorspaces_refactorspaces::EmulatedService::new(engine.clone()));
    #[cfg(feature = "migrationhubstrategy")]
    router.register_service(service_com_amazonaws_migrationhubstrategy_awsmigrationhubstrategyrecommendation::EmulatedService::new(engine.clone()));
    #[cfg(feature = "mpa")]
    router.register_service(service_com_amazonaws_mpa_awsfluffycoreservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "mq")]
    router.register_service(service_com_amazonaws_mq_mq::EmulatedService::new(engine.clone()));
    #[cfg(feature = "mturk")]
    router.register_service(service_com_amazonaws_mturk_mturkrequesterservicev20170117::EmulatedService::new(engine.clone()));
    #[cfg(feature = "mwaa")]
    router.register_service(service_com_amazonaws_mwaa_amazonmwaa::EmulatedService::new(engine.clone()));
    router
}

/// Register services 25 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_25(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "mwaaserverless")]
    router.register_service(service_com_amazonaws_mwaaserverless_amazonmwaaserverless::EmulatedService::new(engine.clone()));
    #[cfg(feature = "neptune")]
    router.register_service(service_com_amazonaws_neptune_amazonrdsv19::EmulatedService::new(engine.clone()));
    #[cfg(feature = "neptunedata")]
    router.register_service(service_com_amazonaws_neptunedata_amazonneptunedataplane::EmulatedService::new(engine.clone()));
    #[cfg(feature = "neptunegraph")]
    router.register_service(service_com_amazonaws_neptunegraph_amazonneptunegraph::EmulatedService::new(engine.clone()));
    #[cfg(feature = "networkfirewall")]
    router.register_service(service_com_amazonaws_networkfirewall_networkfirewall_20201112::EmulatedService::new(engine.clone()));
    #[cfg(feature = "networkflowmonitor")]
    router.register_service(service_com_amazonaws_networkflowmonitor_networkflowmonitor::EmulatedService::new(engine.clone()));
    #[cfg(feature = "networkmanager")]
    router.register_service(service_com_amazonaws_networkmanager_networkmanager::EmulatedService::new(engine.clone()));
    #[cfg(feature = "networkmonitor")]
    router.register_service(service_com_amazonaws_networkmonitor_networkmonitor::EmulatedService::new(engine.clone()));
    #[cfg(feature = "networksecuritymanager")]
    router.register_service(service_com_amazonaws_networksecuritymanager_piccolocustomerapiservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "notifications")]
    router.register_service(service_com_amazonaws_notifications_notifications::EmulatedService::new(engine.clone()));
    #[cfg(feature = "notificationscontacts")]
    router.register_service(service_com_amazonaws_notificationscontacts_notificationscontacts::EmulatedService::new(engine.clone()));
    router
}

/// Register services 26 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_26(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "novaact")]
    router.register_service(service_com_amazonaws_novaact_amazonnovaagentsdataplane::EmulatedService::new(engine.clone()));
    #[cfg(feature = "oam")]
    router.register_service(service_com_amazonaws_oam_oamservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "observabilityadmin")]
    router.register_service(service_com_amazonaws_observabilityadmin_observabilityadmin::EmulatedService::new(engine.clone()));
    #[cfg(feature = "odb")]
    router.register_service(service_com_amazonaws_odb_odb::EmulatedService::new(engine.clone()));
    #[cfg(feature = "omics")]
    router.register_service(service_com_amazonaws_omics_omics::EmulatedService::new(engine.clone()));
    #[cfg(feature = "opensearch")]
    router.register_service(service_com_amazonaws_opensearch_amazonopensearchservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "opensearchserverless")]
    router.register_service(service_com_amazonaws_opensearchserverless_opensearchserverless::EmulatedService::new(engine.clone()));
    #[cfg(feature = "organizations")]
    router.register_service(service_com_amazonaws_organizations_awsorganizationsv20161128::EmulatedService::new(engine.clone()));
    #[cfg(feature = "osis")]
    router.register_service(service_com_amazonaws_osis_amazonopensearchingestionservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "outposts")]
    router.register_service(service_com_amazonaws_outposts_outpostsolafservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "partnercentralaccount")]
    router.register_service(service_com_amazonaws_partnercentralaccount_partnercentralaccount::EmulatedService::new(engine.clone()));
    router
}

/// Register services 27 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_27(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "partnercentralbenefits")]
    router.register_service(service_com_amazonaws_partnercentralbenefits_partnercentralbenefitsservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "partnercentralchannel")]
    router.register_service(service_com_amazonaws_partnercentralchannel_partnercentralchannel::EmulatedService::new(engine.clone()));
    #[cfg(feature = "partnercentralrevenuemeasurement")]
    router.register_service(service_com_amazonaws_partnercentralrevenuemeasurement_partnercentralrevenuemeasurement::EmulatedService::new(engine.clone()));
    #[cfg(feature = "partnercentralselling")]
    router.register_service(service_com_amazonaws_partnercentralselling_awspartnercentralselling::EmulatedService::new(engine.clone()));
    #[cfg(feature = "paymentcryptography")]
    router.register_service(service_com_amazonaws_paymentcryptography_paymentcryptographycontrolplane::EmulatedService::new(engine.clone()));
    #[cfg(feature = "paymentcryptographydata")]
    router.register_service(service_com_amazonaws_paymentcryptographydata_paymentcryptographydataplane::EmulatedService::new(engine.clone()));
    #[cfg(feature = "pcaconnectorad")]
    router.register_service(service_com_amazonaws_pcaconnectorad_pcaconnectorad::EmulatedService::new(engine.clone()));
    #[cfg(feature = "pcaconnectorscep")]
    router.register_service(service_com_amazonaws_pcaconnectorscep_pcaconnectorscep::EmulatedService::new(engine.clone()));
    #[cfg(feature = "pcs")]
    router.register_service(service_com_amazonaws_pcs_awsparallelcomputingservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "personalize")]
    router.register_service(service_com_amazonaws_personalize_amazonpersonalize::EmulatedService::new(engine.clone()));
    #[cfg(feature = "personalizeevents")]
    router.register_service(service_com_amazonaws_personalizeevents_amazonpersonalizeevents::EmulatedService::new(engine.clone()));
    router
}

/// Register services 28 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_28(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "personalizeruntime")]
    router.register_service(service_com_amazonaws_personalizeruntime_amazonpersonalizeruntime::EmulatedService::new(engine.clone()));
    #[cfg(feature = "pi")]
    router.register_service(service_com_amazonaws_pi_performanceinsightsv20180227::EmulatedService::new(engine.clone()));
    #[cfg(feature = "pinpoint")]
    router.register_service(service_com_amazonaws_pinpoint_pinpoint::EmulatedService::new(engine.clone()));
    #[cfg(feature = "pinpointemail")]
    router.register_service(service_com_amazonaws_pinpointemail_amazonpinpointemailservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "pinpointsmsvoice")]
    router.register_service(service_com_amazonaws_pinpointsmsvoice_pinpointsmsvoice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "pinpointsmsvoicev2")]
    router.register_service(service_com_amazonaws_pinpointsmsvoicev2_pinpointsmsvoicev2::EmulatedService::new(engine.clone()));
    #[cfg(feature = "pipes")]
    router.register_service(service_com_amazonaws_pipes_pipes::EmulatedService::new(engine.clone()));
    #[cfg(feature = "polly")]
    router.register_service(service_com_amazonaws_polly_parrot_v1::EmulatedService::new(engine.clone()));
    #[cfg(feature = "pricing")]
    router.register_service(service_com_amazonaws_pricing_awspricelistservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "pricingplanmanager")]
    router.register_service(service_com_amazonaws_pricingplanmanager_awspricingplanmanager::EmulatedService::new(engine.clone()));
    #[cfg(feature = "proton")]
    router.register_service(service_com_amazonaws_proton_awsproton20200720::EmulatedService::new(engine.clone()));
    router
}

/// Register services 29 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_29(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "qapps")]
    router.register_service(service_com_amazonaws_qapps_qappsservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "qbusiness")]
    router.register_service(service_com_amazonaws_qbusiness_expertq::EmulatedService::new(engine.clone()));
    #[cfg(feature = "qconnect")]
    router.register_service(service_com_amazonaws_qconnect_wisdomservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "quicksight")]
    router.register_service(service_com_amazonaws_quicksight_quicksight_20180401::EmulatedService::new(engine.clone()));
    #[cfg(feature = "ram")]
    router.register_service(service_com_amazonaws_ram_amazonresourcesharing::EmulatedService::new(engine.clone()));
    #[cfg(feature = "rbin")]
    router.register_service(service_com_amazonaws_rbin_amazonrecyclebin::EmulatedService::new(engine.clone()));
    #[cfg(feature = "rds")]
    router.register_service(service_com_amazonaws_rds_amazonrdsv19::EmulatedService::new(engine.clone()));
    #[cfg(feature = "rdsdata")]
    router.register_service(service_com_amazonaws_rdsdata_rdsdataservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "redshift")]
    router.register_service(service_com_amazonaws_redshift_redshiftserviceversion20121201::EmulatedService::new(engine.clone()));
    #[cfg(feature = "redshiftdata")]
    router.register_service(service_com_amazonaws_redshiftdata_redshiftdata::EmulatedService::new(engine.clone()));
    #[cfg(feature = "redshiftserverless")]
    router.register_service(service_com_amazonaws_redshiftserverless_redshiftserverless::EmulatedService::new(engine.clone()));
    router
}

/// Register services 30 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_30(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "rekognition")]
    router.register_service(service_com_amazonaws_rekognition_rekognitionservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "repostspace")]
    router.register_service(service_com_amazonaws_repostspace_repostspace::EmulatedService::new(engine.clone()));
    #[cfg(feature = "resiliencehub")]
    router.register_service(service_com_amazonaws_resiliencehub_awsresiliencehub::EmulatedService::new(engine.clone()));
    #[cfg(feature = "resiliencehubv2")]
    router.register_service(service_com_amazonaws_resiliencehubv2_ngrhservicecore::EmulatedService::new(engine.clone()));
    #[cfg(feature = "resourceexplorer2")]
    router.register_service(service_com_amazonaws_resourceexplorer2_resourceexplorer::EmulatedService::new(engine.clone()));
    #[cfg(feature = "resourcegroups")]
    router.register_service(service_com_amazonaws_resourcegroups_ardi::EmulatedService::new(engine.clone()));
    #[cfg(feature = "resourcegroupstaggingapi")]
    router.register_service(service_com_amazonaws_resourcegroupstaggingapi_resourcegroupstaggingapi_20170126::EmulatedService::new(engine.clone()));
    #[cfg(feature = "rolesanywhere")]
    router.register_service(service_com_amazonaws_rolesanywhere_rolesanywhere::EmulatedService::new(engine.clone()));
    #[cfg(feature = "route53")]
    router.register_service(service_com_amazonaws_route53_awsdnsv20130401::EmulatedService::new(engine.clone()));
    #[cfg(feature = "route53domains")]
    router.register_service(service_com_amazonaws_route53domains_route53domains_v20140515::EmulatedService::new(engine.clone()));
    #[cfg(feature = "route53globalresolver")]
    router.register_service(service_com_amazonaws_route53globalresolver_ec2dnsglobalresolvercustomerapi::EmulatedService::new(engine.clone()));
    router
}

/// Register services 31 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_31(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "route53profiles")]
    router.register_service(service_com_amazonaws_route53profiles_route53profiles::EmulatedService::new(engine.clone()));
    #[cfg(feature = "route53recoverycluster")]
    router.register_service(service_com_amazonaws_route53recoverycluster_togglecustomerapi::EmulatedService::new(engine.clone()));
    #[cfg(feature = "route53recoverycontrolconfig")]
    router.register_service(service_com_amazonaws_route53recoverycontrolconfig_route53recoverycontrolconfig::EmulatedService::new(engine.clone()));
    #[cfg(feature = "route53recoveryreadiness")]
    router.register_service(service_com_amazonaws_route53recoveryreadiness_route53recoveryreadiness::EmulatedService::new(engine.clone()));
    #[cfg(feature = "route53resolver")]
    router.register_service(service_com_amazonaws_route53resolver_route53resolver::EmulatedService::new(engine.clone()));
    #[cfg(feature = "rtbfabric")]
    router.register_service(service_com_amazonaws_rtbfabric_rtbfabric::EmulatedService::new(engine.clone()));
    #[cfg(feature = "rum")]
    router.register_service(service_com_amazonaws_rum_rum::EmulatedService::new(engine.clone()));
    #[cfg(feature = "s3")]
    router.register_service(service_com_amazonaws_s3_amazons3::EmulatedService::new(engine.clone()));
    #[cfg(feature = "s3control")]
    router.register_service(service_com_amazonaws_s3control_awss3controlservicev20180820::EmulatedService::new(engine.clone()));
    #[cfg(feature = "s3files")]
    router.register_service(service_com_amazonaws_s3files_s3files::EmulatedService::new(engine.clone()));
    #[cfg(feature = "s3outposts")]
    router.register_service(service_com_amazonaws_s3outposts_s3outposts::EmulatedService::new(engine.clone()));
    router
}

/// Register services 32 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_32(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "s3tables")]
    router.register_service(service_com_amazonaws_s3tables_s3tablebuckets::EmulatedService::new(engine.clone()));
    #[cfg(feature = "s3vectors")]
    router.register_service(service_com_amazonaws_s3vectors_s3vectors::EmulatedService::new(engine.clone()));
    #[cfg(feature = "sagemaker")]
    router.register_service(service_com_amazonaws_sagemaker_sagemaker::EmulatedService::new(engine.clone()));
    #[cfg(feature = "sagemakera2iruntime")]
    router.register_service(service_com_amazonaws_sagemakera2iruntime_amazonsagemakera2iruntime::EmulatedService::new(engine.clone()));
    #[cfg(feature = "sagemakeredge")]
    router.register_service(service_com_amazonaws_sagemakeredge_amazonsagemakeredge::EmulatedService::new(engine.clone()));
    #[cfg(feature = "sagemakerfeaturestoreruntime")]
    router.register_service(service_com_amazonaws_sagemakerfeaturestoreruntime_amazonsagemakerfeaturestoreruntime::EmulatedService::new(engine.clone()));
    #[cfg(feature = "sagemakergeospatial")]
    router.register_service(service_com_amazonaws_sagemakergeospatial_sagemakergeospatial::EmulatedService::new(engine.clone()));
    #[cfg(feature = "sagemakerjobruntime")]
    router.register_service(service_com_amazonaws_sagemakerjobruntime_agenticrftruntimeservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "sagemakermetrics")]
    router.register_service(service_com_amazonaws_sagemakermetrics_sagemakermetricsservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "sagemakerruntime")]
    router.register_service(service_com_amazonaws_sagemakerruntime_amazonsagemakerruntime::EmulatedService::new(engine.clone()));
    #[cfg(feature = "sagemakerruntimehttp2")]
    router.register_service(service_com_amazonaws_sagemakerruntimehttp2_amazonsagemakerruntimehttp2::EmulatedService::new(engine.clone()));
    router
}

/// Register services 33 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_33(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "sagemakertrainingsessionruntime")]
    router.register_service(service_com_amazonaws_sagemakertrainingsessionruntime_legorouterservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "savingsplans")]
    router.register_service(service_com_amazonaws_savingsplans_awssavingsplan::EmulatedService::new(engine.clone()));
    #[cfg(feature = "scheduler")]
    router.register_service(service_com_amazonaws_scheduler_awschronosservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "schemas")]
    router.register_service(service_com_amazonaws_schemas_schemas::EmulatedService::new(engine.clone()));
    #[cfg(feature = "secretsmanager")]
    router.register_service(service_com_amazonaws_secretsmanager_secretsmanager::EmulatedService::new(engine.clone()));
    #[cfg(feature = "securityagent")]
    router.register_service(service_com_amazonaws_securityagent_securityagent::EmulatedService::new(engine.clone()));
    #[cfg(feature = "securityhub")]
    router.register_service(service_com_amazonaws_securityhub_securityhubapiservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "securityir")]
    router.register_service(service_com_amazonaws_securityir_securityincidentresponse::EmulatedService::new(engine.clone()));
    #[cfg(feature = "securitylake")]
    router.register_service(service_com_amazonaws_securitylake_securitylake::EmulatedService::new(engine.clone()));
    #[cfg(feature = "serverlessapplicationrepository")]
    router.register_service(service_com_amazonaws_serverlessapplicationrepository_serverlessapplicationrepository::EmulatedService::new(engine.clone()));
    #[cfg(feature = "servicecatalog")]
    router.register_service(service_com_amazonaws_servicecatalog_aws242servicecatalogservice::EmulatedService::new(engine.clone()));
    router
}

/// Register services 34 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_34(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "servicecatalogappregistry")]
    router.register_service(service_com_amazonaws_servicecatalogappregistry_aws242appregistry::EmulatedService::new(engine.clone()));
    #[cfg(feature = "servicediscovery")]
    router.register_service(service_com_amazonaws_servicediscovery_route53autonaming_v20170314::EmulatedService::new(engine.clone()));
    #[cfg(feature = "servicequotas")]
    router.register_service(service_com_amazonaws_servicequotas_servicequotasv20190624::EmulatedService::new(engine.clone()));
    #[cfg(feature = "ses")]
    router.register_service(service_com_amazonaws_ses_simpleemailservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "sesv2")]
    router.register_service(service_com_amazonaws_sesv2_simpleemailservice_v2::EmulatedService::new(engine.clone()));
    #[cfg(feature = "sfn")]
    router.register_service(service_com_amazonaws_sfn_awsstepfunctions::EmulatedService::new(engine.clone()));
    #[cfg(feature = "shield")]
    router.register_service(service_com_amazonaws_shield_awsshield_20160616::EmulatedService::new(engine.clone()));
    #[cfg(feature = "signer")]
    router.register_service(service_com_amazonaws_signer_wallabyservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "signerdata")]
    router.register_service(service_com_amazonaws_signerdata_signerdataplane::EmulatedService::new(engine.clone()));
    #[cfg(feature = "signin")]
    router.register_service(service_com_amazonaws_signin_signin::EmulatedService::new(engine.clone()));
    #[cfg(feature = "simpledbv2")]
    router.register_service(service_com_amazonaws_simpledbv2_simpledbv2::EmulatedService::new(engine.clone()));
    router
}

/// Register services 35 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_35(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "snowball")]
    router.register_service(service_com_amazonaws_snowball_awsiesnowballjobmanagementservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "snowdevicemanagement")]
    router.register_service(service_com_amazonaws_snowdevicemanagement_snowdevicemanagement::EmulatedService::new(engine.clone()));
    #[cfg(feature = "sns")]
    router.register_service(service_com_amazonaws_sns_amazonsimplenotificationservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "socialmessaging")]
    router.register_service(service_com_amazonaws_socialmessaging_socialmessaging::EmulatedService::new(engine.clone()));
    #[cfg(feature = "sqs")]
    router.register_service(service_com_amazonaws_sqs_amazonsqs::EmulatedService::new(engine.clone()));
    #[cfg(feature = "ssm")]
    router.register_service(service_com_amazonaws_ssm_amazonssm::EmulatedService::new(engine.clone()));
    #[cfg(feature = "ssmcontacts")]
    router.register_service(service_com_amazonaws_ssmcontacts_ssmcontacts::EmulatedService::new(engine.clone()));
    #[cfg(feature = "ssmguiconnect")]
    router.register_service(service_com_amazonaws_ssmguiconnect_ssmguiconnect::EmulatedService::new(engine.clone()));
    #[cfg(feature = "ssmincidents")]
    router.register_service(service_com_amazonaws_ssmincidents_ssmincidents::EmulatedService::new(engine.clone()));
    #[cfg(feature = "ssmquicksetup")]
    router.register_service(service_com_amazonaws_ssmquicksetup_quicksetup::EmulatedService::new(engine.clone()));
    #[cfg(feature = "ssmsap")]
    router.register_service(service_com_amazonaws_ssmsap_ssmsap::EmulatedService::new(engine.clone()));
    router
}

/// Register services 36 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_36(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "sso")]
    router.register_service(service_com_amazonaws_sso_swbportalservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "ssoadmin")]
    router.register_service(service_com_amazonaws_ssoadmin_swbexternalservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "ssooidc")]
    router.register_service(service_com_amazonaws_ssooidc_awsssooidcservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "storagegateway")]
    router.register_service(service_com_amazonaws_storagegateway_storagegateway_20130630::EmulatedService::new(engine.clone()));
    #[cfg(feature = "sts")]
    router.register_service(service_com_amazonaws_sts_awssecuritytokenservicev20110615::EmulatedService::new(engine.clone()));
    #[cfg(feature = "supplychain")]
    router.register_service(service_com_amazonaws_supplychain_galaxypublicapigateway::EmulatedService::new(engine.clone()));
    #[cfg(feature = "support")]
    router.register_service(service_com_amazonaws_support_awssupport_20130415::EmulatedService::new(engine.clone()));
    #[cfg(feature = "supportapp")]
    router.register_service(service_com_amazonaws_supportapp_supportapp::EmulatedService::new(engine.clone()));
    #[cfg(feature = "supportauthz")]
    router.register_service(service_com_amazonaws_supportauthz_supportauthz::EmulatedService::new(engine.clone()));
    #[cfg(feature = "sustainability")]
    router.register_service(service_com_amazonaws_sustainability_awssustainabilityapiservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "swf")]
    router.register_service(service_com_amazonaws_swf_simpleworkflowservice::EmulatedService::new(engine.clone()));
    router
}

/// Register services 37 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_37(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "synthetics")]
    router.register_service(service_com_amazonaws_synthetics_synthetics::EmulatedService::new(engine.clone()));
    #[cfg(feature = "taxsettings")]
    router.register_service(service_com_amazonaws_taxsettings_taxsettings::EmulatedService::new(engine.clone()));
    #[cfg(feature = "textract")]
    router.register_service(service_com_amazonaws_textract_textract::EmulatedService::new(engine.clone()));
    #[cfg(feature = "timestreaminfluxdb")]
    router.register_service(service_com_amazonaws_timestreaminfluxdb_amazontimestreaminfluxdb::EmulatedService::new(engine.clone()));
    #[cfg(feature = "timestreamquery")]
    router.register_service(service_com_amazonaws_timestreamquery_timestream_20181101::EmulatedService::new(engine.clone()));
    #[cfg(feature = "timestreamwrite")]
    router.register_service(service_com_amazonaws_timestreamwrite_timestream_20181101::EmulatedService::new(engine.clone()));
    #[cfg(feature = "transcribe")]
    router.register_service(service_com_amazonaws_transcribe_transcribe::EmulatedService::new(engine.clone()));
    #[cfg(feature = "transcribestreaming")]
    router.register_service(service_com_amazonaws_transcribestreaming_transcribe::EmulatedService::new(engine.clone()));
    #[cfg(feature = "transfer")]
    router.register_service(service_com_amazonaws_transfer_transferservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "translate")]
    router.register_service(service_com_amazonaws_translate_awsshinefrontendservice_20170701::EmulatedService::new(engine.clone()));
    #[cfg(feature = "trustedadvisor")]
    router.register_service(service_com_amazonaws_trustedadvisor_trustedadvisor::EmulatedService::new(engine.clone()));
    router
}

/// Register services 38 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_38(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "uxc")]
    router.register_service(service_com_amazonaws_uxc_awsaccountuxsetting::EmulatedService::new(engine.clone()));
    #[cfg(feature = "verifiedpermissions")]
    router.register_service(service_com_amazonaws_verifiedpermissions_verifiedpermissions::EmulatedService::new(engine.clone()));
    #[cfg(feature = "voiceid")]
    router.register_service(service_com_amazonaws_voiceid_voiceid::EmulatedService::new(engine.clone()));
    #[cfg(feature = "vpclattice")]
    router.register_service(service_com_amazonaws_vpclattice_mercurycontrolplane::EmulatedService::new(engine.clone()));
    #[cfg(feature = "waf")]
    router.register_service(service_com_amazonaws_waf_awswaf_20150824::EmulatedService::new(engine.clone()));
    #[cfg(feature = "wafregional")]
    router.register_service(service_com_amazonaws_wafregional_awswaf_regional_20161128::EmulatedService::new(engine.clone()));
    #[cfg(feature = "wafv2")]
    router.register_service(service_com_amazonaws_wafv2_awswaf_20190729::EmulatedService::new(engine.clone()));
    #[cfg(feature = "wellarchitected")]
    router.register_service(service_com_amazonaws_wellarchitected_wellarchitectedapiservicelambda::EmulatedService::new(engine.clone()));
    #[cfg(feature = "wickr")]
    router.register_service(service_com_amazonaws_wickr_wickradminapi::EmulatedService::new(engine.clone()));
    #[cfg(feature = "wisdom")]
    router.register_service(service_com_amazonaws_wisdom_wisdomservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "workdocs")]
    router.register_service(service_com_amazonaws_workdocs_awsgorillaboyservice::EmulatedService::new(engine.clone()));
    router
}

/// Register services 39 of this workspace on `router`.
///
/// `#[inline(never)]` is LOAD-BEARING: inlined back into `build_router` the frames
/// would coalesce again and the stack saving would be lost.
#[inline(never)]
#[allow(unused_mut, unused_variables)]
fn mount_chunk_39(
    mut router: MultiServiceRouter,
    engine: &Arc<InMemoryEngine>,
) -> MultiServiceRouter {
    #[cfg(feature = "workmail")]
    router.register_service(service_com_amazonaws_workmail_workmailservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "workmailmessageflow")]
    router.register_service(service_com_amazonaws_workmailmessageflow_giraffemessageintransitservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "workspaces")]
    router.register_service(service_com_amazonaws_workspaces_workspacesservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "workspacesinstances")]
    router.register_service(service_com_amazonaws_workspacesinstances_eucmifrontendapiservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "workspacesthinclient")]
    router.register_service(service_com_amazonaws_workspacesthinclient_thinclient::EmulatedService::new(engine.clone()));
    #[cfg(feature = "workspacesweb")]
    router.register_service(service_com_amazonaws_workspacesweb_awserminecontrolplaneservice::EmulatedService::new(engine.clone()));
    #[cfg(feature = "xray")]
    router.register_service(service_com_amazonaws_xray_awsxray::EmulatedService::new(engine.clone()));
    router
}

