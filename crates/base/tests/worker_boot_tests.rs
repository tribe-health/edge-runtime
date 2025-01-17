use std::collections::HashMap;

use base::worker_ctx::create_worker;
use sb_worker_context::essentials::{EdgeContextInitOpts, EdgeContextOpts, EdgeUserRuntimeOpts};

#[tokio::test]
async fn test_worker_boot_invalid_imports() {
    let user_rt_opts = EdgeUserRuntimeOpts::default();
    let opts = EdgeContextInitOpts {
        service_path: "./test_cases/invalid_imports".into(),
        no_module_cache: false,
        import_map_path: None,
        env_vars: HashMap::new(),
        conf: EdgeContextOpts::UserWorker(user_rt_opts),
    };
    let result = create_worker(opts, None).await;

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "worker boot error");
}
