# intentiq

Een AI-gedreven cybersecurity CLI-tool geschreven in Rust die systeemlogs analyseert en vertaalt naar menselijke intentie.

Waar traditionele monitoring stopt bij _"wat is er gebeurd?"_, gaat intentiq een stap verder: **"waarom gebeurt dit en met welke intentie?"**

---

## Inhoudsopgave

- [Opdracht 1 - Plan van Aanpak](#opdracht-1---plan-van-aanpak)
- [Opdracht 2 - Technisch Ontwerp](#opdracht-2---technisch-ontwerp)
- [Installatie en gebruik](#installatie-en-gebruik)

---

## Opdracht 1 - Plan van Aanpak

### Wat houdt de opdracht in?

De opdracht is het bouwen van een CLI-tool die ruwe Linux-systeemlogs inleest, parsed naar gestructureerde events, deze groepeert per actor (IP-adres of gebruiker) en vervolgens de intentie van die actor classificeert. Denk aan het detecteren van reconnaissance, credential access, privilege escalation, persistence en data exfiltration.

De tool combineert twee lagen van intelligentie:
1. Een **rule-based systeem** dat snel en deterministisch bekende aanvalspatronen herkent
2. Een **LLM via API** voor complexere of ambigue situaties waar regels alleen niet volstaan

### Stappen om het systeem te bouwen

1. Repository en projectstructuur opzetten
2. Plan van Aanpak en Technisch Ontwerp schrijven (README)
3. Log parser bouwen voor alle ondersteunde logformaten
4. Uniform event model implementeren
5. Sessie- en actorgroepering implementeren
6. Rule-based intent detection implementeren
7. MongoDB integratie voor event opslag
8. LLM-integratie via API voor hybride classificatie
9. CLI output en rapportage bouwen
10. Documentatie afwerken en presentatie voorbereiden

### Tools en technologieën

| Component | Technologie | Reden |
|---|---|---|
| Programmeertaal | Rust | Performance, memory safety, ideaal voor CLI tooling |
| Log parsing | Regex via `regex` crate | Flexibel parsen van verschillende logformaten |
| Opslag | MongoDB via Docker | Persistentie van events voor historische analyse |
| Intent detection (regel) | Rule-based engine in Rust | Snel, deterministisch, geen externe afhankelijkheid |
| Intent detection (AI) | LLM via API (bijv. Claude) | Complexe patronen die regels niet vangen |
| CLI | Rust `std` + `clap` crate | Argumentverwerking en gebruiksvriendelijke interface |
| Containerisatie | Docker | MongoDB en eventueel Qdrant draaien lokaal |

### Implementatieaanpak per onderdeel

**Log parsing:** Per logtype (auth.log, syslog, ufw.log, nginx, etc.) komt er een eigen parser die met regex de relevante velden extraheert en omzet naar een uniform `Event` struct.

**Sessie- en actorgroepering:** Events worden gegroepeerd op IP-adres of gebruikersnaam binnen een configureerbaar tijdsvenster. Dit geeft een gedragssequentie per actor.

**Rule-based detection:** Een set van regels controleert patronen zoals "meer dan 10 failed logins gevolgd door een succesvolle login" of "sudo-gebruik direct na SSH-login". Elke regel levert een intentielabel en een confidence score op.

**LLM-integratie:** Als de rule-based engine geen duidelijke conclusie kan trekken (lage confidence), wordt de gedragssequentie als tekst naar een LLM-API gestuurd. Het LLM classificeert de intentie en geeft een uitleg.

**MongoDB:** Alle geparsde events worden opgeslagen in MongoDB zodat er later queries op kunnen worden uitgevoerd en historische trends zichtbaar worden.

**CLI output:** De tool geeft een leesbaar rapport in de terminal met actor, intentie, confidence en bewijs.

### Evaluatie van prestaties

- Unit tests per parser (valideert correcte extractie van velden)
- Unit tests per detectieregel (valideert correcte intentieclassificatie)
- Handmatige evaluatie op de meegeleverde dataset uit de case
- Vergelijking van rule-based output vs. LLM output op dezelfde input

---

## Opdracht 2 - Technisch Ontwerp

### Functionele eisen

- De tool leest minimaal de volgende logbestanden in: `auth.log`, `syslog`, `kern.log`, `nginx/access.log`, `nginx/error.log`, `ufw.log`
- Elk event wordt omgezet naar een uniform model met timestamp, source, user, ip, event_type en message
- Events worden gegroepeerd per actor op basis van IP of gebruikersnaam
- De tool detecteert minimaal de volgende intenties: `reconnaissance`, `credential_access`, `privilege_escalation`, `persistence`, `data_exfiltration`
- De tool geeft een CLI-rapport met actor, intentie, confidence en bewijs
- Events worden opgeslagen in MongoDB

### Niet-functionele eisen

- De tool is geschreven in Rust
- De tool draait als single binary via `cargo run`
- MongoDB draait lokaal via Docker
- De tool is uitbreidbaar: nieuwe logformaten en regels kunnen eenvoudig worden toegevoegd

### Architectuur (ASD - modulaire decompositie)

```
intentiq
├── main.rs                  # Entrypoint, CLI argumenten, orchestratie
├── lib.rs                   # Publieke API van de library
├── parser/
│   ├── mod.rs               # Parser module
│   ├── auth.rs              # Parser voor auth.log
│   ├── syslog.rs            # Parser voor syslog
│   ├── ufw.rs               # Parser voor ufw.log
│   └── nginx.rs             # Parser voor nginx logs
├── models/
│   └── event.rs             # Event struct (uniform model)
├── grouping/
│   └── mod.rs               # Actorgroepering en sessies
├── detection/
│   ├── mod.rs               # Detection orchestratie
│   ├── rules.rs             # Rule-based engine
│   └── llm.rs               # LLM API integratie
├── storage/
│   └── mongo.rs             # MongoDB connectie en opslag
└── output/
    └── report.rs            # CLI rapportage
```

### Dataflow

```
Logbestanden
    │
    ▼
[Parser module]
    │  Event { timestamp, source, user, ip, event_type, message }
    ▼
[Grouping module]
    │  Actor { ip/user → Vec<Event> }
    ▼
[Detection module]
    ├── Rule-based engine → IntentResult { intent, confidence, evidence }
    └── LLM API (indien confidence < drempel)
    │
    ▼
[Storage module] ──► MongoDB
    │
    ▼
[Output module]
    │
    ▼
CLI rapport
```

### Event model

```rust
pub struct Event {
    pub timestamp: String,
    pub source: String,
    pub user: Option<String>,
    pub ip: Option<String>,
    pub event_type: String,
    pub message: String,
}
```

### Intent output model

```rust
pub struct IntentResult {
    pub actor: String,
    pub intent: String,
    pub confidence: f64,
    pub evidence: Vec<String>,
}
```

### Database ontwerp (MongoDB)

**Collection: `events`**

```json
{
  "_id": "ObjectId",
  "timestamp": "2026-05-06T09:12:01Z",
  "source": "auth.log",
  "user": "john",
  "ip": "192.168.1.10",
  "event_type": "ssh_login_success",
  "message": "Accepted password for john from 192.168.1.10 port 53422 ssh2"
}
```

**Collection: `intent_results`**

```json
{
  "_id": "ObjectId",
  "actor": "185.220.101.12",
  "intent": "credential_access",
  "confidence": 0.87,
  "evidence": [
    "45 failed logins",
    "successful login",
    "sudo access"
  ],
  "analyzed_at": "2026-05-06T09:35:00Z"
}
```

### Detectieregels (initiële set)

| Patroon | Intent | Confidence |
|---|---|---|
| >10 failed logins van zelfde IP | `reconnaissance` | 0.75 |
| Failed logins gevolgd door succesvolle login | `credential_access` | 0.85 |
| SSH login → sudo → onbekend binary | `privilege_escalation` + `persistence` | 0.90 |
| Grote bestandsoverdracht naar extern IP | `data_exfiltration` | 0.88 |
| >50 requests/seconde van zelfde IP | `reconnaissance` (DDoS) | 0.80 |

---

## Installatie en gebruik

> Wordt aangevuld tijdens de realisatiefase.

```bash
# Vereisten
# - Rust (via rustup)
# - Docker (voor MongoDB)

# MongoDB starten
docker run -d -p 27017:27017 --name intentiq-mongo mongo

# Project bouwen en uitvoeren
cargo run -- --log /var/log/auth.log
```

---

## Voortgang

Zie de [GitHub Project Roadmap](../../projects) voor de actuele planning en issues.