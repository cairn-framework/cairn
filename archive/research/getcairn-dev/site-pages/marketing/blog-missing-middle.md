# The Missing Middle of Engineering AI

**Source:** https://www.getcairn.dev/blog/missing-middle
**Captured:** 2026-04-28

## Landscape

### The Missing Middle of Engineering AI

Enterprise platforms bolt chat windows onto legacy software. Student tools solve homework. In between, where practicing engineers actually work, there is almost nothing.

Every major engineering software vendor shipped an AI feature in 2025. Ansys added a copilot. MathWorks launched MATLAB Copilot. Siemens put an assistant inside NX. SOLIDWORKS introduced three AI companions with names (AURA, LEO, MARIE) as if naming them would make them more capable. What you won't find is a tool that actually _reasons about engineering_.

Meanwhile, the other end of the spectrum has exploded. Symbolab has 300 million users. Photomath was acquired by Google. If you're a sophomore studying for a dynamics exam, AI has you covered. If you're a practicing engineer sizing a heat exchanger to meet ASME thermal design standards, you're on your own.

The AI engineering tools market has a barbell problem: enormous weight at both ends, and a thin bar of nothing in the middle, right where practicing engineers actually work.

## The Barbell

### Two Extremes, No Center

The top end of the market is enterprise incumbents who have bolted LLM copilots onto software designed in the 1990s and 2000s. These tools require existing licenses that cost $1,000 to $3,500 per seat per year. They primarily answer documentation questions and generate boilerplate code. They do not perform engineering calculations, run trade studies, verify unit consistency, or propose design changes.

The bottom end is consumer AI math tools targeting students. They excel at step-by-step problem solving through calculus, but they have no concept of units, material properties, engineering standards, or the iterative multi-variable reasoning that characterizes real engineering work.

| Tier | Examples | Cost |
|------|----------|------|
| Enterprise Copilots | Ansys Engineering Copilot, MATLAB Copilot, Siemens NX Copilot, SOLIDWORKS AURA/LEO/MARIE, Jama Connect Advisor | $1,000 to $3,500/seat/year |
| The missing middle | | $30 to $80/month |
| Student/Consumer | Symbolab, Mathway, Photomath (Google), MathGPT, Wolfram Alpha, ChatGPT plus calculator | Free to $25/month |

No purpose-built AI tool serves the individual practicing engineer.

The gap isn't accidental. Enterprise vendors have no incentive to build downmarket. Their business model is high-touch sales to procurement departments, not self-serve signups from individual engineers. Student tools have no incentive to build upmarket. Engineering domain knowledge is hard to encode and the audience is smaller.

## The Daily Reality

### What Engineers Actually Do (and What AI Can't Help With)

Ask a mechanical engineer what they did last Tuesday and you'll hear something like: ran a trade study comparing three bearing types for a high-speed application, sized a cooling channel using Dittus-Boelert, checked a bolted joint against VDI 2230, pulled material properties for 17-4 PH at elevated temperature, wrote up the analysis in a report for the design review on Thursday. Five tasks. Five different tools. Five different contexts that have to be manually stitched together.

#### A real engineer's AI workflow today

| Tool | Function | Cost |
|------|----------|------|
| ChatGPT | "What's the Dittus-Boelert correlation for turbulent flow?" | $20/mo |
| Wolfram | Verify the correlation, get fluid properties at 150F | $7.25/mo |
| Python | Write a script with Pint for unit tracking, run the calc | Free (plus 30 min) |
| Excel | Organize results, compare against allowables, format for review | $12.50/mo |

4 tools, 3 context switches, **no unit validation between steps**

The engineer used AI exactly once, at the beginning, to recall a formula. The actual engineering work (applying that formula with correct units, checking the Reynolds number to verify the turbulent flow assumption, pulling temperature-dependent fluid properties, comparing the result against a design allowable with an appropriate safety factor) happened entirely outside the AI. The AI was a search engine with better prose.

General-purpose LLMs have specific failure modes that make them dangerous for engineering without guardrails: they reverse-engineer solutions from pattern-matching rather than performing physics-informed reasoning, they silently drop or convert units, they hallucinate material properties, and they present incorrect answers with the same confidence as correct ones. A 2024 study showed GPT-4 ignoring physical context like tensorial order and dimensional constraints. Another study found that LLMs would confidently cite nonexistent functions and fabricated references when generating MATLAB code.

The engineering calculation that fails silently is more dangerous than the one that fails loudly.

## The Frustration

### What Engineers Want vs. What They Get

Industry surveys consistently show the same pattern: data processing, financial calculations, and scientific data analysis receive the lowest AI satisfaction scores among technical professionals.

#### What engineers ask for
- Unit-aware calculations that catch dimensional errors before they propagate
- Trade study tools that suggest evaluation criteria based on the application domain
- Institutional memory: calculations from past projects that can be found and reused
- Auditable AI output that shows its work against engineering standards
- Context that persists across a design session without re-explaining constraints

#### What the market provides
- Chat windows that answer documentation questions
- Requirements quality checkers (passive voice detection)
- Code generation that calls nonexistent functions
- Boilerplate test case generators
- Stateless conversations that forget your system after each session

The frustration isn't that AI doesn't work. It's that AI works well enough to be tantalizing (well enough that engineers use it daily for brainstorming and drafting) but fails precisely at the moment the engineering judgment matters.

## The Architecture Problem

### Why Bolting AI Onto Legacy Software Doesn't Work

There's a simple test for whether a product is AI-native or AI-bolted-on: remove the AI features and ask whether the product works the same way. If the answer is yes, then the AI is a feature, not an architecture. This describes virtually every enterprise engineering tool that shipped an AI copilot in 2025.

#### 01 The Removal Test

If you can remove the AI and the product is unchanged, the AI was never structural. Ansys without its copilot is still Ansys. SOLIDWORKS without AURA is still SOLIDWORKS. The AI features in these products are valuable conveniences, but they don't change what the tool fundamentally does.

An AI-native tool is different: remove the AI and the workflow collapses, because the AI is the workflow.

#### 02 Tool-Augmented Reasoning

The LLM should orchestrate, not compute. The emerging pattern that works: LLMs handle natural language understanding and plan generation while specialized engines handle exact computation. SymPy for symbolic math. Dimensional analysis libraries for unit tracking. Bayesian optimizers for experiment design. The LLM is the conductor, not the orchestra.

#### 03 Governance as Architecture

Every AI-generated result needs an audit trail. Engineering decisions have consequences measured in steel and safety margins, not engagement metrics. An AI tool that proposes a design change must track what was proposed, what was accepted, what was rejected, and why. This isn't a nice-to-have compliance feature, it's the core architectural requirement that separates an engineering tool from a chatbot wearing a hard hat.

The companies building from scratch have a structural advantage here. Zoo, the AI-native CAD startup, designed their modeling language (KCL) specifically so that LLMs could read and write it natively. ThunderGraph, an emerging MBSE startup, builds system models incrementally using an AI graph agent that constructs elements one at a time, then validates the graph through automated traversal.

This is where the opportunity lives. Not in competing with Ansys on simulation fidelity or with SOLIDWORKS on geometric modeling, those are decades-deep technical moats. The opportunity is in the workflows that sit between and around those tools: the calculations, the trade studies, the decision documentation, the design rationale that currently lives in spreadsheets and PowerPoint files and dies the moment someone leaves the team.

## What's Missing

### Three Workflows Nobody Has Built

If you audit the daily work of a mechanical, systems, or aerospace engineer, three workflows consume enormous time, produce enormous value, and have exactly zero purpose-built AI tooling.

**The engineering calculation.** Not a homework problem, a multi-step, multi-variable calculation governed by a standard (ASME, API, MIL-STD), using temperature-dependent material properties, requiring dimensional analysis at every step, and producing a documented result suitable for a design review. The closest existing tool is the ChatGPT plus Wolfram Alpha plugin, which requires manual orchestration between two separate subscriptions and has no concept of engineering standards or calculation documentation.

**The trade study.** Engineers compare design alternatives using structured methodologies (Pugh matrices, AHP, weighted scoring) but every tool for this is either a blank spreadsheet template or a whiteboard with sticky notes. No AI tool suggests evaluation criteria based on the application domain, helps populate performance ratings from specification data, runs sensitivity analysis on criterion weights, or generates the documented decision rationale that design reviews require. The Pugh matrix was published in 1981. Forty-five years later, the state of the art for running one is a spreadsheet.

**The decision record.** When an engineer chooses a bearing type, a cooling architecture, or a sensor technology, the reasoning behind that choice matters as much as the choice itself. In two years, someone will ask why this bearing was selected, or whether the runner-up option should be reconsidered given new requirements. The answer is almost always lost, buried in an email thread, an outdated slide deck, or the memory of an engineer who has since changed jobs. No tool captures design decisions as first-class, queryable, traceable objects connected to the system model they affect.

The engineering decision is the unit of value, not the model element, not the requirement, not the simulation result. Decisions connect all of these. And right now, decisions are the least-tooled artifact in the engineering workflow.

## The Opportunity

### Why the Middle Will Be Built From Scratch

The missing middle will not be filled by enterprise vendors adding features downmarket. Their architecture doesn't support it. Desktop-native, license-gated, file-based tools cannot become browser-native, self-serve, cloud-computed engineering reasoning platforms through incremental updates. It will not be filled by consumer AI tools adding engineering knowledge upmarket. Unit-aware, standards-referenced, governance-tracked computation requires a fundamentally different data model than homework step-solving.

The tools that fill this gap will be built from scratch, by people who understand both the engineering domain and the AI architecture, and who build the governance layer first rather than bolting it on later. They will be web-native because engineers work across devices and shouldn't need IT to install software. They will be affordable because the individual engineer and small team market only works at tens of dollars per month, not thousands per year. And they will treat AI output with the appropriate engineering skepticism, not as truth to be accepted, but as proposals to be reviewed.

The barbell is unstable. The middle will be built. The question is whether it will be built by the incumbents who created the gap, by general-purpose AI companies who don't understand the domain, or by engineers who know what's missing because they've lived without it.

---

### Cairn is the AI engineering workbench for systems that matter.

Sign up and start building for free.
