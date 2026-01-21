use slint::{ModelRc, SharedString, VecModel};

slint::include_modules!();

#[derive(Clone, Default)]
struct QueryResult {
    column1: SharedString,
    column2: SharedString,
    column3: SharedString,
}

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;

    app.set_host("db.company.local".into());
    app.set_port("1521".into());
    app.set_service("ORCL".into());
    app.set_user("system".into());
    app.set_query("select * from employees where status = 'ACTIVE';".into());
    app.set_status("Ready".into());

    let results_model = std::rc::Rc::new(VecModel::from(vec![
        QueryResult {
            column1: "EMP_ID".into(),
            column2: "NAME".into(),
            column3: "STATUS".into(),
        },
    ]));
    app.set_results(ModelRc::from(results_model.clone()));

    let app_handle = app.as_weak();
    app.on_run_query(move || {
        let Some(app) = app_handle.upgrade() else {
            return;
        };

        let query = app.get_query();
        let mut mock_rows = vec![];
        for idx in 0..6 {
            mock_rows.push(QueryResult {
                column1: format!("EMP-{}", 1000 + idx).into(),
                column2: format!("User {:02}", idx + 1).into(),
                column3: "ACTIVE".into(),
            });
        }

        results_model.set_vec(mock_rows);
        app.set_status(SharedString::from(format!("Ran query (mock): {}", query)));
    });

    app.run()
}
