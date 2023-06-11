use pagetop::prelude::*;

define_handle!(APP_PAGETOP_WEBSITE);

include!(concat!(env!("OUT_DIR"), "/doc.rs"));
static DOC: LazyStatic<HashMapResources> = LazyStatic::new(bundle_doc);

struct PageTopWebSite;

impl ModuleTrait for PageTopWebSite {
    fn handle(&self) -> Handle {
        APP_PAGETOP_WEBSITE
    }

    fn dependencies(&self) -> Vec<ModuleStaticRef> {
        vec![
            // Modules.
            &pagetop_homedemo::HomeDemo,
            &pagetop_mdbook::MdBook,
            // Theme.
            &pagetop_bootsier::Bootsier,
        ]
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        pagetop_mdbook::MdBook::configure_service_for_mdbook(cfg, "/doc", &DOC);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&PageTopWebSite).unwrap().run()?.await
}
