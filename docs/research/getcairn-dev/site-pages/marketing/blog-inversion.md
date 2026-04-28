# The Thirty-Year Inversion

**Source:** https://www.getcairn.dev/blog/inversion
**Captured:** 2026-04-28

## Thesis

For three decades, value moved from atoms to bits. That era is ending. What happens when the arrow reverses, and why the bottleneck nobody invested in is about to matter more than anything else.

**Greg, April 2026, 9 min read**

Software ate the world. Marc Andreessen said it in 2011, and he was right. For fifteen years, the highest-leverage thing a company could do was write code. Physical products became delivery mechanisms for software services. The car became an app platform. The thermostat became a subscription. The factory became a data pipeline that happened to produce objects. The talent followed: the best engineers went to software because that's where the problems were interesting, the iteration cycles were fast, and the returns were enormous.

That era is ending. Not because software stopped mattering (it matters more than ever) but because _AI is collapsing the cost of producing it_. When a competent developer with an LLM can produce in an afternoon what took a team a quarter, the scarcity shifts. Software is becoming abundant. The question is no longer "can we build the code?" The question is: _can we build the thing the code controls?_

For thirty years, the arrow pointed from atoms to bits, physical value captured as digital value. The arrow is reversing. The next thirty years will be defined by how well we turn bits back into atoms.

## The First Era

### How We Got Here: Atoms to Bits

The internet era followed a consistent pattern: take a physical process, digitize it, and capture the margin. Retail became e-commerce. Film became streaming. Banking became fintech. Publishing became social media. In each case, the physical infrastructure didn't disappear (warehouses, studios, vaults, printing presses all still exist) but the _value_ migrated to the digital layer that orchestrated them. Amazon is worth more than every physical retailer combined not because it has better warehouses, but because it has better software.

This created a thirty-year talent siphon. The highest-paying, highest-status engineering jobs moved to software. Mechanical engineers watched as CS graduates with two years of experience earned more than they would after a decade. Electrical engineers retrained as firmware developers, then as full-stack developers, then abandoned hardware entirely. Aerospace engineers went to SpaceX (one of the only physical-product companies that could compete with FAANG on compensation) or left the field.

The result is a discipline imbalance that few people talk about. The global software developer population is roughly 28 million and growing at 20%+ annually. The global systems engineering population (the people who design complex physical systems from satellites to medical devices to power grids) is perhaps 500,000, growing at single digits, with an aging workforce and a pipeline that can't backfill retirements. We've been systematically underinvesting in the discipline that designs the physical world for an entire generation.

## The Inversion

### Bits Back to Atoms

Three forces are converging to reverse the arrow.

_First, AI is commoditizing software creation._ GitHub Copilot, Claude, Cursor, Devin. The tools are different but the effect is the same: writing code is no longer the hard part. The hard part is knowing what to build, why to build it, and how it integrates with the physical world. A language model can generate a REST API in seconds. It cannot design a battery management system that won't catch fire, or a suspension geometry that handles uneven terrain, or an antenna pattern that maintains link margin in a rolling sea state.

_Second, the physical world is demanding more complexity._ Electric vehicles have 3 to 5 times the electronic systems content of combustion vehicles. Autonomous systems (drones, robots, self-driving vehicles) combine mechanical, electrical, thermal, and software engineering in ways that no single discipline can manage alone. Satellite constellations are scaling from dozens to tens of thousands. Fusion energy, advanced manufacturing, biotech hardware. Every frontier is physical, multidisciplinary, and systems-engineering-intensive.

_Third, the consequences of getting physical systems wrong are escalating._ A software bug gets a hotfix. A structural failure in a 3,000-meter autonomous undersea vehicle means you lose an $800K asset with no recovery. A battery thermal runaway in a consumer product means a recall, a lawsuit, and a brand crisis. As physical systems become more complex and more autonomous, the cost of design errors compounds, and the discipline responsible for preventing them has been starved of talent and tooling for thirty years.

### 1995 to 2025: Atoms to Bits

- Value migrates from physical to digital
- Software captures the margin on physical products
- Best engineers go to FAANG, fintech, SaaS
- Physical engineering underinvested for a generation
- Systems engineering tools frozen in the 2000s

### 2025 to 2055: Bits to Atoms

- AI commoditizes software creation
- Scarcity shifts to physical system design
- Autonomous systems demand multidisciplinary integration
- Systems engineering becomes the bottleneck discipline
- Tooling must catch up or break

## The Bottleneck

### The Discipline Nobody Invested In

Systems engineering is the discipline that integrates all the others. It doesn't design the battery or the motor or the software, it designs the _system_ that combines them into something that works. It decomposes requirements into subsystems, traces constraints through interfaces, verifies that the whole is consistent with the parts, and manages the complexity that emerges when dozens of engineering disciplines must coordinate on a single product.

The tooling for this discipline is, to put it plainly, twenty years behind the rest of engineering. The dominant MBSE platform charges $1,000 to $2,500 per user per year, has a user interface that practitioners describe as "gratuitously complex," and (as of early 2026) offers zero AI-assisted capabilities. No automated decomposition. No intelligent traceability. No architecture generation. The tool that is supposed to manage the most complex engineering artifacts in the world has not meaningfully evolved since the era of desktop Java applications.

Only 5% of organizations have fully transitioned to model-based systems engineering. Not because the methodology is wrong (decades of evidence in aerospace, defense, and automotive prove it works) but because the tools make it harder than it needs to be. The learning curve is steep, the cost is high, the collaboration is poor, and the return on investment is difficult to demonstrate when the tool itself is the primary barrier to adoption. This is a tooling failure, not a methodology failure.

### The Investment Imbalance

| Category | Status |
|----------|--------|
| Web frameworks | Saturated |
| DevOps / CI/CD | Saturated |
| Design tools | Mature |
| Code AI assistants | Growth |
| PLM / CAD | Modernizing |
| Hardware sim | Legacy |
| MBSE / SysEng | Starved |

## The Implication

### What Happens When the Bottleneck Breaks

The inversion creates an asymmetry that most of the technology industry hasn't noticed yet. AI is making software engineers 10 times more productive, that's the headline. The story underneath it: the newly productive software engineers are building autonomous drones, surgical robots, electric aircraft, and orbital infrastructure that all require _systems engineering_ to integrate. Software productivity is accelerating the demand for the discipline it spent thirty years cannibalizing.

This bottleneck will not be solved by hiring. The pipeline doesn't exist. You cannot manufacture experienced systems engineers on a two-year timeline the way you can retrain web developers. The knowledge is deep, the judgment is earned through decade-long apprenticeship, and the consequences of inexperience are measured in failed programs, not failed deployments.

The bottleneck will be solved by _tooling that amplifies the systems engineers who already exist_. Not replacing their judgment (the discipline is too consequential for that) but extending their reach. Helping them decompose systems faster, trace constraints more completely, identify gaps they'd otherwise miss, and communicate their reasoning to the growing teams of software engineers, mechanical engineers, and electrical engineers who need to understand the system-level picture.

## The Opportunity

### 01: AI doesn't replace the systems engineer. It gives them leverage.

A language model cannot design a safe system. But it can propose a decomposition for the engineer to review. It can trace requirements through an architecture and flag gaps. It can generate interface definitions from node descriptions. It can draft verification plans from requirements. Each of these tasks takes hours manually. Each can be proposed in seconds and reviewed in minutes, if the tooling treats AI as an engineering partner rather than a chatbot.

### 02: The Timing. SysML v2 creates a once-per-generation switching window.

The systems modeling language was completely rewritten in 2025. SysML v2 breaks backward compatibility with v1, forces every organization to re-evaluate their tooling, and (critically) introduces a textual notation that is far more amenable to LLM processing than the purely graphical notation it replaces. The standard is resetting at the exact moment AI capabilities are mature enough to exploit it. This alignment is not coincidental. It's the inflection point.

The thirty-year inversion is not a prediction. It is already happening. The companies building the physical future (autonomous vehicles, space infrastructure, clean energy systems, advanced manufacturing) are discovering that software is the easy part. The hard part is the system. And the systems engineering discipline they need has been waiting, underfunded and under-tooled, for the world to remember that atoms matter.

It's time to build the tools that match the moment.

## Cairn is the AI engineering workbench for systems that matter.

Sign up and start building for free.

[Try Cairn free](/app)
