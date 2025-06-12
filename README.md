# LeetCode Daily Challenge Worker

Este es un worker de Cloudflare que se ejecuta diariamente a las 0:10 UTC-0. Su función es obtener el reto diario de LeetCode utilizando la API de GraphQL de LeetCode. Luego, procesa los datos y los envía en formato Markdown a un punto final definido en la variable de entorno `ENDPOINT`. En nuestro caso, este punto final corresponde a un bot de Discord que crea un hilo de discusión donde los usuarios pueden debatir soluciones y enfoques para resolver el reto diario.

## Funcionamiento

1. El worker se activa automáticamente a las 0:10 UTC-0 gracias a un disparador de programación (schedule trigger) en Cloudflare.
2. Se realiza una llamada a la API de GraphQL de LeetCode para obtener los detalles del reto diario.
3. Los datos del reto se procesan y se convierten a formato Markdown.
4. Se envía una solicitud POST a la API definido en la variable de entorno `ENDPOINT` con los datos del reto en formato Markdown.
5. En nuestro caso, el `ENDPOINT` corresponde a un bot de Discord que crea un hilo de discusión con los detalles del reto.

## Configuración

### Requisitos

Para construir y desplegar este proyecto, necesitarás lo siguiente:

- [Rust](https://rust-lang.org)
- [wrangler](https://developers.cloudflare.com/workers/wrangler/install-and-update/)
- [worker-build](https://crates.io/crates/worker-build)
    - [wasm-pack](https://rustwasm.github.io/wasm-pack/)

### Variables de Entorno

- `CANGREBOT_API_ENDPOINT`: URL del punto final al que se enviarán los datos del reto diario.
- `CANGREBOT_APIKEY`: Solo es necesario para nuestro caso que tenemos limitado nuestro endpoint para usuarios permitidos

### Pruebas Locales

Para probar el worker localmente, solo necesitas configurar la variable de entorno `ENDPOINT` en el archivo `wrangler.toml`. Esto es necesario solo si quieres usar un punto final diferente al que ya está configurado.

### Despliegue Automático

Este proyecto está configurado para desplegar automáticamente utilizando los flujos de trabajo de GitHub Actions. Para que funcione correctamente, debes configurar los siguientes secretos en GitHub:

- `CLOUDFLARE_ACCOUNT_ID`: ID de tu cuenta de Cloudflare.
- `CLOUDFLARE_API_TOKEN`: Token de API de Cloudflare.
- `CANGREBOT_API_ENDPOINT`: URL de la API a la que se enviarán los datos del reto diario.
- `CANGREBOT_APIKEY`: Solo es necesario para nuestro caso que tenemos limitado nuestro endpoint para usuarios permitidos
