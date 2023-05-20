use pagetop_build::bundle_resources;
use pagetop_mdbook::util::except_mdbook_common_resources;

fn main() -> std::io::Result<()> {
    bundle_resources("./static/doc", "doc", Some(except_mdbook_common_resources))
}
