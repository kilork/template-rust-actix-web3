use actix_web_static_files::NpmBuild;

fn main() {
    NpmBuild::new("./web")
        .target("./web/dist")
        .install()
        .unwrap()
        .run("build")
        .unwrap()
        .change_detection()
        .to_resource_dir()
        .build()
        .unwrap();
}
