use pagetop::prelude::*;

mod home;

static_locales!(LOCALES_WEBSITE);

static_files!(app);
static_files!(doc => BUNDLE_DOC);

struct PageTopWebSite;

impl PackageTrait for PageTopWebSite {
    fn name(&self) -> L10n {
        L10n::t("app_name", &LOCALES_WEBSITE)
    }

    fn description(&self) -> L10n {
        L10n::t("app_description", &LOCALES_WEBSITE)
    }

    fn dependencies(&self) -> Vec<PackageRef> {
        vec![
            // Packages.
            &pagetop_mdbook::MdBook,
            // Theme.
            &pagetop_bootsier::Bootsier,
        ]
    }

    fn init(&self) {
        let branding = Branding::new()
            .with_logo(Some(Image::pagetop()))
            .with_slogan(L10n::t("app_slogan", &LOCALES_WEBSITE))
            .with_frontpage(|cx| match cx.langid().language.as_str() {
                "es" => "/es",
                _ => "/",
            });
        let menu = Menu::new()
            .add_item(menu::Item::link(
                L10n::t("menu_home", &LOCALES_WEBSITE),
                |cx| match cx.langid().language.as_str() {
                    "es" => "/es",
                    _ => "/",
                },
            ))
            .add_item(menu::Item::link(
                L10n::t("menu_documentation", &LOCALES_WEBSITE),
                |cx| match cx.langid().language.as_str() {
                    "es" => "/doc/latest/es",
                    _ => "/doc/latest/en",
                },
            ))
            .add_item(menu::Item::link_blank(
                L10n::t("menu_api", &LOCALES_WEBSITE),
                |_| "https://docs.rs/pagetop",
            ))
            .add_item(menu::Item::link_blank(
                L10n::t("menu_code", &LOCALES_WEBSITE),
                |_| "https://github.com/manuelcillero/pagetop",
            ))
            .add_item(menu::Item::html(Html::with(html! {
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
            })));
        add_component_in(
            Region::Named("header"),
            ArcAnyComponent::new(
                flex::Container::new()
                    .with_direction(flex::Direction::Row(BreakPoint::None))
                    .with_content_justify(flex::ContentJustify::SpaceBetween)
                    .with_items_align(flex::ItemAlign::Bottom)
                    .add_item(flex::Item::new().add_component(branding))
                    .add_item(flex::Item::new().add_component(menu)),
            ),
        );
        add_component_in(
            Region::Named("footer"),
            ArcAnyComponent::new(PoweredBy::new()),
        );
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.route("/doc/latest/{lang}", service::web::get().to(doc_latest))
            .route("/", service::web::get().to(home_default))
            .route("/{lang}", service::web::get().to(home_page));
        pagetop_mdbook::MdBook::service_for_mdbook(scfg, "/doc", &BUNDLE_DOC);
        service_for_static_files!(scfg, app => "/app");
    }
}

async fn doc_latest(path: service::web::Path<String>) -> service::HttpResponse {
    match path.into_inner().as_str() {
        "es" => Redirect::see_other("/doc/v0.0/es/index.html"),
        _ => Redirect::see_other("/doc/v0.0/en/index.html"),
    }
}

async fn home_default(request: service::HttpRequest) -> ResultPage<Markup, ErrorPage> {
    home::page(request, langid_for("en").unwrap_or(&LANGID_FALLBACK))
}

async fn home_page(
    request: service::HttpRequest,
    path: service::web::Path<String>,
) -> ResultPage<Markup, ErrorPage> {
    match langid_for(path.into_inner()) {
        Ok(lang) => home::page(request, lang),
        _ => Err(ErrorPage::NotFound(request)),
    }
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&PageTopWebSite).run()?.await
}
