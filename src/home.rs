use pagetop::prelude::*;

use super::LOCALES_WEBSITE;

pub fn page(
    request: service::HttpRequest,
    lang: &'static LanguageIdentifier,
) -> ResultPage<Markup, ErrorPage> {
    Page::new(request)
        .with_title(L10n::t("homepage_title", &LOCALES_WEBSITE))
        .with_context(ContextOp::LangId(lang))
        .with_context(ContextOp::AddStyleSheet(StyleSheet::at(
            "/app/css/styles.css",
        )))
        .with_body_classes(ClassesOp::Add, "default-homepage")
        .with_component_in("content", hello_world())
        .with_component_in("content", welcome())
        .with_component_in("content", about_pagetop())
        .with_component_in("content", promo_pagetop())
        .with_component_in("content", reporting_issues())
        .render()
}

fn hello_world() -> Wrapper {
    Wrapper::header().with_id("hello-world").add_component(
        flex::Container::new()
            .with_direction(flex::Direction::Column(BreakPoint::MD))
            .add_item(
                flex::Item::new()
                    .with_inner_classes(ClassesOp::Add, "hello-col-text")
                    .with_size(flex::ItemSize::Percent40)
                    .add_component(
                        Heading::h1(L10n::t("page_title", &LOCALES_WEBSITE))
                            .with_size(HeadingSize::Medium),
                    )
                    .add_component(
                        Paragraph::translated(L10n::t("hello_intro", &LOCALES_WEBSITE).with_arg(
                            "app",
                            format!(
                                "<span class=\"app-name\">{}</span>",
                                &config::SETTINGS.app.name,
                            ),
                        ))
                        .with_font_size(FontSize::Medium),
                    )
                    .add_component(Paragraph::translated(
                        L10n::t("hello_powered", &LOCALES_WEBSITE).with_arg(
                            "pagetop",
                            format!(
                                "<a href=\"{}\" target=\"_blank\">{}</a>",
                                "https://pagetop.cillero.es", "PageTop",
                            ),
                        ),
                    ))
                    .add_component(
                        Button::anchor(
                            "https://github.com/manuelcillero/pagetop",
                            L10n::t("hello_code", &LOCALES_WEBSITE),
                        )
                        .with_target(ButtonTarget::Blank)
                        .with_left_icon(Some(Icon::with("git")))
                        .with_classes(ClassesOp::Add, "code-link"),
                    )
                    .add_component(
                        Button::anchor("#welcome", L10n::t("hello_welcome", &LOCALES_WEBSITE))
                            .with_style(ButtonStyle::Link)
                            .with_left_icon(Some(Icon::with("arrow-down-circle-fill")))
                            .with_classes(ClassesOp::Add, "welcome-link"),
                    ),
            )
            .add_item(
                flex::Item::new()
                    .with_inner_classes(ClassesOp::Add, "hello-col-image")
                    .with_size(flex::ItemSize::Percent60)
                    .add_component(Image::with("/app/images/header.svg")),
            ),
    )
}

fn welcome() -> Wrapper {
    Wrapper::section()
        .with_id("welcome")
        .with_classes(ClassesOp::Add, "welcome-col-text")
        .add_component(Heading::h2(L10n::t("welcome_page", &LOCALES_WEBSITE)))
        .add_component(
            Heading::h3(L10n::t("welcome_subtitle", &LOCALES_WEBSITE).with_arg(
                "app",
                format!(
                    "<span class=\"app-name\">{}</span>",
                    &config::SETTINGS.app.name
                ),
            ))
            .with_size(HeadingSize::Subtitle),
        )
        .add_component(
            Paragraph::translated(L10n::t("welcome_text1", &LOCALES_WEBSITE))
                .with_font_size(FontSize::Medium),
        )
        .add_component(Paragraph::translated(L10n::t(
            "welcome_text2",
            &LOCALES_WEBSITE,
        )))
}

fn about_pagetop() -> Wrapper {
    Wrapper::new().with_id("pagetop").add_component(
        flex::Container::new()
            .with_direction(flex::Direction::Column(BreakPoint::SM))
            .add_item(
                flex::Item::new()
                    .with_inner_classes(ClassesOp::Add, "pagetop-col-image")
                    .with_size(flex::ItemSize::Percent40)
                    .add_component(Image::with("/app/images/about.svg")),
            )
            .add_item(
                flex::Item::new()
                    .with_inner_classes(ClassesOp::Add, "pagetop-col-text")
                    .add_component(Heading::h2(L10n::t("pagetop_title", &LOCALES_WEBSITE)))
                    .add_component(
                        Paragraph::translated(L10n::t("pagetop_text1", &LOCALES_WEBSITE))
                            .with_font_size(FontSize::Medium),
                    )
                    .add_component(Paragraph::translated(L10n::t(
                        "pagetop_text2",
                        &LOCALES_WEBSITE,
                    )))
                    .add_component(Paragraph::translated(
                        L10n::t("pagetop_text3", &LOCALES_WEBSITE)
                            .with_arg("href", "https://docs.rs/pagetop/latest/pagetop"),
                    )),
            ),
    )
}

fn promo_pagetop() -> Wrapper {
    Wrapper::new().with_id("promo").add_component(
        flex::Container::new()
            .with_direction(flex::Direction::Column(BreakPoint::MD))
            .add_item(
                flex::Item::new()
                    .with_inner_classes(ClassesOp::Add, "promo-col-text")
                    .with_size(flex::ItemSize::Percent60)
                    .add_component(Heading::h2(L10n::t(
                        "pagetop_promo_title",
                        &LOCALES_WEBSITE,
                    )))
                    .add_component(
                        Paragraph::translated(
                            L10n::t("pagetop_promo_text1", &LOCALES_WEBSITE).with_arg(
                                "pagetop",
                                format!(
                                    "<a href=\"{}\" target=\"_blank\">{}</a>",
                                    "https://crates.io/crates/pagetop", "PageTop",
                                ),
                            ),
                        )
                        .with_font_size(FontSize::Medium),
                    ),
            )
            .add_item(
                flex::Item::new()
                    .with_inner_classes(ClassesOp::Add, "promo-col-image")
                    .with_size(flex::ItemSize::Percent40)
                    .add_component(Image::with("/app/images/pagetop.png")),
            ),
    )
}

fn reporting_issues() -> Wrapper {
    Wrapper::new().with_id("reporting").add_component(
        flex::Container::new()
            .with_direction(flex::Direction::Column(BreakPoint::MD))
            .add_item(
                flex::Item::new()
                    .with_inner_classes(ClassesOp::Add, "reporting-col-image")
                    .add_component(Image::with("/app/images/support.jpg")),
            )
            .add_item(
                flex::Item::new()
                    .with_inner_classes(ClassesOp::Add, "reporting-col-text")
                    .with_size(flex::ItemSize::Percent50)
                    .add_component(Heading::h2(L10n::t(
                        "report_problems_title",
                        &LOCALES_WEBSITE,
                    )))
                    .add_component(
                        Paragraph::translated(L10n::t("report_problems_text1", &LOCALES_WEBSITE))
                            .with_font_size(FontSize::Medium),
                    )
                    .add_component(Paragraph::translated(L10n::t(
                        "report_problems_text2",
                        &LOCALES_WEBSITE,
                    ))),
            ),
    )
}
