# Comenzando

Esta sección te ayudará a conocer PageTop de la manera más rápida posible. Te enseñará a preparar un entorno de desarrollo apropiado para crear una aplicación web sencilla usando PageTop.


# Inicio rápido

Si quieres entrar de lleno en el código de PageTop y ya cuentas con un entorno de Rust operativo puedes seguir leyendo este apartado de "inicio rápido".

En otro caso puedes pasar a la siguiente página para preparar un entorno de Rust desde cero y empezar a programar tu primera aplicación web con PageTop.

<!-- Nota: la configuración para "compilaciones rápidas" se encuentra en la próxima página, por lo que podrías querer leer esa sección primero. -->

## Empieza con los ejemplos

1. Clona el [repositorio de PageTop](https://github.com/manuelcillero/pagetop):

   ```bash
   git clone https://github.com/manuelcillero/pagetop
   ```

2. Cambia a la carpeta recién creada "pagetop":

   ```bash
   cd pagetop
   ```

3. Asegurate de que trabajas con la última versión de PageTop (ya que por defecto se descarga la rama principal de git):

   ```bash
   git checkout latest
   ```

4. Prueba los ejemplos de la [carpeta de ejemplos](https://github.com/manuelcillero/pagetop/tree/latest/examples):

   ```bash
   cargo run --example hello-world
   ```

## Prueba Drust

**Drust** es un Sistema de Gestión de Contenidos (CMS) desarrollado con PageTop y modestamente inspirado en [Drupal](https://drupal.org). Permitirá crear blogs personales, foros, portales web o aplicaciones más sofisticadas, y se fundamenta en estos principios clave:

- **Simplicidad**: Intuitivo para principiantes y versátil para usuarios avanzados.
- **Seguridad**: Diseñado para proteger contra las vulnerabilidades web más comunes.
- **Eficiencia**: Capaz de soportar la demanda que se espera de un sitio web fluido y ágil.
- **Funcionalidad**: Facilitando la creación de sitios web dinámicos sin necesidad de programar.
- **Modularidad**: Integrando los paquetes y temas de PageTop, y admitiendo paquetes externos para extender sus capacidades.

Drust requiere una base de datos para funcionar. Usa el script `tools/drust-create-db.sh` para crearla. Desde la carpeta "pagetop":

```bash
cd tools
./drust-create-db.sh
```

Cambia a la carpeta "drust":

```bash
cd ..
cd drust
```

Verifica que los datos de conexión a la base de datos de Drust están correctamente asignados en el archivo de configuración `config/local.default.toml`.

Y ejecuta la aplicación:

```bash
cargo run
```
