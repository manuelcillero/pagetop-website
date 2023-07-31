use pagetop::prelude::*;
use pagetop_minimal::component::*;

use super::LOCALES_WEBSITE;

pub fn page(request: service::HttpRequest, lang: &'static str) -> ResultPage<Markup, FatalError> {
    Page::new(request)
        .with_title(L10n::t("homepage_title", &LOCALES_WEBSITE))
        .with_context(ContextOp::LangId(langid_for(lang)))
        .with_context(ContextOp::AddStyleSheet(StyleSheet::at(
            "/app/css/styles.css",
        )))
        .with_body_classes(ClassesOp::AddFirst, "default-homepage")
        .with_in("content", hello_world())
        .with_in("content", welcome())
        .with_in("content", about_pagetop())
        .with_in("content", promo_pagetop())
        .with_in("content", reporting_issues())
        .render()
}

fn hello_world() -> Container {
    Container::header().with_id("hello-world").with_component(
        grid::Row::new()
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "hello-col-text")
                    .with_size(grid::ColumnSize::Is5of12)
                    .with_component(
                        Heading::h1(L10n::t("page_title", &LOCALES_WEBSITE))
                            .with_display(HeadingDisplay::Medium),
                    )
                    .with_component(
                        Paragraph::with(L10n::e("hello_intro", &LOCALES_WEBSITE).with_arg(
                            "app",
                            format!(
                                "<span class=\"app-name\">{}</span>",
                                &config::SETTINGS.app.name,
                            ),
                        ))
                        .with_display(ParagraphDisplay::Small),
                    )
                    .with_component(Paragraph::with(
                        L10n::e("hello_powered", &LOCALES_WEBSITE).with_arg(
                            "pagetop",
                            format!(
                                "<a href=\"{}\" target=\"_blank\">{}</a>",
                                "https://pagetop.cillero.es", "PageTop",
                            ),
                        ),
                    ))
                    .with_component(
                        Anchor::button(
                            "https://github.com/manuelcillero/pagetop",
                            L10n::t("hello_code", &LOCALES_WEBSITE),
                        )
                        .with_target(AnchorTarget::Blank)
                        .with_left_icon(Icon::with("git"))
                        .with_classes(ClassesOp::Add, "code-link"),
                    )
                    .with_component(
                        Anchor::link("#welcome", L10n::t("hello_welcome", &LOCALES_WEBSITE))
                            .with_left_icon(Icon::with("arrow-down-circle-fill"))
                            .with_classes(ClassesOp::Add, "welcome-link"),
                    ),
            )
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "hello-col-image")
                    .with_component(Image::with("/app/images/header.svg")),
            ),
    )
}

fn welcome() -> Container {
    Container::section()
        .with_id("welcome")
        .with_classes(ClassesOp::Add, "welcome-col-text")
        .with_component(Heading::h2(L10n::t("welcome_page", &LOCALES_WEBSITE)))
        .with_component(
            Heading::h3(L10n::e("welcome_subtitle", &LOCALES_WEBSITE).with_arg(
                "app",
                format!(
                    "<span class=\"app-name\">{}</span>",
                    &config::SETTINGS.app.name
                ),
            ))
            .with_display(HeadingDisplay::Subtitle),
        )
        .with_component(
            Paragraph::with(L10n::t("welcome_text1", &LOCALES_WEBSITE))
                .with_display(ParagraphDisplay::Small),
        )
        .with_component(Paragraph::with(L10n::t("welcome_text2", &LOCALES_WEBSITE)))
}

fn about_pagetop() -> Container {
    Container::new().with_id("pagetop").with_component(
        grid::Row::new()
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "pagetop-col-image")
                    .with_size(grid::ColumnSize::Is5of12)
                    .with_component(Image::with("/app/images/about.svg")),
            )
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "pagetop-col-text")
                    .with_component(Heading::h2(L10n::t("pagetop_title", &LOCALES_WEBSITE)))
                    .with_component(
                        Paragraph::with(L10n::t("pagetop_text1", &LOCALES_WEBSITE))
                            .with_display(ParagraphDisplay::Small),
                    )
                    .with_component(Paragraph::with(L10n::t("pagetop_text2", &LOCALES_WEBSITE)))
                    .with_component(Paragraph::with(
                        L10n::e("pagetop_text3", &LOCALES_WEBSITE)
                            .with_arg("href", "https://docs.rs/pagetop/latest/pagetop"),
                    )),
            ),
    )
}

fn promo_pagetop() -> Container {
    Container::new().with_id("promo").with_component(
        grid::Row::new()
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "promo-col-text")
                    .with_component(Heading::h2(L10n::t(
                        "pagetop_promo_title",
                        &LOCALES_WEBSITE,
                    )))
                    .with_component(
                        Paragraph::with(L10n::e("pagetop_promo_text1", &LOCALES_WEBSITE).with_arg(
                            "pagetop",
                            format!(
                                "<a href=\"{}\" target=\"_blank\">{}</a>",
                                "https://crates.io/crates/pagetop", "PageTop",
                            ),
                        ))
                        .with_display(ParagraphDisplay::Small),
                    ),
            )
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "promo-col-image")
                    .with_size(grid::ColumnSize::Is6of12)
                    .with_component(Image::with("/app/images/pagetop.png")),
            ),
    )
}

fn reporting_issues() -> Container {
    Container::new().with_id("reporting").with_component(
        grid::Row::new()
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "reporting-col-image")
                    .with_component(Image::with("/app/images/support.jpg")),
            )
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "reporting-col-text")
                    .with_size(grid::ColumnSize::Is6of12)
                    .with_component(Heading::h2(L10n::t(
                        "report_problems_title",
                        &LOCALES_WEBSITE,
                    )))
                    .with_component(
                        Paragraph::with(L10n::t("report_problems_text1", &LOCALES_WEBSITE))
                            .with_display(ParagraphDisplay::Small),
                    )
                    .with_component(Paragraph::with(L10n::t(
                        "report_problems_text2",
                        &LOCALES_WEBSITE,
                    ))),
            ),
    )
}
