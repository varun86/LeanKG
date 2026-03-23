use leankg::db::models::BusinessLogic;
use leankg::db::schema::init_db;
use leankg::db::{
    self, FeatureTraceEntry, FeatureTraceability, UserStoryTraceEntry, UserStoryTraceability,
};
use tempfile::TempDir;

#[tokio::test]
async fn test_create_and_get_business_logic() {
    let tmp = TempDir::new().unwrap();
    let db_path = tmp.path().join("leankg.db");
    let db = init_db(db_path.as_path()).await.unwrap();

    let bl = db::create_business_logic(
        &db,
        "src/main.rs::main",
        "Main entry point",
        Some("US-01"),
        Some("FEAT-01"),
    )
    .await
    .unwrap();

    assert_eq!(bl.element_qualified, "src/main.rs::main");
    assert_eq!(bl.description, "Main entry point");
    assert_eq!(bl.user_story_id, Some("US-01".to_string()));
    assert_eq!(bl.feature_id, Some("FEAT-01".to_string()));

    let retrieved = db::get_business_logic(&db, "src/main.rs::main")
        .await
        .unwrap();
    assert!(retrieved.is_some());
    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.description, "Main entry point");
}

#[tokio::test]
async fn test_update_business_logic() {
    let tmp = TempDir::new().unwrap();
    let db_path = tmp.path().join("leankg.db");
    let db = init_db(db_path.as_path()).await.unwrap();

    db::create_business_logic(&db, "src/main.rs::main", "Original description", None, None)
        .await
        .unwrap();

    let updated = db::update_business_logic(
        &db,
        "src/main.rs::main",
        "Updated description",
        Some("US-02"),
        Some("FEAT-02"),
    )
    .await
    .unwrap();

    assert!(updated.is_some());
    let updated = updated.unwrap();
    assert_eq!(updated.description, "Updated description");
    assert_eq!(updated.user_story_id, Some("US-02".to_string()));
    assert_eq!(updated.feature_id, Some("FEAT-02".to_string()));
}

#[tokio::test]
async fn test_delete_business_logic() {
    let tmp = TempDir::new().unwrap();
    let db_path = tmp.path().join("leankg.db");
    let db = init_db(db_path.as_path()).await.unwrap();

    db::create_business_logic(&db, "src/main.rs::main", "To be deleted", None, None)
        .await
        .unwrap();

    db::delete_business_logic(&db, "src/main.rs::main")
        .await
        .unwrap();

    let retrieved = db::get_business_logic(&db, "src/main.rs::main")
        .await
        .unwrap();
    assert!(retrieved.is_none());
}

#[tokio::test]
async fn test_get_by_user_story() {
    let tmp = TempDir::new().unwrap();
    let db_path = tmp.path().join("leankg.db");
    let db = init_db(db_path.as_path()).await.unwrap();

    db::create_business_logic(&db, "src/auth.rs::login", "User login", Some("US-01"), None)
        .await
        .unwrap();
    db::create_business_logic(
        &db,
        "src/auth.rs::logout",
        "User logout",
        Some("US-01"),
        None,
    )
    .await
    .unwrap();
    db::create_business_logic(&db, "src/main.rs::main", "Main", Some("US-02"), None)
        .await
        .unwrap();

    let results = db::get_by_user_story(&db, "US-01").await.unwrap();
    assert_eq!(results.len(), 2);
}

#[tokio::test]
async fn test_get_by_feature() {
    let tmp = TempDir::new().unwrap();
    let db_path = tmp.path().join("leankg.db");
    let db = init_db(db_path.as_path()).await.unwrap();

    db::create_business_logic(
        &db,
        "src/auth.rs::login",
        "User login",
        None,
        Some("FEAT-AUTH"),
    )
    .await
    .unwrap();
    db::create_business_logic(
        &db,
        "src/auth.rs::logout",
        "User logout",
        None,
        Some("FEAT-AUTH"),
    )
    .await
    .unwrap();
    db::create_business_logic(&db, "src/main.rs::main", "Main", None, Some("FEAT-MAIN"))
        .await
        .unwrap();

    let results = db::get_by_feature(&db, "FEAT-AUTH").await.unwrap();
    assert_eq!(results.len(), 2);
}

#[tokio::test]
async fn test_search_business_logic() {
    let tmp = TempDir::new().unwrap();
    let db_path = tmp.path().join("leankg.db");
    let db = init_db(db_path.as_path()).await.unwrap();

    db::create_business_logic(
        &db,
        "src/auth.rs::login",
        "Handles user authentication",
        None,
        None,
    )
    .await
    .unwrap();
    db::create_business_logic(&db, "src/main.rs::main", "Main entry point", None, None)
        .await
        .unwrap();
    db::create_business_logic(
        &db,
        "src/validation.rs::validate",
        "Input validation logic",
        None,
        None,
    )
    .await
    .unwrap();

    let results = db::search_business_logic(&db, "authentication")
        .await
        .unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].element_qualified, "src/auth.rs::login");

    let results = db::search_business_logic(&db, "validation").await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].element_qualified, "src/validation.rs::validate");
}

#[tokio::test]
async fn test_get_feature_traceability() {
    let tmp = TempDir::new().unwrap();
    let db_path = tmp.path().join("leankg.db");
    let db = init_db(db_path.as_path()).await.unwrap();

    db::create_business_logic(
        &db,
        "src/auth.rs::login",
        "User login",
        Some("US-01"),
        Some("FEAT-AUTH"),
    )
    .await
    .unwrap();
    db::create_business_logic(
        &db,
        "src/auth.rs::logout",
        "User logout",
        Some("US-02"),
        Some("FEAT-AUTH"),
    )
    .await
    .unwrap();

    let trace = db::get_feature_traceability(&db, "FEAT-AUTH")
        .await
        .unwrap();
    assert_eq!(trace.feature_id, "FEAT-AUTH");
    assert_eq!(trace.code_elements.len(), 2);
}

#[tokio::test]
async fn test_get_user_story_traceability() {
    let tmp = TempDir::new().unwrap();
    let db_path = tmp.path().join("leankg.db");
    let db = init_db(db_path.as_path()).await.unwrap();

    db::create_business_logic(
        &db,
        "src/auth.rs::login",
        "User login",
        Some("US-01"),
        Some("FEAT-AUTH"),
    )
    .await
    .unwrap();
    db::create_business_logic(
        &db,
        "src/dashboard.rs::show",
        "Dashboard view",
        Some("US-01"),
        Some("FEAT-DASH"),
    )
    .await
    .unwrap();

    let trace = db::get_user_story_traceability(&db, "US-01").await.unwrap();
    assert_eq!(trace.user_story_id, "US-01");
    assert_eq!(trace.code_elements.len(), 2);
}

#[tokio::test]
async fn test_all_feature_traceability() {
    let tmp = TempDir::new().unwrap();
    let db_path = tmp.path().join("leankg.db");
    let db = init_db(db_path.as_path()).await.unwrap();

    db::create_business_logic(
        &db,
        "src/auth.rs::login",
        "User login",
        None,
        Some("FEAT-AUTH"),
    )
    .await
    .unwrap();
    db::create_business_logic(
        &db,
        "src/auth.rs::logout",
        "User logout",
        None,
        Some("FEAT-AUTH"),
    )
    .await
    .unwrap();
    db::create_business_logic(&db, "src/main.rs::main", "Main", None, Some("FEAT-MAIN"))
        .await
        .unwrap();

    let traces = db::all_feature_traceability(&db).await.unwrap();
    assert_eq!(traces.len(), 2);

    let auth_trace = traces.iter().find(|t| t.feature_id == "FEAT-AUTH").unwrap();
    assert_eq!(auth_trace.count, 2);

    let main_trace = traces.iter().find(|t| t.feature_id == "FEAT-MAIN").unwrap();
    assert_eq!(main_trace.count, 1);
}

#[tokio::test]
async fn test_all_user_story_traceability() {
    let tmp = TempDir::new().unwrap();
    let db_path = tmp.path().join("leankg.db");
    let db = init_db(db_path.as_path()).await.unwrap();

    db::create_business_logic(&db, "src/auth.rs::login", "Login", Some("US-01"), None)
        .await
        .unwrap();
    db::create_business_logic(&db, "src/main.rs::main", "Main", Some("US-02"), None)
        .await
        .unwrap();
    db::create_business_logic(
        &db,
        "src/dashboard.rs::show",
        "Dashboard",
        Some("US-02"),
        None,
    )
    .await
    .unwrap();

    let traces = db::all_user_story_traceability(&db).await.unwrap();
    assert_eq!(traces.len(), 2);

    let us01_trace = traces.iter().find(|t| t.user_story_id == "US-01").unwrap();
    assert_eq!(us01_trace.count, 1);

    let us02_trace = traces.iter().find(|t| t.user_story_id == "US-02").unwrap();
    assert_eq!(us02_trace.count, 2);
}

#[tokio::test]
async fn test_find_by_business_domain() {
    let tmp = TempDir::new().unwrap();
    let db_path = tmp.path().join("leankg.db");
    let db = init_db(db_path.as_path()).await.unwrap();

    db::create_business_logic(
        &db,
        "src/auth.rs::login",
        "Handles user authentication via OAuth",
        None,
        None,
    )
    .await
    .unwrap();
    db::create_business_logic(
        &db,
        "src/validation.rs::validate_email",
        "Email validation logic",
        None,
        None,
    )
    .await
    .unwrap();
    db::create_business_logic(
        &db,
        "src/auth.rs::check_perms",
        "Authentication and authorization",
        None,
        None,
    )
    .await
    .unwrap();

    let results = db::find_by_business_domain(&db, "authentication")
        .await
        .unwrap();
    assert_eq!(results.len(), 2);

    let results = db::find_by_business_domain(&db, "validation")
        .await
        .unwrap();
    assert_eq!(results.len(), 1);

    let results = db::find_by_business_domain(&db, "authorization")
        .await
        .unwrap();
    assert_eq!(results.len(), 1);
}

#[tokio::test]
async fn test_all_business_logic() {
    let tmp = TempDir::new().unwrap();
    let db_path = tmp.path().join("leankg.db");
    let db = init_db(db_path.as_path()).await.unwrap();

    db::create_business_logic(&db, "src/a.rs::func", "Function A", None, None)
        .await
        .unwrap();
    db::create_business_logic(&db, "src/b.rs::func", "Function B", None, None)
        .await
        .unwrap();

    let all = db::all_business_logic(&db).await.unwrap();
    assert_eq!(all.len(), 2);
}
