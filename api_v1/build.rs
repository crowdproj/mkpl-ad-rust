use futures_util::StreamExt;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::copy;
// use std::process::{Command, Stdio};
use std::path::Path;
use std::process::Command;

struct BuildCtx {
    oapi_dir: OsString,
    jar_url: String,
    jar_path: PathBuf,
    spec_path: OsString,
    template_dir: OsString,
    or_ignore_path: OsString,
    tg_ignore_path: OsString,
}

#[tokio::main]
async fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=./.generated/");
    println!("cargo:rerun-if-changed=./.openapi-generator-ignore");

    // std::env::vars().for_each(
    //     |f| println!("cargo:warning=ENV: {}={}", f.0, f.1)
    // );

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let bas_dir = std::env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let temp_oapi_dir = std::path::Path::new(&bas_dir).join(".generated");
    let ctx = BuildCtx {
        oapi_dir: std::path::Path::new(&temp_oapi_dir).as_os_str().to_os_string(),
        jar_url: "https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/7.10.0/openapi-generator-cli-7.10.0.jar".to_string(),
        jar_path: std::path::Path::new(&out_dir).join("openapi-generator-cli.jar"),
        spec_path: std::path::Path::new(&bas_dir).join("../specs/spec-crowdproj-ad-v1.yaml").as_os_str().to_os_string(),
        template_dir: std::path::Path::new(&bas_dir).join("templates").as_os_str().to_os_string(),
        or_ignore_path: std::path::Path::new(&bas_dir).join(".openapi-generator-ignore").as_os_str().to_os_string(),
        tg_ignore_path: std::path::Path::new(&temp_oapi_dir).join(".openapi-generator-ignore").as_os_str().to_os_string(),
    };

    download_jar(&ctx).await;
    match Path::new(&ctx.spec_path).is_file() {
        true => println!("File spec does exist {}", ctx.spec_path.to_string_lossy()),
        false => println!(
            "cargo:error=File spec does NOT exist {}",
            ctx.spec_path.to_string_lossy()
        ),
    }
    process_spec(&ctx).await;

    let path = std::path::Path::new(&out_dir).join("test.rs");
    std::fs::write(&path, "pub fn test() { todo!() }").unwrap();
}

async fn download_jar(ctx: &BuildCtx) {
    let mut dest = File::create(&ctx.jar_path).await.unwrap();
    let mut response = reqwest::get(&ctx.jar_url).await.unwrap().bytes_stream();
    while let Some(item) = response.next().await {
        copy(&mut item.unwrap().as_ref(), &mut dest).await.unwrap();
    }
}

async fn process_spec(ctx: &BuildCtx) {
    fs::create_dir_all(&ctx.oapi_dir).unwrap();
    let log_path = std::path::Path::new(&ctx.oapi_dir).join("generator.log");
    let log = std::fs::File::create(log_path).expect("cargo:error=failed to open log");
    fs::copy(&ctx.or_ignore_path, &ctx.tg_ignore_path).unwrap();

    Command::new("java")
        .arg("-jar")
        .arg(&ctx.jar_path)
        .arg("generate")
        .arg("-i")
        .arg(&ctx.spec_path)
        .arg("-g")
        .arg("rust-server")
        // .arg(format!("--ignore-file-override={}",))
        // .arg("--global-property")
        // .arg("models,supportingFiles,apis,modelDocs=false")
        .arg("-t")
        .arg(&ctx.template_dir)
        // .arg("--global-property=models,debugModels,modelDocs,modelTests")
        .current_dir(&ctx.oapi_dir)
        // .stdout(Stdio::piped())
        // .stderr(Stdio::piped())
        .stdout(log.try_clone().unwrap())
        .stderr(log)
        .output()
        .expect("cargo:error=Failed to generate code from specs");
}
