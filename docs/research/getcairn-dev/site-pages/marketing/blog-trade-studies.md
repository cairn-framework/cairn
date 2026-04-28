# Trade Studies Deserve Better Than a Spreadsheet

**Source:** https://www.getcairn.dev/blog/trade-studies
**Captured:** 2026-04-28

## Decisions

The Pugh matrix was published in 1981. Forty-five years later, the state of the art for running one is still a spreadsheet that dies the moment someone closes the file.

**Greg, April 2026, 9 min read**

Sometime in the next month, an engineer at an aerospace company will open a blank Excel spreadsheet, type "Criterion" in cell A1, and begin building a Pugh matrix from scratch. They will manually list evaluation criteria. They will manually assign weights based on a conversation they had in a conference room that nobody documented. They will manually score each design alternative using a scale they invented that morning. They will arrive at a winner, copy the table into PowerPoint, present it at a design review, and save the file to a SharePoint folder where it will never be opened again.

Eighteen months later, a new engineer will join the program and ask why the team chose concept B over concept A. Nobody will remember. The spreadsheet, if found, will contain numbers without context: weights without justification, scores without rationale, a conclusion without reasoning. "The trade study will have produced a decision but destroyed the knowledge that supported it."

This is not a failure of process. The engineers followed the methodology correctly. It is a failure of _tooling_, and it has been the same failure, unchanged, for forty-five years.

### Lifecycle of a trade study

- Created in Excel, Tuesday morning
- Copied to PowerPoint, Thursday review
- Emailed as PDF, to 14 people
- Filed in SharePoint, never opened again
- "Why did we pick B?", 18 months later
- Nobody knows, decision orphaned

## The Methodology

### Good Frameworks, Terrible Tools

The engineering profession has excellent decision-making frameworks. The Pugh matrix, published by Stuart Pugh in 1981, provides a structured method for comparing design alternatives against a reference concept using qualitative scoring. The Analytic Hierarchy Process (AHP), developed by Thomas Saaty in 1980, decomposes complex decisions into pairwise comparisons and derives mathematically consistent priority weights. Weighted scoring matrices, Kepner-Tregoe analysis, Quality Function Deployment. Each of these methods has been refined over decades and taught in every engineering program.

The tools for executing these methods have not been refined at all. They are spreadsheets. In 1981, when Pugh published his method, the state-of-the-art tool for running a Pugh matrix was a table drawn on paper. In 2026, the state-of-the-art tool for running a Pugh matrix is a table drawn in Excel. The methodology matured. The tooling froze.

### Pugh Matrix: Bearing Selection

High-speed spindle, 15,000 RPM, 500N radial load, clean room environment

| Criterion | Wt. | Angular Contact (Reference) | Ceramic Hybrid | Air Bearing |
|-----------|-----|-------|----------|------------|
| Speed Rating | 5 | 0 | +1 | +2 |
| Load Capacity | 4 | 0 | 0 | -2 |
| Contamination Resistance | 5 | 0 | +1 | +2 |
| Unit Cost | 3 | 0 | -1 | -2 |
| Lead Time | 2 | 0 | -1 | -2 |
| Maintenance Interval | 3 | 0 | +1 | +1 |
| Weighted Total | | 0 | +5 | +2 |

Winner: **Ceramic Hybrid**, but only if the weights hold.

Where did the weights come from? What happens if "Cost" matters more? Who decided these criteria?

The matrix above looks decisive. Ceramic hybrid wins. But "the decision is only as good as the inputs, and every input is vulnerable." Who decided the weights? A conversation in a meeting that wasn't recorded. Why isn't "vibration damping" a criterion? Nobody thought of it, or someone did and was overruled, and neither fact was documented. What happens if the cost weight increases from 3 to 5? Nobody ran the sensitivity analysis. What specification data supports the +1 contamination score for ceramic hybrid? It came from a datasheet someone read last week and paraphrased from memory.

Every one of these failure modes is a tooling problem, not a methodology problem. Pugh's method explicitly calls for systematic criteria identification, rigorous scoring against specification data, and iterative refinement. The spreadsheet doesn't enforce any of this. It accepts whatever you type and calls it a trade study.

"A spreadsheet doesn't know the difference between a rigorous trade study and a table full of guesses."

## The Gap

### Zero AI Trade Study Tools Exist

This is not an exaggeration. Across the entire landscape of AI-powered engineering tools (enterprise platforms, startup products, open-source projects, research prototypes) not a single purpose-built AI trade study tool exists. No product automates Pugh matrices. No product assists with AHP pairwise comparisons. No product runs sensitivity analysis on criterion weights. No product generates documented decision rationale from trade study results.

Caltech's AI-Assisted Model-Based Systems Engineering workshop teaches students to run AI-assisted trade studies, using ad-hoc ChatGPT workflows combined with Cameo, manually orchestrated by the engineer. The demand is clear enough that a premier engineering school built a course around it. But there is no product. The workflow is duct tape.

Consider what an AI-assisted trade study could actually do. An engineer describes the decision context: "I need to select a bearing type for a high-speed spindle application, 15,000 RPM, 500N radial load, clean room environment." An intelligent tool could suggest evaluation criteria the engineer might have missed based on the application domain: vibration damping, thermal expansion coefficient, outgassing rate for clean room compatibility. It could pull specification data from manufacturer datasheets to populate performance ratings with traceable sources rather than paraphrased memory. It could run Monte Carlo sensitivity analysis across the weights to show whether the decision is robust or whether a small change in priorities flips the winner. And it could generate a natural-language decision rationale suitable for a design review, not a table of numbers, but a documented argument.

None of this is technically difficult. Weighted scoring and AHP eigenvector calculations are undergraduate math. Monte Carlo sampling is a few lines of Python. The AI components (criteria suggestion, data extraction, rationale generation) are exactly what LLMs do well. The missing ingredient is not technology. It's someone building the product.

## The Deeper Problem

### Trade Studies Are Disconnected From the Model

Even if a better trade study tool existed, it would solve only half the problem if it operated in isolation. The real pathology is that trade studies exist outside the system model. An engineer selects a bearing type in a spreadsheet, then manually creates a node in the architecture model, then manually writes requirements that reflect the choice, then manually updates the interface definitions. The decision and the model are connected only through the engineer's memory and discipline.

When the trade study is disconnected from the model, three things break. First, _traceability dies_, there is no link between "we chose ceramic hybrid bearings" and the system model node that represents the bearing assembly. You cannot navigate from the model element to the decision that created it. Second, _revisitation is blind_, when requirements change eighteen months later and someone asks whether the bearing decision should be reconsidered, they have to find the original spreadsheet, understand the context, re-evaluate the criteria, and check whether the runner-up alternatives are still viable. If the trade study lived in the model, all of this would be immediate. Third, _the pruned alternatives vanish_, the concepts that were evaluated and rejected disappear entirely, along with the reasoning for rejection. Future engineers don't just lack the answer to "why did we pick B?", they lack the answer to "what else was considered?"

### 01 Model-Native Decisions

The trade study should live where its consequences live, inside the model.

When an engineer selects a concept, the decision itself (the alternatives considered, the criteria used, the weights assigned, the scores given, the rationale written) should attach to the model node it created. Navigate to "Bearing Assembly" and see not just what it is, but why it was chosen, what else was considered, and what would trigger reconsideration.

### 02 First-Class Alternatives

The dead paths are as important as the chosen path.

Pruned alternatives aren't clutter, they're institutional knowledge. "We considered air bearings but rejected them due to load capacity limitations at 500N radial" is engineering intelligence that prevents future teams from re-evaluating a dead end and informs future teams if the constraint changes. A system model that only shows what was selected is a model with amnesia.

### 03 Weight Sensitivity

If a 10% change in weights flips the winner, the decision isn't robust.

The most dangerous trade study is one that produces a clear winner that is actually fragile. Monte Carlo sensitivity analysis on criterion weights reveals whether the decision is robust across reasonable variations in priority. If ceramic hybrid wins by a large margin regardless of weight perturbation, the team should move fast. If it wins by a hair and flips to air bearing when cost weight increases by one point, the team needs a harder conversation.

## The Alternative

### What a Model-Native Trade Study Looks Like

Imagine the trade study not as a spreadsheet that exists alongside the system model, but as a structured operation within it. The engineer identifies a node ("Bearing Assembly") and says "run a trade study." The tool already knows the parent subsystem, the sibling components, the interface requirements, the performance parameters. It suggests evaluation criteria derived from the system context and the application domain. The engineer adjusts, adds, removes. The tool helps populate scores from specification data. The engineer validates, overrides where judgment matters. The weights are assigned through explicit pairwise comparison rather than arbitrary numbering, and the tool checks them for consistency.

The result is not a spreadsheet. It is a decision record attached to the model node, containing the full set of alternatives (including the rejected ones, preserved as pruned branches), the evaluation criteria with their justifications, the scoring with traceable sources, the sensitivity analysis showing decision robustness, and a generated rationale narrative suitable for design review documentation. When someone asks "why ceramic hybrid?" eighteen months from now, the answer is one click away, not buried in a SharePoint folder, not lost in an email thread, not dependent on the memory of an engineer who has moved on.

### The spreadsheet trade study

Static, disconnected, ephemeral

- Lives in Excel, disconnected from the system model
- Weights assigned without justification or consistency check
- Scores from memory, no traceable source data
- No sensitivity analysis on decision robustness
- Rejected alternatives disappear entirely
- Rationale dies with the file or the engineer

### The model-native trade study

Governed, traceable, alive

- Attached to the model node it created, one click from element to decision
- Pairwise comparison enforces weight consistency (AHP)
- Scores linked to specification data and parameter catalogs
- Monte Carlo sensitivity reveals if decision is fragile or robust
- Pruned alternatives preserved as first-class model elements
- Decision rationale generated, versioned, queryable forever

The technical requirements for building this are not exotic. The decision-analysis math is well-understood. The AI components (criteria suggestion, specification extraction, rationale generation) are standard LLM capabilities. The data model requirement is the interesting one: the trade study must be a governed operation that produces a reviewable result, not a freeform document that exists outside the engineering workflow. The decision must be proposed, reviewed, and accepted through the same governance process as any other model change.

Stuart Pugh gave engineers a rigorous methodology in 1981. Thomas Saaty gave engineers a mathematically consistent weighting method in 1980. What nobody has given engineers (in forty-five years) is a tool that implements these methods with the intelligence, traceability, and governance that the decisions they support actually deserve. The spreadsheet is not a tool for engineering decisions. It's the absence of one.

The engineering decision is the unit of value, not the model element, not the requirement, not the simulation result. Decisions connect all of these. And right now, decisions are the least-tooled artifact in the entire engineering workflow.

---

### Cairn is the AI engineering workbench for systems that matter.

Sign up and start building for free.

[Try Cairn free](/app)
