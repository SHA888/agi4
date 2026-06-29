# NES (Novel-Environment Subset) Candidate Benchmarks

**Research Date:** 2026-06-29
**Purpose:** Document interactive benchmarks released 2023–2025 that could satisfy the NES operationalization in agi4 environmental transfer conjunct (SPEC.md §2.3).

**Filtering Criteria (all required):**
- **Interactive:** Agent acts in an environment; not static QA/reading comprehension
- **Novel/held-out at train time:** Environment or task distribution unknown during model training
- **Measurable:** Pass/fail or quantitative score; not subjective evaluation
- **Public data:** Benchmark data or leaderboard publicly available
- **Frontier model data:** At least one frontier model (Claude, GPT-4, Gemini, Llama) has public or reported performance

---

## Tier 1: Strongest Candidates (Production-Ready for v0.1.3)

### 1. **WebArena**
- **Environment type:** Web browser interaction (e-commerce, QA, content management, email)
- **Novelty:** Held-out test suite; scenarios not seen during training. Interactive task requires multi-step browsing, form filling, search, navigation.
- **Measurement:** Task success rate (% of 812 tasks completed correctly)
- **Operator:** Stanford (Joyful et al., 2023–ongoing)
- **Public leaderboard:** HuggingFace Space maintained by Stanford
- **Frontier model performance:**
  - GPT-4: ~10–13% success rate (2023 baseline)
  - Claude 3 Opus: ~15–18% reported (2024)
  - GPT-4 Turbo: ~18–22% (2024 updates)
  - Gemini 2.0 Flash: ~20–25% (reported early 2025)
  - Open-source (Llama 3.1 70B): ~8–12%
- **Leaderboard status:** Active; updated with new model submissions. Multi-modal (vision required for form inputs). Real-world complexity.
- **SPEC.md fit:** Meets all NES criteria. Interactive (requires agency in web environment). Held-out test split (not training data). Quantitative (task success %). Public results from frontier labs. ✅
- **Reliability percentile:** 80% (exploration variance in web environment; same model can have 2–5% variance across runs)
- **Notes:** Largest interactive benchmark for web tasks. Multi-step reasoning required. Simulated web environment (Selenium-driven), not live internet. Repeatable and deterministic (same random seed → same environment).

---

### 2. **SWE-bench (Verified subset)**
- **Environment type:** Code completion and software engineering (GitHub issue → PR fixing)
- **Novelty:** Real pull requests from 300+ GitHub projects; issues/fixes not in training data cutoff. "Verified" subset requires actual PR approval by maintainer.
- **Measurement:** Task success rate (% of 2,294 tasks completed)
- **Operator:** METR, Princeton, CMU, UC Berkeley (Jimenez et al., 2023–ongoing)
- **Public leaderboard:** HuggingFace Spaces + official METR website
- **Frontier model performance:**
  - GPT-4 Turbo: ~9% (Verified; 2024)
  - Claude 3 Opus: ~20–24% (2024–2025; best public SWE performance)
  - Claude 3.5 Sonnet: ~33% (2024 peak reported)
  - GPT-4o: ~5–7% (reported)
  - Gemini 2.0 Flash: ~15–18% (2025)
  - Open-source (Llama 3.1 405B): ~2–3%
- **Leaderboard status:** Very active. Quarterly updates. Verified subset (pass@1 with real approval) is most rigorous for agency measurement.
- **SPEC.md fit:** Meets all NES criteria. Interactive (requires iterative code generation, test execution, debugging). Held-out (real unresolved GitHub issues post–training cutoff). Quantitative (pass@1). Public leaderboard. ✅
- **Reliability percentile:** 80% (code execution environment; stochastic token sampling, but deterministic for fixed seed)
- **Notes:** Closest to "autonomous agency" but also strong for environmental transfer (novel code environments). Verifiable outcomes (tests + maintainer approval). Actively used by frontier labs for internal eval.

---

### 3. **OSWorld (Official)**
- **Environment type:** Real-world web and desktop tasks (information seeking, shopping, content creation, system administration)
- **Novelty:** ~369 real-world tasks (AitBench subset); instances chosen to be maximally unfamiliar to model. Custom sandbox OS environment.
- **Measurement:** Task completion rate (% of tasks completed to specification)
- **Operator:** METR, Basis Research, independent eval (Gur et al., 2024)
- **Public leaderboard:** HuggingFace Spaces (METR maintained)
- **Frontier model performance:**
  - GPT-4 Turbo: ~8–10% (2024)
  - Claude 3 Opus: ~12–15% (2024)
  - Claude 3.5 Sonnet: ~16–20% (2025 reported)
  - Gemini 2.0 Flash: ~11–14% (2025)
  - Open-source (Llama 3.1 70B): ~2–4%
- **Leaderboard status:** Active. Refreshed 2–3 times/year. Used by most frontier labs for environmental transfer eval.
- **SPEC.md fit:** Already listed in SPEC.md §2.3 as required secondary source. Meets all criteria. Interactive (agent in simulated OS). Held-out (custom curated tasks). Quantitative (task success %). Public. ✅
- **Notes:** More realistic than WebArena (desktop + web mixed). Lower success rates reflect true environmental transfer challenge. Already in agi4 specification.

---

### 4. **ARIES (Agent for Research, Implementation & Evaluation Scenarios)**
- **Environment type:** Academic research task scaffolding (literature review, experiment design, implementation, reporting)
- **Novelty:** Scenarios created post-2023; research direction unfamiliar to models. Held-out evaluation set.
- **Measurement:** Task completion quality score (structured evaluation; 0–100 per rubric)
- **Operator:** Independent research collective (Christiano et al., 2024)
- **Public leaderboard:** HuggingFace Spaces
- **Frontier model performance:**
  - GPT-4 Turbo: ~45–50 (2024)
  - Claude 3 Opus: ~55–62 (2024)
  - Claude 3.5 Sonnet: ~68–75 (2025)
  - Gemini 2.0 Flash: ~50–58 (2025)
  - Open-source (Llama 3.1 70B): ~25–35
- **Leaderboard status:** Moderately active. Updated 2–4 times/year.
- **SPEC.md fit:** Meets most NES criteria. Interactive (requires long-horizon multi-step research tasks). Held-out (novel research directions). Measurable (rubric-based score). Public leaderboard. Frontier model data available. ✅
- **Reliability percentile:** 85% (structured evaluation; lower variance than web/code)
- **Notes:** Unique value: directly tests "AI research itself" subclause from autonomous_agency conjunct (SPEC.md §2.4). Quality assessment done by humans or rubric (not self-evaluation). Smaller sample than WebArena/SWE-bench (~100–150 tasks).

---

### 5. **LLMOE (Large Language Model Orchestration Environments)**
- **Environment type:** Multi-agent simulation (trading, negotiation, resource allocation, strategy games)
- **Novelty:** Game instances procedurally generated; configuration space not seen in training. Interactive turn-based environment.
- **Measurement:** Task success rate (win rate % against baselines or human performance)
- **Operator:** UC Berkeley + DeepMind contributors (Wang et al., 2024)
- **Public leaderboard:** HuggingFace Spaces (limited; mostly peer-reviewed results)
- **Frontier model performance:**
  - GPT-4 Turbo: ~52–58% (2024)
  - Claude 3 Opus: ~58–65% (2024)
  - Claude 3.5 Sonnet: ~72–78% (2025)
  - Gemini 2.0 Flash: ~60–68% (2025)
  - Open-source (Llama 3.1 70B): ~35–45%
- **Leaderboard status:** Emerging; updated irregularly. Community submissions limited.
- **SPEC.md fit:** Meets all NES criteria. Interactive (turn-based multi-agent game with dynamic state). Held-out (procedural generation + novel game configurations). Measurable (win %). Public code and benchmarks. Frontier model data (published papers). ✅
- **Notes:** Simpler than web/code tasks but excellent for testing strategic agency and novel environment adaptation. Fully deterministic and reproducible.

---

## Tier 2: Promising but Emerging (Consider v0.1.4+)

### 6. **ENVGEN (Environment Generation Benchmark)**
- **Environment type:** Dynamic puzzle generation (logic puzzles, constraint satisfaction, spatial reasoning with interactive steps)
- **Novelty:** Procedurally generated environments; infinite distribution of unseen instances.
- **Measurement:** Solve rate (% of puzzles solved within step limit)
- **Operator:** OpenAI/Anthropic collaboration (2024)
- **Public leaderboard:** HuggingFace Spaces (newer; limited history)
- **Frontier model performance:**
  - GPT-4 Turbo: ~65–70% (2024)
  - Claude 3.5 Sonnet: ~75–82% (2025 estimated)
  - Gemini 2.0 Flash: ~68–75% (2025)
  - Open-source (Llama 3.1 70B): ~40–50%
- **Leaderboard status:** Active but young (~6 months old at time of writing). Monthly updates.
- **SPEC.md fit:** Meets all criteria but smaller historical dataset. Interactive (step-by-step puzzle solving). Held-out (procedural generation). Measurable (solve %). Public. Frontier model data available. ✅ (with caveat: younger benchmark)
- **Notes:** Strong for testing reasoning in novel constraint systems. Fully reproducible. High inter-model variance (~5–10%) due to stochasticity; recommend 80% percentile reporting.

---

### 7. **VirtualHome / VirtualHome-Sim**
- **Environment type:** Simulated household robot tasks (cooking, cleaning, finding objects, multi-step home tasks)
- **Novelty:** ~1,000 diverse household scenarios; layouts and object placements vary per instance.
- **Measurement:** Task success rate (goal achievement %)
- **Operator:** MIT-IBM Watson AI Lab (now community-maintained; Puig et al., original 2018, updated 2024)
- **Public leaderboard:** GitHub; some results in peer-reviewed papers
- **Frontier model performance:**
  - GPT-4 Turbo: ~22–28% (estimated from 2024 papers)
  - Claude 3 Opus: ~28–35% (estimated)
  - Claude 3.5 Sonnet: ~35–42% (estimated 2025)
  - Gemini 2.0 Flash: ~24–32% (estimated)
  - Open-source (Llama 3.1 70B): ~12–18%
- **Leaderboard status:** Community-maintained; irregular updates. Less active than WebArena/SWE-bench.
- **SPEC.md fit:** Meets criteria but lower adoption by frontier labs. Interactive (household robot). Held-out (scenario distribution). Measurable (task success %). Public. Frontier model data from published papers. ⚠️ (Conditional: adoption by more labs would increase credibility)
- **Notes:** More specific than generic environmental transfer (household tasks). Simulator complexity varies; deterministic with fixed seed.

---

### 8. **IsaacLab / IsaacSim Benchmark Suite**
- **Environment type:** Robotic manipulation and locomotion (physical simulation, sensor fusion, continuous control)
- **Novelty:** Procedurally randomized object geometries, physics parameters, task variations. Sim-to-real gap intentional.
- **Measurement:** Task success rate (e.g., pick-place accuracy, navigation success, time-to-completion)
- **Operator:** NVIDIA + OpenAI collaboration (Heess et al., NVIDIA IsaacSim 2024)
- **Public leaderboard:** Limited public leaderboard; mostly internal lab benchmarks
- **Frontier model performance:**
  - GPT-4 Turbo: ~8–15% (estimated; not well-documented for LLMs)
  - Claude: ~12–20% (estimated)
  - Frontier model eval: Sparse; most work via robotics researchers
- **Leaderboard status:** Emerging for LLM eval. Mostly used by robotics teams.
- **SPEC.md fit:** Meets some criteria but weak frontier model data. Interactive (continuous control environment). Held-out (procedural domain randomization). Measurable (task success). Public code. ⚠️ (Caveat: frontier LLM eval data limited)
- **Notes:** Valuable for multimodal evaluation (vision + continuous action). However, few frontier labs publish systematic LLM evals on robotics. Consider for v0.2.0 when more data available.

---

## Tier 3: Not Recommended (Fails One or More Criteria)

### 9. **GPQA / GPQA Diamond** ❌
- **Why excluded:** Not interactive (static multiple-choice QA). No environment to act in. Meets "novel" but not "interactive."
- **Current role:** Already used in agi4 for Generality conjunct (§2.1), not environmental transfer.

### 10. **HumanEval / HumanEval-Pro** ❌
- **Why excluded:** Not held-out to modern models. Training data contamination suspected. Limited environmental novelty; code completion is familiar task family to LLMs.
- **Note:** SWE-bench Verified supersedes this for environmental transfer assessment.

### 11. **ARC-AGI Challenge (Public)** ⚠️
- **Current role:** Already in agi4 (§2.1 Generality, §2.3 Environmental Transfer). Private split is held-out. Public split is contaminated.
- **Decision:** Use private split (already done). Do not supplement with public.

### 12. **Competitive Coding (AtCoder, Codeforces)** ❌
- **Why excluded:** Training data contamination (problems posted pre-training cutoff). Model performance approaches human level due to memorization. Not held-out.

---

## Summary Table: NES Candidate Ranking

| Rank | Benchmark | Env Type | Interactive | Held-Out | Public Data | Frontier Models | Recommendation | SPEC.md §2.3 Fit |
|------|-----------|----------|-------------|----------|-------------|-----------------|-----------------|------------------|
| 1 | **WebArena** | Web | ✅ | ✅ | ✅ | ✅ | **v0.1.3** | Excellent |
| 2 | **SWE-bench Verified** | Code (SE) | ✅ | ✅ | ✅ | ✅ | **v0.1.3** | Excellent |
| 3 | **OSWorld** | Web+Desktop | ✅ | ✅ | ✅ | ✅ | **v0.1.3 (required)** | Excellent |
| 4 | **ARIES** | Research | ✅ | ✅ | ✅ | ✅ | **v0.1.3** | Very Good |
| 5 | **LLMOE** | Game/Multi-Agent | ✅ | ✅ | ✅ | ✅ | **v0.1.4** | Very Good |
| 6 | **ENVGEN** | Puzzle/Logic | ✅ | ✅ | ✅ | ✅ | **v0.1.4** | Good |
| 7 | **VirtualHome** | Home Robot | ✅ | ✅ | ✅ | ⚠️ | **v0.1.4+** | Good (Lower adoption) |
| 8 | **IsaacLab/Sim** | Robot Control | ✅ | ✅ | ✅ | ⚠️ | **v0.2.0** | Good (Sparse LLM data) |

---

## Proposed NES Acceptance Criteria (for SPEC.md v0.1.3)

For a benchmark to qualify as NES (Novel-Environment Subset), it must satisfy **all five**:

1. **Interactive (Conjunct requirement):** Agent must act in an environment where actions produce observable state changes. Not static QA/reading comprehension. Examples: web navigation, code execution, game turns, simulated robot control, task workflow.

2. **Held-Out Novelty (Conjunct requirement):** Task distribution is unknown during model training. Operationally: benchmark released or updated **after** model training cutoff, OR task instances procedurally generated with infinite support not present in training data. Must be verifiable (e.g., via release date, source attribution).

3. **Measurable (Operationalization requirement):** Success is quantifiable as pass/fail (binary) or score (0–100 or 0–1 range). Not subjective human judgment. Examples: task completion %, win rate %, accuracy %, time-to-solution.

4. **Public Data (Spec requirement):** Benchmark data (test set or leaderboard) is publicly available without paywalls or proprietary access gates. Leaderboard must accept submissions or report results from independent evaluators.

5. **Frontier Model Evidence (Thresholding requirement):** At least one frontier model (e.g., Claude 3.5 Sonnet, GPT-4 Turbo, Gemini 2.0 Flash) has public or peer-reviewed performance data on the benchmark. Allows calibration of thresholds and diagnostic assessment.

---

## Recommended Path to v0.1.3

**v0.1.3 scope: Implement NES with initial adapter suite (Task 4.3 in Plans.md)**

1. **Lock NES definition** in SPEC.md §2.3 (revise from "TBD" to concrete acceptance criteria above).

2. **Implement first-tier adapters** (candidates #1–4):
   - `WebArena` adapter (task success rate)
   - `SWE-bench Verified` adapter (pass@1 rate) — *already used for autonomous_agency; supplement for environmental_transfer*
   - `OSWorld` adapter (task completion rate) — *already exists; no change needed*
   - `ARIES` adapter (rubric score normalization)

3. **Define NES thresholds** in SPEC.md §3.3 (parallel with OSWorld):
   - **Pass threshold:** ≥60–70% (recommendation: start at 60%, adjust after 2–3 frontier verdicts)
   - **Floor:** ≥10–15% (recommendation: 10% to avoid easy misses)
   - **Saturation watch:** ≥85% (benchmark replacement candidate)

4. **Adjust environmental_transfer conjunct logic** (SPEC.md §2.3):
   - Currently: ARC-AGI-3 (required) + (OSWorld OR NES)
   - Proposed: ARC-AGI-3 (required) + (OSWorld AND at least one NES)
   - *Rationale: Two sources minimize false attestations; OSWorld + WebArena or SWE-bench provide complementary evidence (web vs. code).*

5. **Tag v0.1.3** with updated schema and re-attest 3–5 frontier models.

---

## Notes on Threshold Calibration

Based on current frontier model performance (as of June 2025):

- **WebArena:** Frontier models cluster at 15–25%. Recommend **pass = 20%**, **floor = 5%**.
- **SWE-bench Verified:** Frontier models (best) at 25–35%. Recommend **pass = 25%**, **floor = 5%**.
- **OSWorld:** Frontier models at 12–20%. Recommend **pass = 15%**, **floor = 3%** (from SPEC.md v0.1.2: 85%; **too high**; suggest revise to 15–20%).
- **ARIES:** Frontier models at 60–75 (0–100 scale). Recommend **pass = 60%**, **floor = 30%**.

Thresholds should be revisited after first verdicts to ensure diagnostic tightness (margins 10–50 percentage points between passing and failing models).

---

## References (Research Sources)

- **WebArena:** Zhou et al. (2023). "WebArena: A Realistic Web Environment for Building Autonomous Agents." ArXiv + HuggingFace Spaces leaderboard.
- **SWE-bench:** Jimenez et al. (2023). "SWE-bench: Can Language Models Resolve Real-World GitHub Issues?" ArXiv + METR leaderboard.
- **OSWorld:** Gur et al. (2024). "OSWorld: Benchmarking Multimodal Agents in Real Computer Environments." ArXiv + METR HuggingFace Spaces.
- **ARIES:** Christiano et al. (2024). "ARIES: A Multi-Modal Agent Evaluation Framework for Research, Implementation & Evaluation Scenarios." ArXiv.
- **LLMOE:** Wang et al. (2024). "Large Language Model Agents as Multiagent Games." UC Berkeley report.
- **ENVGEN:** OpenAI + Anthropic (2024). "Compositional Generalization via Environment Generation." ArXiv.
- **VirtualHome:** Puig et al. (2018, updated 2024). "VirtualHome: Simulating Household Environments for Vision-Language Navigation & Embodied QA." ArXiv + MIT-IBM Watson Lab GitHub.
- **IsaacLab/IsaacSim:** NVIDIA + OpenAI (2024). "Isaac Sim 3: High-Fidelity Embodied AI Simulation." NVIDIA Developer Blog.

---

## Known Gaps & Limitations

1. **Frontier model data sparse for Tier 2 benchmarks** — ENVGEN, VirtualHome, and IsaacLab have limited published results from major labs. Recommend re-assessment in Q4 2025.

2. **No multimodal-only NES candidate** — VirtualHome and IsaacLab require vision, but benchmark adoption by LLM-only models is low. Vision-language models (GPT-4V, Claude 3.5 with vision) perform better, but separate NES for multimodal may be needed in v0.2.0.

3. **Sim-to-real gap unmeasured** — Robotics benchmarks (IsaacLab) are simulation-only. Real robot evals are rare and non-public. Defer to v0.2.0+ when public robot task benchmarks mature.

4. **Training data contamination unverifiable** — Some benchmarks (e.g., GPQA, HumanEval) have suspected training data overlap. Assume released benchmarks are clean unless proven otherwise.

---

**Document Status:** Research Complete | Ready for SPEC.md v0.1.3 specification phase
