use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let version = if version.is_empty() {
        "latest".into()
    } else {
        format!("{}", version)
    };

    let stdout = dag()
        .pkgx()?
        .with_exec(vec![
            "pkgx",
            "install",
            &format!("scala-sbt.org@{}", version),
        ])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn compile(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "sbt", "compile", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn clean(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "sbt", "clean", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn doc(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "sbt", "doc", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn package(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "sbt", "package", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn package_bin(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "sbt", "packageBin", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn package_src(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "sbt", "packageSrc", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn publish(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "sbt", "publish", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn publish_local(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "sbt", "publishLocal", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn publish_m2(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "sbt", "publishM2", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn test(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "sbt", "test", &args])?
        .stdout()?;
    Ok(stdout)
}
