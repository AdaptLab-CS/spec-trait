## Plan: Valutazione Completa (Prestazioni, Compilazione, Binario, Mantenibilità)

Obiettivo: aggiungere una valutazione forte e multidimensionale per la rebuttal senza toccare il codice esistente, creando una nuova area autonoma in root per benchmark sintetici (runtime, build time, binary size) e analisi grafica in notebook. Approccio: 4 pilastri di misurazione con Criterion + cargo timing + objdump/strip per tagli binari + metriche code complexity.

---

## Pilastro 1: Prestazioni Runtime

1. Implementare benchmark runtime in crate dedicato usando Criterion con confronto esplicito tra baseline generica e versione specializzata equivalente.
2. Includere almeno 3 famiglie di scenari: dispatch hot-path, vincoli trait con branching di tipo, combinazioni parametriche ad alta cardinalità; ogni famiglia con input size crescente.
3. Garantire fairness: stessa semantica, stessi dati di input, stessa configurazione compilatore, black_box e setup separato dal tratto misurato.
4. Salvare output grezzi Criterion e produrre un CSV normalizzato per notebook (tempo medio, deviazione standard, speedup, intervalli di confidenza).

---

## Pilastro 2: Costo di Compilazione

1. Misurare tempo totale di build per baseline vs specializzato usando `cargo build --timings` (o wrapper custom con `time` command).
2. Includere sia build clean che build incrementale per rappresentare workflow realistico.
3. Catturare numero di unit compilate, cache hit/miss patterns (tramite /target/debug/deps e loro timestamp).
4. Produrre tabella comparativa: build time baseline, build time specializzato, delta percentuale, impatto su CI.

---

## Pilastro 3: Dimensione Binario

1. Misurare binary size finale per baseline vs specializzato (cargo build --release, poi strip e objdump).
2. Scomporre per sezione: .text, .rodata, .data, debug symbols.
3. Analizzare monomorphization bloat: contare number of concrete instantiations per generic type in baseline vs specializzato.
4. Quantificare code duplication via simhash/similarity metrics su binario.

---

## Pilastro 4: Mantenibilità (Proxy mediante Code Metrics)

1. Contare numero di specialization hints (#when[], spec! macro) nel codice specializzato.
2. Misurare complessità ciclomica prima/dopo per file interessati (tramite cargo-clippy --message-format=json o radon per Python wrapper).
3. Contare linee di codice boilerplate generato dalle macro (confronto file sorgente vs expanded con cargo expand).
4. Raccogliere feedback qualitativo: lista di casi dove specializzazione è banale (1-3 linee) vs casi complessi.

---

## Steps Implementativi

**Fase 1 - Struttura eval isolata**
- Creare una nuova directory di valutazione in root (spec-trait-eval/) con sottocartelle per: crate benchmark, output raw, output normalizzato (CSV), notebook, script orchestrazione.
- Definire un README locale con protocollo sperimentale (hardware, toolchain, flag build, numero run, warmup, criterio di esclusione outlier).

**Fase 2 - Runtime benchmark (Criterion)**
- Implementare crate Cargo dedicato con 20 benchmark sintetici coprendo i 4 pilastri di metrica.
- Setup, run e raccolta dati Criterion.

**Fase 3 - Build time measurement**
- Script bash che esegue build clean e incrementale, cattura tempo con `time` e strutture /target/debug/deps.
- Genera CSV: build_config, total_time_s, num_units, cache_info.

**Fase 4 - Binary size analysis**
- Script bash che fa strip, objdump, calcola sezioni, stima monomorphization bloat.
- Genera CSV: binary_baseline_kb, binary_specialized_kb, section_breakdown, deduplicated_size.

**Fase 5 - Code complexity metrics**
- Script Python che conta #when[], spec!, estrae complessità ciclomica da clippy JSON, chiama cargo expand e conta linee boilerplate.
- Genera CSV: num_specialization_hints, cyclomatic_baseline, cyclomatic_specialized, lines_boilerplate.

**Fase 6 - Notebook aggregazione**
- Notebook Python (pandas + seaborn + matplotlib) che importa tutti i CSV dai pilastri.
- Genera figure: 4 subplot (uno per pilastro), tabella riassuntiva consolidata, narrative conclusiva con trade-off.
- Output: HTML report e PNG/PDF figure per paper.

---

## Relevant Files (Solo Riferimento, Nessuna Modifica)

Oltre che al crate [spec-trait-bin](spec-trait-impl/crates/spec-trait-bin) e tutti gli altri crate esistenti, i file di riferimento per formato e stile di pipeline/reporting sono:

- [spec-trait-inst/tests/scripts/analysis.sh](spec-trait-inst/tests/scripts/analysis.sh) — stile pipeline/reporting già usato nel progetto.
- [spec-trait-inst/tests/scripts/summary-90.csv](spec-trait-inst/tests/scripts/summary-90.csv) — formato tabellare da prendere come riferimento.
- [spec-trait-inst/src/analysis/sti_analysis/mod.rs](spec-trait-inst/src/analysis/sti_analysis/mod.rs#L316) — esempio di metriche tempo/memoria già presenti.

---

## Verification

1. Due run completi indipendenti sulla stessa macchina: verificare stabilità della mediana per scenario.
2. Per ogni benchmark: assert di equivalenza funzionale baseline vs specializzato.
3. Rigenerazione notebook da zero da output raw.
4. Verifica finale che nessun file preesistente sia stato modificato.

---

## Decisions

- Scope approvato: solo aggiunte in spec-trait-eval.
- Engine benchmark: Criterion.
- Dataset iniziale: micro-benchmark sintetici.
- Notebook stack: pandas + seaborn + matplotlib.
- Fuori scope per ora: codebase grandi reali, integrazione CI estesa.

---

## Further Considerations

1. Numero scenari minimo raccomandato: 6-9 benchmark totali per mostrare sia casi favorevoli sia casi neutri.
2. Definire in anticipo una soglia di rilevanza pratica (es. speedup mediano >= 5%) per evitare cherry-picking.
3. Se i risultati includono regressioni, riportarle esplicitamente come boundary condition per aumentare credibilità.
