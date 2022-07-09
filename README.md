# DIY the hard way - Secret chat System

Empieza: 04/07/2022

Fecha mínima: 25/07/2022

Fecha máxima: 04/08/2022

## Leyenda

	- RSIS: Requisitos que debe cumplir el sistema.
	- RS: Requisitos que debe cumplir el servidor.
	- RC: Requisitos que debe cumplir el cliente.

## Aclaración

	- Cliente: Software que permite a las personas interactuar con los demás.
	- Servidor: Software que conecta los clientes.

Toda la actividad que se realice de desarrollo tiene que estar registrada en GitHub o GitLab (para que lo podáis poner en el curriculum)

La idea es que desde la organización que se ha creado, hagáis un _fork_ y desarrollareis en base a los requisitos que se ponen ahí.
GH repository: https://github.com/DIY-the-hard-way/secre...secret...sh.git
Hay total libertad para **AÑADIR** otros requisitos, pero **NO ELIMINAR NI MODIFICAR** los requisitos base, que son los que se proponen.

Cuando se realice el _fork_ se han de crear **dos carpetas**:

	- Client: Se encontrará el código fuente del Cliente
	- Server: Se encontrará el código fuente del Servidor

## Requisitos Básicos

	- RSIS0: El sistema estará desarrollado en C, C++, Rust o Python.
	- RSIS1: El sistema necesitará tener un cifrado asimétrico.
	- RSIS2: El sistema usará la tecnología de sockets para el envío y recepción de mensajes.


	- RS0: Un servidor representa una sala.
	- RS1: El servidor tendrá su propio logger, donde se encuentren todos los movimientos.
	- RS2: El servidor realizará un handshacke que constará como mínimo de dos pasos antes de poder añadir a un cliente a la sala.
	- RS3: EL servidor debe identificar a cada cliente con un nickname.


	- RC0: Los clientes solo pueden enviar mensajes de texto. Éstos no tienen límite.
	- RC1: El cliente será una aplicación que puede ser tanto una CLI, TUI o GUI.
	- RC2: Las personas, una vez abren su cliente, tendrán un menú con diferentes opciones.
		- R2.0: Permitirá añadir nuevos servidores.
		- R2.1: Permitirá usar un servidor.
		- R2.2: Permitirá escoger un servidor como predeterminado.
	- RC3: El usuario podrá volver al menú con una combinación de teclas.
	- RC4: El cliente tendrá un banner en ASCII cada vez que se abra el software.
	- RC5: Toda configuración del cliente será guardada en un fichero de configuración.
	- RC6: El cliente debe tener un fichero de configuración por defecto.
