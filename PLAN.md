# Vuxe — Build Plan

> A FOSS, privacy-first daily audio briefing app. Users own their data, bring their own API keys, and get a clean solo-host podcast every morning — plus on-demand DeepCasts on any topic. Licensed under AGPL 3 — the SaaS loophole is closed by design.

---

## Guiding Principles

- **User sovereignty first.** Vuxe never stores API keys in plaintext, never trains on user data, and never calls an LLM or TTS provider on behalf of the user without their explicit credential.
- **Minimal surface area.** Three screens. Do one thing exceptionally well before expanding scope.
- **Ship early, iterate fast.** A working briefing pipeline on day one beats a perfect app on day ninety.
- **FOSS all the way.** Hosted on Codeberg under AGPL 3. No proprietary dependencies in the critical path where avoidable.
- **Self-hosting is a first-class feature.** Anyone with Docker Compose should be able to run Vuxe on a homelab or a cheap VPS.

---

## The Stack

| Layer | Technology | Why |
|---|---|---|
| Mobile | KMP + Compose Multiplatform | Single codebase, Android + iOS |
| Backend | Rust + Axum | Fully self-hostable, fast, no proprietary dependencies |
| Database | Postgres | Battle-tested, self-hosters already know it |
| Auth (hosted) | Clerk + Google SSO | Fast to wire up for the managed hosted version |
| Auth (self-hosted) | Any OIDC provider (Keycloak, Authentik, etc.) | Self-hosters bring their own identity provider |
| LLM | OpenRouter (user's own key) | Model-agnostic, user controls cost and model choice |
| TTS | ElevenLabs (user's own key) | High quality solo voice output |
| News | RSS / GNews | No proprietary dependency for headlines |
| Weather | Open-Meteo | Free, EU-based, no API key required |
| Self-hosting | Docker Compose | Single file, runs anywhere |
| Hosted version | Fly.io or Railway | Long-running Rust process friendly |
| Code | Codeberg | FOSS home, CI/CD via Forgejo Actions |

---

## What Vuxe Does

### Daily Briefing
Every morning, Vuxe generates a personalised ~5 minute solo-host audio briefing covering the user's chosen news topics and local weather. It can fire automatically at a scheduled time or be triggered on demand. The briefing is written by an LLM using the user's own OpenRouter key, then converted to audio by ElevenLabs using the user's own ElevenLabs key. Vuxe orchestrates the pipeline — nothing more.

### DeepCasts
On demand, a user can type any topic or question and receive a focused ~5 minute audio episode on it within seconds. Same pipeline, different prompt. Think of it as a podcast episode you summon from thin air.

---

## Core Features

- Personalised daily briefing — news topics and weather, ~5 minutes, solo host voice
- On-demand DeepCasts for any topic
- Automatic scheduling or manual trigger
- User-configurable voice via ElevenLabs
- User-configurable news topics and language
- Bring your own API keys — Vuxe never pays your LLM or TTS bills
- API keys encrypted at rest, write-only from the client, never logged
- Full self-hosting support via Docker Compose
- Configurable backend URL in the app for self-hosters

---

## Self-Hosting

Anyone can run Vuxe themselves. The backend and database ship as a single `docker-compose.yml`. Self-hosters configure their own OIDC identity provider (Keycloak, Authentik, Kanidm, or anything standards-compliant) and point the mobile app at their own instance via a custom server URL on the login screen. Full documentation will be provided.

---

## Build Phases

### Phase 1 — Foundation
- Repository setup on Codeberg, CI/CD via Forgejo Actions
- Scaffold Rust + Axum backend and Postgres
- Auth working end-to-end: Clerk for hosted, OIDC for self-hosted
- Scaffold KMP project, Android + iOS targets
- Docker Compose self-hosting working from day one

### Phase 2 — Briefing Pipeline
- Weather and news data gathering
- LLM script generation via OpenRouter
- Audio generation via ElevenLabs
- Full briefing pipeline wired together and testable

### Phase 3 — Mobile: Home Screen
- Today screen with briefing status and audio player
- Manual briefing trigger
- End-to-end on a real device

### Phase 4 — DeepCasts + Settings
- DeepCasts screen and generation pipeline
- Settings: API keys, voice, topics, location, schedule
- Onboarding flow for new users
- Automatic briefing scheduling

### Phase 5 — iOS + Polish
- Verify Compose Multiplatform on iOS
- Audio playback on iOS
- Error states, loading states, empty states
- App icon, splash, branding

### Phase 6 — Launch
- Self-hosting documentation
- Privacy policy
- Google Play listing
- Apple App Store listing
- F-Droid and IzzyOnDroid submission
- Announce on Mastodon, Bluesky, and FOSS communities

---

## Open Questions

1. **Audio playback on iOS via Compose Multiplatform** — the biggest technical unknown. Worth a proof-of-concept spike before building everything else around it.
2. **ElevenLabs audio storage** — generated audio URLs may expire. Decide early whether to cache audio files locally (natural for self-hosters) or regenerate on demand.
3. **Custom server URL in the app** — self-hosters need to point the mobile app at their own backend. This touches the login screen UX and app store listing and should be designed early.
4. **F-Droid reproducible builds** — KMP + Compose Multiplatform reproducibility on Android needs verifying early, as Kotlin toolchain versions can affect this.

---

## Success Criteria for v1

- A user can sign up, add their API keys, set their topics and location, and receive a ready-to-play ~5 minute morning briefing
- A user can generate a DeepCast on any topic in seconds
- The app works on both Android and iOS from a single KMP codebase
- Anyone can self-host the full stack with Docker Compose
- The full source is on Codeberg under AGPL 3
- The app is listed on F-Droid or IzzyOnDroid

---
