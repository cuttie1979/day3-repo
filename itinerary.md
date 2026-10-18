# Day 3 — itinerary

Mi készüljön el, mi legyen tesztelve, mit kell mérni. Sorrendben, pipálhatóan.

## Hol mi készül

| Hol | Mi | Állapot |
|---|---|---|
| a Qubes qube | a 14 deep research futás, a memória | todo |
| ez a repo | a kutatás markdownjai, a demó, a promptok | részben kész |
| a site repo | a deck (kész) + az engine bővítés (commitolva, nincs pusholva) | részben kész |
| a prezentáló laptop | a nap futtatása | todo |

---

## 1 · Kutatás — a qube-ban

14 futás, `research-plan.md` szerint. Mindegyikhez a parancs kész, csak bemásolni kell.

- [ ] `feynman setup` kész, a DeepSeek provider bejelentkezve
- [ ] `feynman doctor` tiszta
- [ ] `FEYNMAN_TELEMETRY=off` (vagy `DO_NOT_TRACK=1`) beállítva
- [ ] `OPENALEX_API_KEY`, `SEMANTIC_SCHOLAR_API_KEY`, `CROSSREF_MAILTO` beállítva
- [ ] a 14 futás lefutott

**Minden futás után ezt nézd meg — ez a kritikus ellenőrzés:**

| # | Ellenőrzés | Miért |
|---|---|---|
| 1 | keletkezett-e fájl az `outputs/` alatt | egy flash-tier modell hajlamos fájl nélkül, csak szövegben válaszolni |
| 2 | van-e provenance fájl | a `verifier` és a citation-check nyoma |
| 3 | mennyi ideig futott | ebből lesz a napon az élő demó időzítése |
| 4 | hány forrást idéz | ha 3 alatt van, a kérdés túl szűk volt |

| Kérdés | Futott | `outputs/` | Provenance | Idő | Források |
|---|---|---|---|---|---|
| 1 Karpathy / CLAUDE.md | | | | | |
| 2 Musk / first principles | | | | | |
| 3 követelmény → technikai | | | | | |
| 4 acceptance criteria kérdések | | | | | |
| 5 quality gate-ek | | | | | |
| 6 regulated traceability | | | | | |
| 7 engineering dokumentumok | | | | | |
| 8 test-first + agentek | | | | | |
| 9 test adequacy | | | | | |
| 10 formázás/lint hatás | | | | | |
| 11 security-by-design | | | | | |
| 12 kontextusfájl hatékonyság | | | | | |
| 13 AI-kód hibái | | | | | |
| 14 a 100× állítás | | | | | |

**Kimenet:** a markdownok a `research/` mappába, `<slug>.md` néven.

---

## 2 · Memória — a kutatás betöltése

- [ ] a markdownok a `research/` mappában (qube → `qvm-copy` → `QubesIncoming/` → ide)
- [ ] minden fájl betöltve a MemPalace-be, **dátummal és forrással**
- [ ] a best practice-ek külön bejegyzésként (a workflow kapuihoz)
- [ ] a #14 (a 100×) eredménye megjelölve: állítás vagy cáfolat

| AC | Ellenőrzés |
|---|---|
| minden bejegyzésen van dátum | `mempalace_kg_query` visszaadja az érvényességi intervallumot |
| minden bejegyzésen van forrás | a provenance fájlra hivatkozik |
| visszakereshető | egy keresés a wing/room-ra visszaadja |
| nincs bejegyzés forrás nélkül | ami nincs, az **hipotézis**, nem gyakorlat |

**Miért most:** a 17. dia promptja (WORKFLOW.md) a memóriából dolgozik. Ha a memória üres, az agent kitalálja a kapukat — és pont az a nap tanulsága, hogy azt nem szabad.

---

## 3 · A demó — `rust-aes-demo/`

Kész és ellenőrizve ezen a gépen. A qube-ban **újra ellenőrizni kell**.

- [ ] Rust toolchain a qube-ban (`rustc --version`)
- [ ] `~/.cargo` és `target/` előmelegítve (a napon ne fordításra várjunk)
- [ ] `./demo.sh` lefut, mind a nyolc lépés

| AC | Elvárt |
|---|---|
| `cargo test --test roundtrip` | 6/6 zöld — **a naivon is** |
| `cargo test --test properties` | 0/4 — mind a négy bukik |
| `--features hardened --test roundtrip` | 6/6 zöld |
| `--features hardened --test properties` | 4/4 zöld |
| `cargo run --example key_debug` | a kulcs kiírva |
| `--features hardened --example key_debug` | `Key(Secret([REDACTED [u8; 32]]))` |
| `./mutate.sh` | 3 passed, 1 failed, és visszaállít |
| `cargo clippy` + `cargo fmt --check` | tiszta **mindkét** állapoton |

---

## 4 · A promptok — a demo környezetben

- [ ] a három létrehozó prompt (dia 15, 16, 17) lefutott
- [ ] a kalibrációs prompt (dia 60) lefutott

| AC | Ellenőrzés |
|---|---|
| a generált agentek **V2 szintaxist** használnak | `grep -c 'permissions:'` > 0 és `grep -c 'permission:'` == 0 |
| a lencsék nem persona-k | minden `wf-*` testében ott a `finding = …` és `evidencia = …` sor |
| egyik sem írhatja a mércét | a permissions listában `deny` a `.opencode/**` és a `WORKFLOW.md` fájlra |
| az orchestrator nem implementál | `edit` deny |
| a WORKFLOW.md kapui forrásoltak | minden kapu mellett memória-hivatkozás |

---

## 5 · A workflow — a nap végi szakasz

- [ ] `WORKFLOW.md` létrejött a projekt gyökerében
- [ ] az öt `wf-*` agent létrejött
- [ ] az orchestrator (Jarvis) létrejött globálisan

| AC | Elvárt |
|---|---|
| a review megtalálja a naiv hibáit | a hardkódolt IV, a hiányzó authentikáció, a rossz kulcs elfogadása, a kulcs kiírása |
| a review nem riaszt a megkeményítetten | nulla finding |
| minden finding forrásolt | memória-hivatkozás + annak forrása |
| az orchestrator javasol, nem ír | a memóriába nem ír közvetlenül |

**Ha a review nem találja meg mindet:** ez **eredmény**, nem hiba. A nap tanulsága épp ez: a mérce annyit ér, amennyi benne van.

---

## 6 · A site — a deck és az engine

- [x] a deck kész: 70 dia, 10 blokk, `prezzz check` zöld
- [x] a `guess` dia felületi szövegei és CSS-e
- [x] az engine bővítés: a `guess` renderelője megkapja a `lede`-t (commitolva)
- [ ] **döntés: push** — a site `main`-re push élőre megy; az engine megosztott repo
- [ ] az engine változás és a site változás **együtt** megy ki (különben az idézet eltűnik)

| AC | Elvárt |
|---|---|
| a projektoron a 3. dián látszik az idézet | pull-quote, a kérdés alatt |
| a telefonon is látszik | a kérdés alatt, félkövér |
| felfedés előtt nincs címke | egyik opción sem |
| felfedés után a helyes opció jelölt | „helyes válasz" pirula + a finding |

---

## 7 · Mit mérj a próbán

Ezekből lesznek a `notes`-ok valódi számai. A day2 notes-ai így épülnek.

| Mit | Hol mérhető | Hova kerül |
|---|---|---|
| a deep research futásideje | a qube | a 9. dia `Háttér:` sora |
| hány finding a naivon / a megkeményítetten | a kalibráció | a 61. dia |
| a `./demo.sh` futásideje | a qube | a 46. dia |
| a `cargo test --test properties` futásideje | a qube | a 49. dia |
| a teljes nap valós ideje blokkonként | a napi sáv (a késés/csúszás mutató) | a blokkok `notes`-a |
| az egyes szavazások részvétele | a moderálás | a szavazás-diák `notes`-a |

**A nap után:** a `say` szövegekbe azok a helyek, ahol a terem másképp reagált, mint vártad.

---

## 8 · A nap előtt — utolsó 24 óra

- [ ] a qube fel, hálózat, a Feynman bejelentkezve
- [ ] a kutatás markdownjai **letöltve** (tartalék, ha a hálózat elszáll)
- [ ] a Teams-link beállítva a moderálásban (`m` → Teams-link)
- [ ] **adatok törlése** (`RESET`) — tiszta lap a szavazásoknak
- [ ] a claim link megnyitva a prezentáló böngészőben (`aicode.page/3/claim/<kulcs>`)
- [ ] a projektor-ablak megosztva a Teamsben (ne a távirányító)
- [ ] a `rust-aes-demo` klónozva a prezentáló gépen, cache előmelegítve
- [ ] a remote nyitva a telefonon vagy egy másik ablakban

---

## Sorrend, ha csak a felét éred rá

1. **A 14 kutatás** — enélkül a 17. dia promptja nem működik, és a review-nak nincs mérőeszköze
2. **A memória betöltése** — ez a mérce
3. **A Rust demó ellenőrzése a qube-ban** — ez a nap kézzelfogható fele
4. **A promptok futtatása** — ez a nap legjobb demója
5. **A workflow és a kalibráció** — ez a csúcspont
6. A site push
