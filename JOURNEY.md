# Perjalanan Gravedigger

> Dari nama vigil → blight → rot → ossuary → gravedigger, dari AUR-only sampai
> multi-ekosistem dependency health scanner. Berawal dari iseng, berakhir di kuburan.

---

## 🥚 The Origin — Watchtower (pre-git)

Sebelum git, sebelum nama, ada satu file Rust yang iseng dibuat. Namanya belum
ada — cuma skrip kecil buat ngecek **AUR packages** yang udah basi. Mirip
[Arch Wiki: watchtower](https://wiki.archlinux.org/title/Watchtower), tapi
lebih sederhana. Jauh lebih sederhana.

---

## 🚀 v0.0.1 — "Vigil" (lahir)

**1a02eef** — `Initial commit: AUR package health scanner`

Nama pertama: **Vigil**. Kenapa Vigil? Karena tugasnya "ngawasin"
dependencies. Kayak jaga malem, tapi buat package manager.

Waktu itu cuma bisa:
- Scan AUR packages via `pacman -Qm`
- Cek `OutOfDate` flag dari AUR RPC
- Cek GitHub last commit (kalau URL-nya GitHub)

Udah. Sederhana. Tapi *it worked*.

Keluarga: `watchtower`
Nama github: `I-XXII-V/watchtower` → `I-XXII-V/vigil`

---

## 📦 v0.1.0 — Multi-ekosistem

**6a83d2f** — `feat: add cargo and npm support`

Vigil mulai gede. Dari cuma AUR, sekarang bisa:
- **Cargo** — scan `Cargo.lock`, fetch dari crates.io API
- **npm** — scan `package-lock.json`, fetch dari npm registry

Terus tambah lagi:

**270e688** — `feat: add PyPI, Go, and downstream (who-depends) support`
- **PyPI** — `poetry.lock` / `Pipfile.lock`
- **Go** — `go.mod`
- **who-depends** — liat siapa aja yang depend-to crate tertentu

**ed8d13d** — `feat: add --json/-j flag`
Output JSON buat di-pipe ke jq. Akhirnya bisa otomasi.

**e2cbdcd** — `feat: CVE scanning via OSV.dev + file-based cache layer`
Ngecek CVE lewat OSV.dev. Cache 6 jam biar gak kena rate limit.

**6f9bcfd** — `feat: --ci flag + --licenses flag`
CI mode: exit code 1 kalau ada yang mati atau kena CVE. License breakdown
buat legal yang tiba-tiba nanyain.

---

## 🏷️ v0.2.0 — AUR scoring fix + hijack detection

**786d9e3**

Dua terobosan besar:
1. **LastModified fallback** — kalau GitHub rate limited, pake timestamp
   PKGBUILD. Gak ada lagi ❓ cuma karena `GITHUB_TOKEN` gak diset.
2. **Hijack detection (🚩)** — kalau PKGBUILD diupdate recently (< 90 hari)
   tapi orphaned + popularitas rendah, flag sebagai hijack risk.

Summary jadi: `✅ 12 ⚠️ 5 🚩 2 🔴 1 🪦 0 ❓ 39`

**bf06e1f** — hijack counter masuk summary
**268ae24** — fix readme, registry scoring, hijack di JSON

---

## 🔧 v0.2.1 — Refactoring season

**fbc153a**

- Centralized date parsing (`types.rs`)
- Consistent stale reasons across all ecosystems
- Fix npm indentasi + edge cases
- Fix API error body leak
- Fix double GitHub call di `single_package_json`

---

## 🔧 v0.2.2 — ASCII art + massive refactoring

**d310ac8**

Banyak banget:
- ASCII art watchtower di `--help`
- SVG logo (gagal, dihapus)
- Centralized helpers
- Safe unwrap everywhere
- Code jadi lebih bersih

---

## 🔧 v0.2.3 — Go hijack, NPM provenance, OSV ecosystems

**ef9b00f**

Tiga fitur besar:
1. **Go hijack detection** — archived/404 GitHub repo flagged sebagai 🚩
2. **NPM provenance** — SLSA provenance attestation dari npm registry.
   Tau package ini dibangun di GitHub Actions atau dikitik manual.
3. **5 OSV ecosystems baru** — RubyGems, NuGet, Maven, Packagist, Hex.

---

## 🔧 v0.2.4 — Perbaikan + CI

**4c01ba5**

- Fix provenance caching (gak fetch ulang tiap kali)
- Fix JSON output untuk packages yang gagal fetch
- Fix negative display values
- Fix URL parsing
- Fix double API calls (shared client + cached GitHub data)
- Stale reason logic beneran

---

## ✨ Rename #1 — Name crisis: Vigil → Blight

**13db1dc** — `Rename Watchtower → Vigil`

Tunggu, ini rename pertama? Iya, dari Watchtower (nama pre-git) ke Vigil
ditandain di commit ini.

Tapi ternyata... **Vigil** itu nama yang kurang pas. Terlalu "heroik". Project
ini ngurusin dependencies yang mati, bukan jagain kota.

**7242ffa** — `chore: rename vigil → blight`

**Blight.** Busuk. Layu. Cocok. Dari vigil (jaga) jadi blight (penyakit
tanaman). Ini bukan tool buat jagain — ini tool buat ngecek apa yang udah
busuk.

---

## 🚀 v0.1.0 — First GitHub Release

**34f6fc5** — pkgbuild + GitHub Release

Tag: `v0.1.0`
Nama: `blight v0.1.0`

Padahal kita udah lewat v0.2.4 di Vigil. Tapi setelah rename, version
di-reset. Blight mulai dari v0.1.0 lagi. Fresh start.

Fitur yang dibawa:
- Health scoring (✅ ⚠️ 🔴 🪦 🚩 🚨)
- Multi-ekosistem: AUR, Cargo, npm, PyPI, Go
- `blight diff` — compare dependency health antar git ref
- CI mode (exit code 1)
- JSON output
- Sorted by severity, worst first

Build & test: **89/89 passed** (82 unit + 7 integration)

---

## 🚀 v0.1.0 — Wait, renaming AGAIN? Blight → Rot

**2bfa3da** — `chore: rename blight → rot`

Blight ternyata... ya gitu. Terlalu poetic. **Rot** lebih pendek, lebih
tegas, lebih ngena. Rot is rot.

Folder: `blight/` → `rot/`
GitHub: `I-XXII-V/blight` → `I-XXII-V/rot`

---

## ✨ UX Improvements

**ba7c89d** — `feat: sort output by severity, hide ❓ behind --verbose, show days since`

Tiga perubahan UX yang signifikan:
1. **Sort output by severity** — yang paling parah di atas (🪦 > 🔴 > ⚠️ > ✅)
2. **❓ hidden behind --verbose** — gak usah liat fetch errors tiap scan
3. **Days since** — tiap package nunjukin udah berapa hari gak update

---

## 🚀 v0.1.0 — Rename AGAIN??? Rot → Ossuary

**dcdb0ef** — `chore: rename rot → ossuary`

Ossuary? **Tempat nyimpen tulang.** Karena ini tool buat ngecek package
yang udah mati. Tulang berulang. Makin dalem makin dark.

Folder: `rot/` → `ossuary/`
GitHub: `I-XXII-V/rot` → `I-XXII-V/ossuary`

---

## 🚀 v0.1.0 — FINAL NAME: Ossuary → Gravedigger

**b6955ba** — `chore: rename ossuary → gravedigger`

Kok ganti lagi? Karena ossuary itu tempatnya. **Gravedigger** itu yang
nggali. Yang ngorek-ngorek. Yang nentuin mana yang masih hidup dan mana
yang udah mati.

Ini nama terakhir. **I promise.**

Folder: `ossuary/` → `gravedigger/`
GitHub: `I-XXII-V/ossuary` → `I-XXII-V/gravedigger`

---

## 🏷️ v0.1.1 — Edition 2024

**449737c** — `chore: bump to v0.1.1, upgrade to edition 2024`

Naik ke Rust edition 2024. Konsekuensi: clippy marah-marah.

**97546bf** — `chore: fix clippy::collapsible_if warnings (edition 2024)`

15 `collapsible_if` warnings. Semua dibenerin. CI hijau lagi.

---

## 📊 Timeline Lengkap

```
2026-06-18  Pre-git: Watchtower (AUR-only scanner)
2026-06-18  1a02eef  Initial commit: Vigil
2026-06-18  6a83d2f  feat: add cargo + npm support
2026-06-18  270e688  feat: add PyPI + Go + who-depends
2026-06-18  ed8d13d  feat: --json flag
2026-06-18  e2cbdcd  feat: CVE scanning via OSV.dev
2026-06-18  6f9bcfd  feat: --ci + --licenses flags
2026-06-19  786d9e3  v0.2.0: AUR scoring + hijack detection
2026-06-19  fbc153a  v0.2.1: Refactoring + fixes
2026-06-19  d310ac8  v0.2.2: ASCII art + massive refactoring
2026-06-20  ef9b00f  v0.2.3: Go hijack + NPM provenance + OSV ecosystems
2026-06-21  4c01ba5  v0.2.4: Fixes + CI improvements
2026-06-21  13db1dc  Rename: Watchtower → Vigil
2026-06-21  7242ffa  Rename: Vigil → Blight
2026-06-21  2bb9064  feat: diff subcommand
2026-06-22  ba7c89d  feat: sort by severity + --verbose + days since
2026-06-22  2bfa3da  Rename: Blight → Rot
2026-06-22  34f6fc5  GitHub Release v0.1.0
2026-06-22  dcdb0ef  Rename: Rot → Ossuary
2026-06-23  b6955ba  Rename: Ossuary → Gravedigger
2026-06-23  449737c  v0.1.1: Edition 2024
2026-06-23  97546bf  Fix clippy::collapsible_if
```

---

## 🧠 Total Rename Count

| # | From | To | Why |
|---|------|----|-----|
| 0 | Watchtower | Vigil | Nama pre-git → nama resmi pertama |
| 1 | Vigil | Blight | "Vigil" terlalu heroik |
| 2 | Blight | Rot | "Blight" terlalu puitis |
| 3 | Rot | Ossuary | "Rot" terlalu pendek |
| 4 | Ossuary | Gravedigger | *This is the final one* |

---

## ✨ Feature Evolution

```
AUR only
  ├── Cargo (Cargo.lock + crates.io API)
  ├── npm (package-lock.json + npm registry)
  ├── PyPI (poetry.lock / Pipfile.lock + pypi.org)
  ├── Go (go.mod + Go proxy)
  ├── who-depends (crates.io reverse dependencies)
  ├── diff (git ref comparison + health scoring)
  ├── CVE scanning via OSV.dev
  │     ├── crates.io
  │     ├── npm
  │     ├── PyPI
  │     ├── Go
  │     ├── RubyGems
  │     ├── NuGet
  │     ├── Maven
  │     ├── Packagist
  │     └── Hex
  ├── Hijack detection (Go, AUR)
  ├── NPM provenance (SLSA attestation)
  ├── --json output
  ├── --ci mode (exit 1)
  ├── --licenses breakdown
  ├── --stale filter
  ├── --verbose (show ❓)
  ├── Sort by severity
  ├── Days-since display
  └── Cache layer (file-based, 6h TTL)
```

---

## 📈 Stats (per v0.1.1)

| Metric | Value |
|--------|-------|
| Commits | 47 |
| Renames | 4 |
| Files | 16 source + config |
| Tests | 89 (82 unit + 7 integration) |
| Ecosystems | 5 (AUR, Cargo, npm, PyPI, Go) |
| OSV ecosystems | 9 |
| Star | 0 (bantu dong ⭐) |

---

## 🪦 The End

Gravedigger v0.1.1. Dari Vigil yang jaga, jadi Gravedigger yang gali.
Perjalanan nama yang mencerminkan perjalanan fungsi: makin lama makin dalem,
makin kelam, makin jujur tentang nasib dependencies yang udah mati.

> *"Some dependencies die quietly. Some need a gravedigger."*

---

*Made with ❤️ (and ☕ and 😤) by I-XXII-V*
