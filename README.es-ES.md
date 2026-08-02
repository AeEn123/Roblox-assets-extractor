

[//]: <> (No edites este archivo. Esto es generado por build_readme.py. En su lugar, edita el README en docs.)
[🇬🇧 Inglés](/docs/en-GB/README.md) | [🇷🇺 Русский](/docs/ru-RU/README.md) | [🇧🇾 Беларуская](/docs/be-BY/README.md) | 
*¿Hablas inglés y cualquier otro idioma? ¡Ayuda a traducir creando un pull request!*


[![Download for Windows](https://img.shields.io/github/downloads/AeEn123/RoExtract/latest/RoExtract-windows.exe?label=Download&color=blue&style=for-the-badge)](https://github.com/AeEn123/RoExtract/releases/latest/download/RoExtract-windows.exe)
[![Download for Linux](https://img.shields.io/github/downloads/AeEn123/RoExtract/latest/RoExtract-linux?label=Download&style=for-the-badge)](https://github.com/AeEn123/RoExtract/releases/latest/download/RoExtract-linux)
[![Website](https://img.shields.io/badge/Website-red?logo=googlechrome&style=for-the-badge)](https://aeen123.github.io/RoExtract/)

[![Build and Release](https://github.com/AeEn123/RoExtract/actions/workflows/build-and-release.yml/badge.svg)](https://github.com/AeEn123/RoExtract/actions/workflows/build-and-release.yml)
[![Discord invite](https://img.shields.io/discord/470242481582243860?label=Discord)](https://discord.gg/xqNA5jt6DN)
![Total downloads](https://img.shields.io/github/downloads/AeEn123/RoExtract/total?label=Total%20Downloads)

# Descargo de responsabilidad
Este es un proyecto educativo independiente. RoExtract **NO** está afiliado a Roblox Corporation de ninguna manera.

# RoExtract
Esta herramienta extrae datos en caché de tu instalación de Roblox al examinar los encabezados de los archivos en caché.

![Screenshot](/assets/screenshot.png)

# Estado del proyecto
Me he quedado sin motivación para el proyecto; sin embargo, he comenzado a usar intensamente herramientas de [IA Generativa](https://en.wikipedia.org/wiki/Generative_AI) para ayudarme a desarrollarlo sin mucha motivación.
Las personas que no les guste esto y deseen continuar trabajando en el código como estaba antes del uso intensivo de GenAI deberían revisar la rama [human](https://github.com/AeEn123/RoExtract/tree/human).
Por ahora, usaré GenAI para introducir las últimas funciones y corregir errores. Entiendo que esto no es una buena dirección para el proyecto, pero es mejor que abandonarlo y que nadie lo bifurque.

# Preguntas frecuentes
### El programa no puede ejecutarse porque falta VCRUNTIME140.dll 
Instala [Microsoft Visual C++ Redistributable](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist)

### ¿Esto interfiere con el cliente de Roblox?
No, está abriendo archivos que tu cliente ya ha creado. Puedes verlos tú mismo en %Temp%\Roblox

### ¿Es esto un malware?
No, esto no es un malware; al igual que otros programas de Código Libre y de Código Abierto, el código está disponible para que todos lo vean. Sería imposible para nosotros ocultar un malware aquí. Las compilaciones también son seguras, ya que ahora se crean mediante GitHub Actions, lo que significa que todo es completamente transparente. Si aún no confías en este software, puedes usar la demostración web en https://aeen123.github.io/RoExtract/demo (no necesitas descargar nada para eso)

### Windows dice "Windows ha protegido tu PC". ¿Qué debo hacer?
Si Windows detecta un programa de un editor no verificado, aparecerá esta ventana emergente. Si aparece, haz clic en "Más información" y luego en "Ejecutar de todos modos".

### ¿Esto puede hacer que me banen?
No, a diferencia de los trucos, esto **no** inyecta nada en Roblox. Esto convierte la extracción de recursos en un método compatible con los anti-trucos.

### Mis recursos extraídos no se reproducen en mi reproductor de medios, ¿qué puedo hacer?
Algunos reproductores de medios pueden no admitir el formato en el que está el archivo. Si es ese el caso, intenta usar otro reproductor que admita todos los formatos compatibles con este, como VLC. **Si el archivo realmente está dañado, por favor [crea un issue.](https://github.com/AeEn123/RoExtract/issues)**

### ¿Por qué los archivos KTX están en una pestaña diferente? ¿No deberían estar en la pestaña Texturas?
Técnicamente debería ser así, pero la mayoría de los visores de imágenes no admiten archivos KTX, por lo que es mejor moverlos a una pestaña diferente para evitar problemas de compatibilidad; esta pestaña debe usarse para usuarios más avanzados.

### ¿Por qué los archivos RBXM son solo una "Instance" en Roblox Studio?
Roblox Studio no tiene soporte para archivos RBXM en caché. Estos archivos pueden contener datos de juegos, pero aún no lo hemos investigado.

### ¿Esto ocupa almacenamiento con el tiempo?
La propia caché de Roblox sí ocupa almacenamiento con el tiempo; esta herramienta en sí no añade ningún uso de almacenamiento con el paso del tiempo, a menos que estés extrayendo muchos archivos, los cuales puedes eliminar fácilmente. 

# Uso
## Pestañas
Puedes ver múltiples pestañas. RoExtract clasifica los archivos en varias categorías. Puedes filtrarlos haciendo clic en la pestaña correspondiente.
## La barra de herramientas
Cada elemento de la barra de herramientas te permite realizar diferentes operaciones con el directorio o el recurso; también puedes acceder a la barra de herramientas como un menú contextual haciendo clic derecho. Puedes desactivar la barra de herramientas en la parte superior de la pantalla en la configuración, en la opción **Habilitar barra de herramientas** dentro de la sección **Comportamiento**.
## Navegación con teclado y atajos
El programa está diseñado para ser fácil de usar con el mouse, pero también permite la navegación con teclado y atajos para usuarios más avanzados; los atajos se muestran en los botones para indicar cómo puedes acceder a ellos rápidamente.<br>

Puedes alternar entre las pestañas con Alt (o Ctrl) + 1-8, lo que te permite navegar entre pestañas usando solo el teclado; puedes seleccionar recursos con Tab y confirmar con Enter.
## Menú de configuración
En el menú de configuración encontrarás opciones generales de personalización, así como acciones para realizar con tu caché de Roblox. Aquí puedes extraer toda tu caché de Roblox, cambiar el directorio o limpiar la caché.

# Modo CLI
El modo CLI está en desarrollo.
Consulta [CLI.md](/docs/en-GB/CLI.md)

# Instalación para Windows
Por ahora, el programa solo viene en versión portátil para Windows, esto podría cambiar en el futuro.

# Instalación para Linux
## Flatpak (MUY EXPERIMENTAL)
> [!WARNING]
> El soporte para Flatpak es MUY EXPERIMENTAL, úsalo bajo tu propio riesgo.
Actualmente no hay paquetes flatpak preconstruidos, sigue la [guía de compilación](packages/flatpak/README.md) para obtener instrucciones sobre cómo compilar el paquete flatpak.

## Arch Linux
Puedes instalarlo en Arch Linux utilizando el PKGBUILD ubicado en `packages/arch`
Un script de instalación de ejemplo:
```bash
mkdir /tmp/RoExtract
cd /tmp/RoExtract
wget raw.githubusercontent.com/AeEn123/RoExtract/refs/heads/main/packages/arch/PKGBUILD
makepkg -si
```
## Otras distribuciones
Se espera que otras distribuciones sean compatibles pronto. Si sabes cómo crear una y deseas que se fusione en este proyecto, ¡crea un pull request!

# Prueba de compilaciones de desarrollo
Las compilaciones de desarrollo se pueden descargar desde la página de [releases](https://github.com/AeEn123/RoExtract/releases).

Si ya tienes instalada la última compilación de desarrollo de RoExtract, puedes habilitar las compilaciones de desarrollo en la configuración.
# Más información
Este es mi primer proyecto escrito en rust/egui, por lo que pueden aparecer errores; si aparece uno, informa un issue.

> [!IMPORTANT]
> Esta herramienta está diseñada para Windows y GNU/Linux y puede que no funcione en otros sistemas operativos.

> [!TIP]
> Si la lista de archivos es demasiado lenta, puedes limpiar tu caché con el botón de limpiar caché en la configuración. Además, desactivar Windows Defender acelerará la lista de archivos, ya que escanea cada vez que se abre un archivo.

# Compilación desde el código fuente

Compilar desde el código fuente requiere cargo, [el cual puede instalarse desde rustup.](https://rustup.rs/)

## 1. Clona el repositorio
```bash
git clone https://github.com/AeEn123/RoExtract
cd RoExtract
```
## 2. Compila con cargo, el comando que ejecutes depende de tu caso de uso
Si deseas una compilación final que se ejecute rápido pero compile lentamente (recomendado para uso normal)
```bash
cargo build --release
```

Si deseas una compilación de desarrollo que se ejecute lentamente pero compile rápido (recomendado para desarrollo)
```bash
cargo build
```
Espera a que compile todas las dependencias y la aplicación. Después, deberías encontrarla en la carpeta `target`.

# Versión de Python
La versión de Python ha sido descontinuada.
