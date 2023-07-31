use pagetop::prelude::*;
use pagetop_megamenu::component::{MegaItem, MegaMenu};
use pagetop_minimal::component::*;

mod home;

new_handle!(APP_PAGETOP_WEBSITE);

static_locales!(LOCALES_WEBSITE);

static_files!(app);
static_files!(doc => BUNDLE_DOC);

struct PageTopWebSite;

impl ModuleTrait for PageTopWebSite {
    fn handle(&self) -> Handle {
        APP_PAGETOP_WEBSITE
    }

    fn name(&self) -> L10n {
        L10n::t("app_name", &LOCALES_WEBSITE)
    }

    fn description(&self) -> L10n {
        L10n::t("app_description", &LOCALES_WEBSITE)
    }

    fn dependencies(&self) -> Vec<ModuleRef> {
        vec![
            // Modules.
            &pagetop_megamenu::MegaMenu,
            &pagetop_minimal::Minimal,
            &pagetop_mdbook::MdBook,
            // Theme.
            &pagetop_bootsier::Bootsier,
        ]
    }

    fn init(&self) {
        add_component_in(
            Region::Named("header"),
            ComponentRef::to(
                MegaMenu::new()
                    .with_item(MegaItem::link(
                        L10n::t("menu_home", &LOCALES_WEBSITE),
                        |cx| match cx.langid().language.as_str() {
                            "es" => "/es",
                            _ => "/",
                        },
                    ))
                    .with_item(MegaItem::link(
                        L10n::t("menu_documentation", &LOCALES_WEBSITE),
                        |cx| match cx.langid().language.as_str() {
                            "es" => "/doc/latest/es",
                            _ => "/doc/latest/en",
                        },
                    ))
                    .with_item(MegaItem::link_blank(
                        L10n::t("menu_api", &LOCALES_WEBSITE),
                        |_| "https://docs.rs/pagetop",
                    ))
                    .with_item(MegaItem::link_blank(
                        L10n::t("menu_code", &LOCALES_WEBSITE),
                        |_| "https://github.com/manuelcillero/pagetop",
                    ))
                    .with_item(MegaItem::html(Html::with(html! {
                        select id="select-lang" {
                            option value="en" { "EN" }
                            option value="es" { "ES" }
                        }
                        script {
                            r###"
var selectLang=document.getElementById('select-lang');
selectLang.value=document.documentElement.lang;
selectLang.addEventListener('change',function(){window.location.href='/'+selectLang.value;});
                            "###
                        }
                    }))),
            ),
        );
        add_component_in(Region::Named("footer"), ComponentRef::to(PoweredBy::new()));
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        serve_static_files!(scfg, "/app", app);
        scfg.route("/doc/latest/{lang}", service::web::get().to(doc_latest))
            .route("/", service::web::get().to(home_default))
            .route("/{lang}", service::web::get().to(home_page));
        pagetop_mdbook::MdBook::configure_service_for_mdbook(scfg, "/doc", &BUNDLE_DOC);
    }
}

async fn doc_latest(path: service::web::Path<String>) -> service::HttpResponse {
    match path.into_inner().as_str() {
        "es" => Redirect::see_other("/doc/v0.0/es/index.html"),
        _ => Redirect::see_other("/doc/v0.0/en/index.html"),
    }
}

async fn home_default(request: service::HttpRequest) -> ResultPage<Markup, FatalError> {
    home::page(request, "en")
}

async fn home_page(
    request: service::HttpRequest,
    path: service::web::Path<String>,
) -> ResultPage<Markup, FatalError> {
    match path.into_inner().as_str() {
        "es" => home::page(request, "es"),
        _ => home::page(request, "en"),
    }
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&PageTopWebSite).unwrap().run()?.await
}
