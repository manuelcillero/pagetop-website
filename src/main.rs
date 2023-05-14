use pagetop::prelude::*;
use pagetop_bootsier::Bootsier;
use pagetop_homedemo::HomeDemo;
use pagetop_mdbook::MdBook;

define_handle!(APP_PAGETOP_WEBSITE);

include!(concat!(env!("OUT_DIR"), "/guides_en.rs"));
static GUIDES_EN: LazyStatic<HashMapResources> = LazyStatic::new(bundle_guides_en);

include!(concat!(env!("OUT_DIR"), "/guias_es.rs"));
static GUIAS_ES: LazyStatic<HashMapResources> = LazyStatic::new(bundle_guias_es);

struct PageTopWebSite;

impl ModuleTrait for PageTopWebSite {
    fn handle(&self) -> Handle {
        APP_PAGETOP_WEBSITE
    }

    fn dependencies(&self) -> Vec<ModuleStaticRef> {
        vec![&Bootsier, &HomeDemo, &MdBook]
    }

    fn configure_service(&self, cfg: &mut server::web::ServiceConfig) {
        MdBook::configure_service_for_common_resources(cfg);
        MdBook::configure_service_for_mdbook(cfg, "/doc/en", &GUIDES_EN);
        MdBook::configure_service_for_mdbook(cfg, "/doc/es", &GUIAS_ES);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&PageTopWebSite).unwrap().run()?.await
}
