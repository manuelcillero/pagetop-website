# Introducción

Si quieres aprender a crear soluciones web que rescaten la esencia de los orígenes usando HTML, CSS y JavaScript para crear páginas web ofrecidas desde un servidor, pero con la potencia de un lenguaje de programación rápido y seguro como Rust, entonces... ¡has llegado a buen puerto!


# ¿Qué es PageTop?

**PageTop** es un marco de desarrollo web que proporciona herramientas y patrones de diseño predefinidos para el desarrollo de soluciones web seguras, modulares y personalizables con *Renderizado desde el Servidor* (SSR).

PageTop está desarrollado en el [lenguaje de programación Rust](https://www.rust-lang.org/) y se apoya sobre los hombros de auténticos gigantes, usando algunas de las librerías (*crates*) más estables y reconocidas del [ecosistema Rust](https://lib.rs) como:

- [Actix Web](https://actix.rs/) para la gestión de los servicios y del servidor web.
- [Tracing](https://github.com/tokio-rs/tracing) para el sistema de diagnóstico y mensajes de registro estructurados.
- [Fluent templates](https://github.com/XAMPPRocky/fluent-templates) que incorpora [Fluent](https://projectfluent.org/) para internacionalizar los proyectos.
- [SeaORM](https://www.sea-ql.org/SeaORM/) que usa [SQLx](https://docs.rs/sqlx/latest/sqlx/) para modelar el acceso a bases de datos.
- Además de integrar versiones *ad hoc* de [config-rs](https://crates.io/crates/config-rs) y [Maud](https://maud.lambda.xyz/) en su código.
- Y otras que puedes consultar en el archivo [`Cargo.toml`](https://github.com/manuelcillero/pagetop/blob/main/Cargo.toml) de PageTop.

La [API de PageTop](https://docs.rs/pagetop) permite adaptar y extender sus funcionalidades a los diferentes escenarios de una solución web usando *Acciones*, *Componentes*, *Paquetes* y *Temas*:

- Las **Acciones** funcionan como un mecanismo para personalizar el comportamiento interno de PageTop interceptando su flujo de ejecución.
- Los **Componentes** permiten encapsular HTML, CSS y JavaScript en unidades funcionales, configurables y bien definidas.
- Los **Paquetes** amplían o personalizan funcionalidades interactuando con las APIs de PageTop o las APIs de paquetes de terceros.
- Y los **Temas** son *paquetes* que van a permitir a los desarrolladores cambiar la apariencia de páginas y componentes sin afectar su funcionalidad.

PageTop [empezó como un proyecto personal](https://manuel.cillero.es/blog/aprendiendo-rust-presentando-pagetop/) para aprender a programar con Rust. Es [libre y de código abierto](https://github.com/manuelcillero/pagetop), para siempre. Y puedes contribuir aumentando su versatilidad, documentando, traduciendo o corrigiendo errores. Pero también puedes crear tus propios paquetes o temas que otros desarrolladores podrán utilizar en sus proyectos.


# Advertencia

PageTop está aún en las primeras etapas de desarrollo. Faltan características importantes y otras no funcionan como deberían. Y la documentación es escasa. Sólo se liberan versiones de desarrollo con cambios importantes en la API que desaconseja su uso en producción. Úsalo si estás interesado en conocerlo o quieres contribuir.

Si necesitas un entorno *fullstack* estable y robusto para tu próximo proyecto, puedes mirar [Perseus](https://github.com/framesurge/perseus) basado en la excelente librería [Sycamore](https://github.com/sycamore-rs/sycamore), también te entusiasmará [Rocket](https://github.com/rwf2/Rocket), sin descartar [MoonZoon](https://github.com/MoonZoon/MoonZoon) o [Percy](https://github.com/chinedufn/percy). Y puedes crear tu propio *framework* combinando soluciones como [Yew](https://yew.rs/), [Leptos](https://leptos.dev/) o [Dioxus](https://dioxuslabs.com/) con el servidor [Axum](https://github.com/tokio-rs/axum) y el ORM [Diesel](https://diesel.rs/) para construir increíbles aplicaciones [SSR](https://en.wikipedia.org/wiki/Server-side_scripting).

Si aún sigues por aquí, ¡ha llegado el momento de empezar a aprender algo de PageTop!

La guía de [Inicio Rápido](getting-started.html) te enseñará a probar los ejemplos y ejecutar [Drust](getting-started.html#prueba-drust). También te ayudará con la [configuración](configuration.html) de tu entorno de desarrollo y te orientará con los próximos pasos a seguir.
