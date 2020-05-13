use cargo_up::{
    ra_hir::Semantics, ra_ide_db::RootDatabase, ra_syntax::ast, Runner, Upgrader, Version,
};

pub fn runner() -> Runner {
    Runner::new()
        .minimum("2.33.0")
        .unwrap()
        .peers(&["structopt"])
        .version(
            Version::new("3.0.0-beta.1")
                .unwrap()
                .rename_methods(
                    "clap::app::App",
                    &[
                        ["from_yaml", "from"],
                        ["arg_from_usage", "arg"],
                        ["help", "override_help"],
                        ["usage", "override_usage"],
                        ["template", "help_template"],
                        ["get_matches_safe", "try_get_matches"],
                        ["get_matches_from_safe", "try_get_matches_from"],
                        ["get_matches_from_safe_borrow", "try_get_matches_from_mut"],
                    ],
                )
                .rename_methods(
                    "clap::args::arg::Arg",
                    &[
                        ["help", "about"],
                        ["from_usage", "from"],
                        ["set", "setting"],
                        ["unset", "unset_setting"],
                    ],
                )
                .hook_method_call_expr(&print_method_calls),
        )
        .version(
            Version::new("3.0.0-rc.0")
                .unwrap()
                .rename_methods(
                    "clap::build::arg::Arg",
                    &[["from_yaml", "from"], ["with_name", "new"]],
                )
                .rename_methods("clap::build::arg_group::ArgGroup", &[["from_yaml", "from"]]),
        )
}

// #[up("3.0.0-beta.1")]
// replace_dep!("structopt", "clap", features = ["derive"]);

fn print_method_calls(
    upgrader: &mut Upgrader,
    method_call_expr: &ast::MethodCallExpr,
    semantics: &Semantics<RootDatabase>,
) {
    if let Some(name_ref) = method_call_expr.name_ref() {
        println!("method: {}", name_ref.text());
    }
}
