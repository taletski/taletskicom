#[cfg(not(feature = "build_static"))]
fn main() {}

#[cfg(feature = "build_static")]
fn main() {
    static_build::build();
}

// I played around with Tower, Axum and Askama and had a lot of fun.
// For such a small website, the server
//
// But currently performance suffers from a cheap machine cold startups that add an
// 0.5s-0.8s delay.
//
// So for now, as a quick solution to a practical issue I will just build static files with this script
// and serve them with a free CDN for the best performance.
//
// The dynamic server will still live on a temp domain just as a POC.
//
// As the project will continue to grow I will have to decide if I need a dynamic server or if
// or if I should completely switch to an SSG approach.
mod static_build {
    use fs_extra::dir::CopyOptions;
    use templates::Template;

    #[allow(dead_code)]
    pub fn build() {
        let crate_path = std::env::current_dir().expect("Failed to read current dir");
        let dist_path = crate_path.join("target/release_static");

        let _ = std::fs::remove_dir_all(&dist_path);
        std::fs::create_dir_all(&dist_path).expect("Failed to create dist directory");

        let copyt_options = CopyOptions::new().content_only(true);

        fs_extra::dir::copy(crate_path.join("static"), &dist_path, &copyt_options)
            .expect("Failed to copy static/ dir");
        fs_extra::dir::copy(
            crate_path.join("assets"),
            dist_path.join("assets"),
            &copyt_options,
        )
        .expect("Failed to copy assets/ dir");

        let homepage_html = (templates::HomepageTemplate {})
            .render()
            .unwrap_or_else(|e| panic!("Failed to read homepage template: {e}"));
        std::fs::write(dist_path.join("index.html"), homepage_html)
            .expect("Failed to write homepage to index.html");

        std::process::exit(1); // Prevents building the rest of the project
    }
}
