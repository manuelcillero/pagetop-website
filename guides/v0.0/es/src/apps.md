# Aplicaciones

Los programas de PageTop se denominan [Aplicaciones](https://docs.rs/pagetop/latest/pagetop/app/struct.Application.html). La aplicación PageTop más simple luce así:

```rust
use pagetop::prelude::*;

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::new().run()?.await
}
```

La línea `use pagetop::prelude::*;` sirve para importar la API esencial de PageTop. Por brevedad, esta guía podría omitirla en ejemplos posteriores.

Ahora sólo tienes que copiar el código anterior en tu archivo `main.rs` y desde la carpeta del proyecto ejecutar:

```bash
cargo run
```

Si todo ha ido bien, después de compilar el código ejecutará la aplicación y el terminal quedará en espera mostrando el *título* y *lema* predefinidos.

Ahora abre un navegador en el mismo equipo y escribe `http://localhost:8088` en la barra de direcciones. Y ya está, ¡la página de presentación de PageTop te dará la bienvenida!

Sin embargo, aún no hemos indicado a nuestra aplicación qué hacer. Si quieres saber más sobre el funcionamiento interno de las aplicaciones, continúa leyendo. De lo contrario, puedes ir a la siguiente página para aprender cómo añadir lógica a nuestra aplicación.


# ¿Qué hace una aplicación?

Como hemos visto, primero debemos instanciar la [Aplicación](https://docs.rs/pagetop/latest/pagetop/app/struct.Application.html). Podemos hacerlo con dos métodos, [`new()`](https://docs.rs/pagetop/latest/pagetop/app/struct.Application.html#method.new), que hemos usado en el ejemplo anterior, o [`prepare()`](https://docs.rs/pagetop/latest/pagetop/app/struct.Application.html#method.prepare), que veremos en la siguiente página. Uno u otro método se encargan de iniciar los diferentes subsistemas de PageTop por este orden:

1. Inicializa la traza de mensajes de registro y eventos.

2. Valida el identicador global de idioma.

3. Conecta con la base de datos.

4. Registra los paquetes de la aplicación según sus dependencias internas.

5. Registra las acciones de los paquetes.

6. Inicializa los paquetes.

7. Ejecuta las actualizaciones pendientes de la base de datos.

Por sí misma no hace nada más, por eso se usa el método [`run()`](https://docs.rs/pagetop/latest/pagetop/app/struct.Application.html#method.run), para ejecutar el servidor web y poner en marcha nuestra aplicación.

Hablaremos más de todos estos subsistemas en las siguientes páginas. Mientras tanto, ¡vamos a añadir algo de lógia a nuestra aplicación creando un paquete con un nuevo servicio web!