# Gestión de Tareas con API REST en Rust

Debes construir una API REST en Rust utilizando Actix Web para gestionar tareas. La API permitirá crear, listar y actualizar tareas. El dominio es la gestión de tareas en un entorno de proyecto de software, donde los desarrolladores pueden crear nuevas tareas, ver la lista de tareas existentes y actualizar el estado o detalles de una tarea.

## Informacion General

| Campo | Valor |
|-------|-------|
| **Tema** | API REST de tareas con Rust y Actix |
| **Nivel** | senior-l2 |
| **Tipo** | practical |
| **Tiempo estimado** | 10 horas |

## Fases del Reto

### Fase 0: Configuración del Proyecto

**Objetivo:** Obtener el proyecto base funcional enviando el Código Base a un asistente de IA, que lo analizará, corregirá errores y entregará cada archivo con su ruta.

**Tiempo estimado:** 15-30 minutos

**Instrucciones:**

- Asegúrate de tener instalado para ejecutar el proyecto: Un IDE o editor de código.
- Copia todo el contenido del campo **Código Base** de este reto — incluyendo el texto de instrucciones que aparece al inicio.
- Abre un asistente de IA (Claude en claude.ai, ChatGPT o Gemini — se recomienda Claude), pega el contenido copiado en el chat y envíalo.
- El asistente analizará los archivos, corregirá errores y entregará cada archivo con su ruta. Guárdalos en la carpeta donde quieras trabajar, respetando las rutas.
- Verifica que el proyecto arranca sin errores.

**Entregable:** El proyecto compila/arranca sin errores.

<details>
<summary>Pistas de conocimiento</summary>

- Copia el Código Base completo incluyendo el texto de instrucciones al inicio — esas instrucciones le indican al asistente exactamente qué hacer con los archivos.
- Si el asistente no entrega todos los archivos al terminar el análisis, escríbele: "entrégame todos los archivos con su ruta".
- Si el proyecto tiene errores al arrancar, comparte el mensaje de error con el mismo asistente para que lo corrija.

</details>

### Fase 1: Definición y Creación de Endpoints

**Objetivo:** Definir y crear los endpoints REST necesarios para crear, listar y actualizar tareas.

**Tiempo estimado:** 4 horas

**Instrucciones:**

- Identificar los endpoints necesarios para la gestión de tareas.
- Definir la estructura de datos para las tareas.
- Implementar los endpoints para crear, listar y actualizar tareas.

**Entregable:** Endpoints REST funcionales para crear, listar y actualizar tareas.

<details>
<summary>Pistas de conocimiento</summary>

- Considera los métodos HTTP apropiados para cada acción (POST para crear, GET para listar, PUT para actualizar).
- Piensa en la representación de datos adecuada para las tareas.

</details>

### Fase 2: Validación y Manejo de Errores

**Objetivo:** Implementar validaciones y manejo de errores en los endpoints.

**Tiempo estimado:** 3 horas

**Instrucciones:**

- Agregar validaciones para asegurar que los datos de las tareas sean correctos.
- Implementar manejo de errores para los casos en que las validaciones fallen o cuando ocurran errores internos.

**Entregable:** Endpoints REST con validaciones y manejo de errores implementados.

<details>
<summary>Pistas de conocimiento</summary>

- Considera los posibles errores y cómo comunicarlos al cliente.
- Piensa en las validaciones necesarias para asegurar la integridad de los datos.

</details>

### Fase 3: Optimización y Documentación

**Objetivo:** Optimizar el rendimiento de los endpoints y documentar la API.

**Tiempo estimado:** 3 horas

**Instrucciones:**

- Identificar y optimizar posibles puntos de rendimiento en los endpoints.
- Documentar los endpoints utilizando OpenAPI.

**Entregable:** Endpoints REST optimizados y documentación de la API utilizando OpenAPI.

<details>
<summary>Pistas de conocimiento</summary>

- Considera técnicas de optimización como el uso de caché o la minimización de la carga de datos.
- Piensa en cómo documentar claramente los endpoints para que otros desarrolladores puedan usarlos.

</details>

## Dimensiones Evaluadas

- **queEs**: ¿Qué son los endpoints REST y cómo se usan en la gestión de tareas?
- **paraQueSirve**: ¿Para qué sirven las validaciones y el manejo de errores en los endpoints?
- **comoSeUsa**: ¿Cómo se usan los métodos HTTP en los endpoints para crear, listar y actualizar tareas?
- **erroresComunes**: ¿Cuáles son los errores comunes que pueden ocurrir al implementar una API REST y cómo se manejan?
- **queDecisionesImplica**: ¿Qué decisiones implica la optimización de los endpoints y la documentación de la API?

## Criterios de Evaluacion

- Implementación correcta de los endpoints REST para crear, listar y actualizar tareas.
- Validaciones y manejo de errores adecuados en los endpoints.
- Optimización del rendimiento de los endpoints.
- Documentación clara y completa de la API utilizando OpenAPI.

## Como trabajar con un asistente de IA

Hay dos caminos, elegi uno:

- **AGENTS.md** (recomendado) — instrucciones nativas del repo. Abri esta carpeta con tu agente local (Claude Code, Cursor, Codex, Copilot, Gemini) y las carga solo. Sabe que archivos faltan y con que comando se verifica, y completa el scaffold escribiendo en disco.
- **PROMPT_MEJORA.md** — para copiar y pegar en un chat (claude.ai, ChatGPT) si no tenes un agente en el IDE. Declara lo mismo que AGENTS.md; los archivos que te devuelva el chat los guardas en esta carpeta respetando las rutas.

Ninguno de los dos resuelve las fases del reto: eso es tu trabajo.

## Verificacion

El proyecto esta listo para trabajar cuando este comando corre sin errores:

```bash
cargo check
```

---

*Reto generado automaticamente por Challenge Generator - Pragma*
