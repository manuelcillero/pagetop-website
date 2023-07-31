use pagetop_mdbook::util::except_mdbook_common_resources;

fn main() -> std::io::Result<()> {
    pagetop_build::StaticFilesBundle::from_dir("./static/app")
        .with_name("app")
        .build()?;
    pagetop_build::StaticFilesBundle::from_dir("./static/doc")
        .with_name("doc")
        .with_filter(except_mdbook_common_resources)
        .build()
}
