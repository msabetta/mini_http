# Mini HTTP Server

Un **server HTTP ultra-leggero per API interne** scritto in Rust.
Il progetto è pensato come **esperimento educativo e base per microservizi interni**, con focus su:

* performance
* semplicità
* modularità
* configurazione esterna

Questo server dimostra come costruire un backend HTTP minimale usando il runtime async di Rust.

---

# 🚀 Features

* Server HTTP asincrono
* Router semplice e modulare
* Middleware di logging
* Configurazione tramite file TOML
* Architettura estensibile

---

# 🧱 Tecnologie utilizzate

* **Rust**
* Runtime async: **tokio**
* HTTP engine: **hyper**
* Serializzazione: **serde**
* Config parser: **toml**

---

# 📂 Struttura del progetto

```
src/
 ├── main.rs          # Entry point del server
 ├── router.rs        # Gestione routing delle API
 ├── middleware.rs    # Middleware (logging)
 └── config.rs        # Configurazione server

config.toml           # Configurazione del server
Cargo.toml
```

---

# ⚙️ Installazione

### 1️⃣ Clonare il repository

```bash
git clone https://github.com/msabetta/mini_http.git
cd mini-http
```

### 2️⃣ Installare Rust

Se non hai Rust installato:

```bash
curl https://sh.rustup.rs -sSf | sh
```

---

# ▶️ Avviare il server

```bash
cargo run
```

Output:

```
Server running on http://127.0.0.1:8080
```

---

# 🧪 Test delle API

### Ping endpoint

```bash
curl http://localhost:8080/ping
```

Risposta:

```
pong
```

---

### Health check

```bash
curl http://localhost:8080/health
```

Risposta:

```
OK
```

---

# ⚙️ Configurazione

Il server utilizza un file `config.toml`.

Esempio:

```toml
[server]
port = 8080

[ratelimit]
requests_per_minute = 100
```

---

# 🧠 Architettura

Il flusso di una richiesta è:

```
Client Request
      ↓
HTTP Server (hyper)
      ↓
Router
      ↓
Middleware (logging)
      ↓
Endpoint Handler
      ↓
Response
```

Questo design permette di aggiungere facilmente nuovi middleware o endpoint.

---

# 📌 Possibili miglioramenti

* Rate limiting
* Middleware stack configurabile
* Supporto JSON automatico
* Endpoint `/metrics`
* Logging strutturato
* Supporto TLS
* Graceful shutdown

---

# 🎯 Scopo del progetto

Questo progetto è stato sviluppato per:

* imparare networking asincrono in Rust
* capire come funziona un server HTTP sotto il cofano
* sperimentare architetture backend leggere

---

# 🤝 Contributi

Contributi, suggerimenti e pull request sono benvenuti.

---

# 📜 Licenza

MIT License