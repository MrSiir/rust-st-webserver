# Rust Web Server

Este proyecto es una implementación sencilla de un servidor web simple en Rust, es un programa de ejemplo del capitulo 20 del libro _The Rust Programming Language_ (https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html) titulado _Final Project: Building a Multithreaded Web Server_.

## Estructura del Proyecto

### Archivos principales

- **`main.rs`**: Contiene la lógica principal para el servidor, incluidas las conexiones TCP y el manejo de solicitudes.
- **`lib.rs`**: Implementa la configuración y un grupo de hilos para manejar solicitudes concurrentes.

### Carpeta HTML

Los archivos HTML se sirven desde la carpeta `html`. Los archivos deben estar organizados con las siguientes convenciones:

- `index.html`: Página principal.
- `404.html`: Página de error para "No encontrado".
- Otros archivos HTML deben coincidir con las rutas solicitadas por el cliente.

## Requisitos

### Dependencias

- **Rust**: Versión estable. Instálalo desde [Rustup](https://rustup.rs/).

### Configuración

- Asegúrate de que la carpeta `html` esté en el directorio raíz del proyecto.
- Incluye al menos los siguientes archivos en la carpeta `html`:
  - `index.html`
  - `404.html`

## Cómo Ejecutar

### Compilación

Ejecuta el siguiente comando para compilar el proyecto:

```bash
cargo build --release
```

### Ejecución

Proporciona el puerto al ejecutar el programa:

```bash
cargo run <puerto>
```

El servidor estará disponible en `http://localhost:<puerto>`.

## Ejemplo de Uso

1. Accede a `http://localhost:8080/` para cargar la página principal (`index.html`).
2. Accede a `http://localhost:8080/<archivo>` para otros archivos en la carpeta `html`.
3. Si solicitas una página no disponible, el servidor devolverá el contenido de `404.html`.

## Funcionalidades Principales

### Manejo de Conexiones

- El servidor utiliza un `ThreadPool` con 4 hilos por defecto.
- Las conexiones se manejan concurrentemente.

### Respuestas HTTP

- `200 OK`: Devuelve el contenido solicitado.
- `404 Not Found`: Devuelve una página de error personalizada.
- `400 Bad Request`: Para solicitudes mal formadas.

### Cierre del Servidor

El servidor cierra los hilos de manera ordenada cuando el programa se detiene.

## Próximos Pasos

- **Mejoras de seguridad**: Validar rutas para evitar acceso no autorizado a archivos fuera de la carpeta `html`.
- **Soporte para MIME types**: Determinar el tipo de contenido según la extensión del archivo.
- **Registros más detallados**: Agregar más logs para depuración.

## Licencia

Este proyecto está licenciado bajo [MIT License](LICENSE).

Este fichero ha sido generado por ChatGPT.
