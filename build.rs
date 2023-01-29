use pagetop_build::bundle_resources;
use pagetop_mdbook::util::except_mdbook_common_resources;

fn main() -> std::io::Result<()> {
    bundle_resources("./static/doc/en", "guides_en", Some(except_mdbook_common_resources))?;
    bundle_resources("./static/doc/es", "guias_es",  Some(except_mdbook_common_resources))
}
