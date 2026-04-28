# Why AI Can't Do Your Engineering Math (Yet)

**Source:** https://www.getcairn.dev/blog/engineering-math
**Captured:** 2026-04-28

LLMs reverse-engineer solutions from pattern-matching. They drop units, hallucinate material properties, and present wrong answers with perfect confidence. Engineers are right to be skeptical, and that skepticism is the design constraint.

**Greg** | April 2026 | 10 min read

## The Problem with AI Engineering Calculations

Ask an LLM to calculate the pressure drop through a 2-inch schedule 40 steel pipe carrying water at 150F and 50 GPM over 200 feet. You'll get an answer. It will be formatted beautifully, with equations laid out step by step, intermediate values clearly labeled, and a final result presented with apparent precision. It will also, more often than not, contain at least one error that an experienced engineer would catch immediately, a unit conversion done backwards, a Reynolds number calculated with the wrong diameter, a friction factor pulled from the wrong regime, or a fluid property cited at the wrong temperature.

The engineer catches the error because the engineer knows what the answer should look like. A pressure drop of 0.3 psi over 200 feet of 2-inch pipe at 50 GPM feels right. A pressure drop of 30 psi does not. A pressure drop of 0.003 psi does not. The engineer has calibrated intuition, decades of experience that tells them when a number is in the right ballpark. The LLM has no such intuition. It has pattern-matching, and pattern-matching does not know the difference between plausible and correct.

"An LLM doesn't solve a physics problem. It generates text that looks like a solved physics problem. These are not the same thing, and the difference is measured in safety margins."

## The Failure Modes

### How LLMs Get Engineering Wrong

The failure modes are not random. They are systematic, predictable, and rooted in how language models actually work. An LLM trained on millions of solved physics problems learns the _shape_ of a solution (what equations typically appear, what variables are named, what sequence of steps is expected) without learning the physical reasoning that connects them. It is performing a sophisticated form of autocompletion, not physics.

A 2024 study from researchers investigating LLM physics reasoning found that GPT-4 would ignore physical context like tensorial order and dimensional constraints in favor of algebraic pattern-matching. When the researchers removed key premises from physics problems (premises that a physicist would consider essential) the LLM often produced the same answer anyway, because the answer was being generated from the pattern of the problem type, not from the physical reasoning chain.

### Five failure modes engineers encounter daily

#### 1. Silent Unit Conversion Errors

"Calculate heat transfer for water at 150F flowing at 2 m/s through a 50mm tube"

The LLM mixes Imperial temperature with SI velocity and diameter, then silently converts between systems at arbitrary points in the calculation, sometimes correctly, sometimes not. No dimensional check is performed. The final answer carries units but no verification that those units are internally consistent.

**Why it's dangerous**

The Mars Climate Orbiter was lost because of a unit conversion error between pound-seconds and newton-seconds. The LLM treats unit systems as cosmetic labels, not physical constraints.

#### 2. Hallucinated Material Properties

"What is the yield strength of 17-4 PH stainless steel at 600F?"

The LLM returns a specific number, say, 145 ksi, with apparent confidence. It may even cite MMPDS or AMS specifications. But the value is interpolated from training data, not looked up from a source. At elevated temperatures, the properties of precipitation-hardened steels depend on condition (H900, H1025, H1150), exposure time, and aging history. The LLM collapses all of this context into a single number.

**Why it's dangerous**

A stress analyst using an LLM-provided material property that's off by 15% will calculate a margin of safety that doesn't exist. The part passes the analysis but fails in service.

#### 3. Wrong Regime, Right Equation

"Calculate the Nusselt number for this flow condition"

The LLM applies Dittus-Boelert (which is the most common correlation in its training data) without checking whether the flow is turbulent (Re > 10,000), whether the length-to-diameter ratio satisfies the entrance length requirement, or whether the Prandtl number is in the valid range. It selects the equation by frequency of appearance, not by applicability.

**Why it's dangerous**

"The Dittus-Boelter correlation applied to laminar flow can overpredict heat transfer by an order of magnitude."

#### 4. Fabricated References

"Size this pressure vessel per ASME BPVC Section VIII Division 1"

The LLM generates a calculation that references specific paragraph numbers and equations from the ASME code. Some references are real. Some are fabricated. The engineer must check every citation against the actual standard, a process that takes longer than doing the calculation from scratch.

**Why it's dangerous**

An incorrect code reference in a pressure vessel calculation isn't an academic error. It's a liability issue that survives in the design documentation and can surface during an incident investigation years later.

#### 5. Confident Nonsense at the Boundaries

"What is the fatigue endurance limit for 7075-T6 aluminum?"

The LLM returns a specific endurance limit, say, 23 ksi at 10^7 cycles. But aluminum alloys do not exhibit a true endurance limit the way steels do. Their S-N curves continue to decline indefinitely. The concept of a "fatigue limit" for aluminum is a simplification that appears frequently in introductory textbooks but is incorrect for design purposes. The LLM learned the simplified version because it appears more often in text.

**Why it's dangerous**

Designing an aluminum structure to an endurance limit that doesn't exist leads to fatigue failures that the analysis said couldn't happen.

None of these failures are bugs that will be fixed in the next model release. They are structural consequences of how language models work. An LLM generates the most probable next token given the context. When the most probable next token happens to be the physically correct one (which it often is for well-trodden problems) the output is correct. When the most probable next token diverges from physical reality (at system boundaries, regime transitions, edge cases, elevated conditions) the output is wrong with no change in confidence.

## The Paradox

### Why Engineers Use It Anyway

Despite everything above, engineers use LLMs daily. Not because they trust the outputs (most engineers are deeply skeptical) but because the alternative is worse. Before ChatGPT, an engineer who needed to recall the Dittus-Boelter correlation would open Incropera's textbook, find the right chapter, locate the equation, note the applicability constraints, and transcribe it into their calculation. That took fifteen minutes. Now it takes fifteen seconds, even after accounting for the verification step.

The pattern is consistent across the profession: LLMs as _recall accelerators_, not reasoning engines. Engineers use them to remember equations they've used before, to draft report sections they'll rewrite, to generate boilerplate code they'll debug, and to brainstorm approaches they'll evaluate. The LLM saves time on the parts of engineering that are memory-intensive but not judgment-intensive. The moment judgment is required (selecting the right correlation, interpreting the result, comparing against an allowable) the engineer takes over.

This is a rational workflow, but it's also an unstable one. It depends entirely on the engineer knowing enough to catch the LLM's errors. A senior engineer with twenty years of experience has the calibrated intuition to spot a pressure drop that's off by an order of magnitude. A junior engineer with two years of experience might not. And as organizations hire faster, as experienced engineers retire, and as project timelines compress, the gap between what the LLM produces and what the engineer can verify is widening.

"The engineer who trusts AI output without verification will eventually ship an error."

The engineer who verifies every AI output may wonder why they used AI at all. The answer has to be architectural, not behavioral.

## The Architecture

### What "AI Proposes, Physics Validates, Human Decides" Looks Like

The solution is not better LLMs. It is better architecture around LLMs. The fundamental insight is simple: _do not let the LLM compute_. Let the LLM understand the problem, select the approach, and orchestrate the workflow. Let deterministic engines (symbolic math solvers, unit-tracking libraries, material property databases, engineering standard lookups) perform the actual computation. And let the human review the result before it enters the engineering record.

This is not a theoretical architecture. The components exist today. SymPy performs symbolic mathematics with exact arithmetic, it doesn't hallucinate algebra. Pint and similar dimensional analysis libraries track units through every operation and raise errors on dimensional inconsistency, they don't silently drop conversions. CoolProp provides validated thermophysical properties for over 100 fluids, it doesn't interpolate from training data. The missing piece is not the computation engines. It's the orchestration layer that connects natural language input to deterministic computation to human review.

### Anatomy of a guardrailed engineering calculation

| | |
|---|---|
| **LLM** | Parse and Plan. Understand the problem. Select correlations. Identify required properties. Choose the computation sequence. |
| **Engine** | Compute and Track. SymPy solves the equations. Pint enforces dimensional consistency. CoolProp provides validated fluid properties. |
| **Engine** | Validate and Check. Units balance at every step. Regime assumptions verified. Results compared against physical bounds. |
| **Human** | Review and Accept. Every step visible. Every assumption explicit. Every source traceable. Accept, reject, or modify. |

"The LLM never touches the arithmetic. Deterministic engines don't hallucinate."

In this architecture, the LLM's strengths are leveraged (natural language understanding, broad engineering knowledge, the ability to select appropriate methods) while its weaknesses are contained. The LLM decides that Dittus-Boelter is the right correlation. The symbolic engine evaluates it with exact arithmetic and tracked units. The validation layer checks that the Reynolds number actually exceeds 10,000 and that the Prandtl number is within the valid range. The human sees every step, every assumption, every intermediate value with units attached, and decides whether to accept the result.

The critical design principle is _transparency at every stage_. The engineer should never see a final answer without the path that produced it. Not because the engineer wants to re-derive the solution (that defeats the purpose) but because the engineer needs to verify the _approach_, not the arithmetic. "Did the tool use the right correlation? Did it pull properties at the right temperature? Did it apply the right safety factor?" These are engineering judgment calls. The tool should surface them clearly, not bury them in a chat transcript.

### 01 Separation of Concerns

The LLM is the conductor, not the orchestra.

Every component should do what it does best. LLMs understand intent and select methods. Symbolic engines compute without error. Unit libraries enforce dimensional consistency. Property databases return validated data with citations. The orchestration layer connects them. No single component is trusted to do everything, because no single component can.

### 02 Fail Loud, Not Silent

A calculation that refuses to run is safer than one that runs wrong.

When units don't balance, the tool should stop and say why, not silently insert a conversion factor. When a correlation is applied outside its valid regime, the tool should flag it, not produce an answer with a footnote the engineer won't read. Engineering tools must be designed to fail loudly, because silent failures in engineering have consequences measured in steel, safety margins, and sometimes lives.

### 03 The Audit Trail

Every AI-assisted calculation is a document, not a conversation.

A chat transcript disappears when the session ends. An engineering calculation is a permanent record. It goes into a design review package, it supports a certification submission, it's referenced during failure investigations years later. AI-generated calculations need the same governance as human-generated ones: who produced it, what method was used, what assumptions were made, what version of the tool was running, and who accepted the result.

## The Path Forward

### Skepticism Is the Design Constraint

Engineers are right to be skeptical of AI for engineering calculations. That skepticism should not be treated as a marketing problem to be overcome with better demos. It should be treated as a _design constraint_ to be satisfied with better architecture.

### The naive approach: "Trust the AI"

- LLM performs the calculation end-to-end
- Units mentioned but not tracked or verified
- Material properties from training data, uncited
- Regime applicability assumed, not checked
- Result presented as final, take it or leave it
- Chat transcript lost after session ends

### The guardrailed approach: "AI proposes, physics validates, human decides"

- LLM selects the method, deterministic engines compute
- Units tracked through every operation, dimensional error halts execution
- Properties from validated databases with traceable citations
- Regime boundaries checked before correlation is applied
- Every step visible, engineer reviews approach, not arithmetic
- Calculation saved as governed, versioned engineering record

The "(Yet)" in the title of this piece is not about waiting for a smarter model. No amount of parameter scaling will give an LLM the ability to perform reliable dimensional analysis, because dimensional analysis is not a language task, it's a mathematical constraint that must be enforced, not predicted. The "(Yet)" is about architecture. When engineering AI tools are built with the right separation of concerns (LLM for understanding, engines for computation, governance for trust) then AI will be able to do your engineering math. Not because the AI got smarter, but because the system around it got honest about what each component can and cannot do.

Until then, the senior engineer's calibrated intuition remains the last line of defense between a confidently wrong AI output and a shipping product. That's not a workflow. That's a liability. And building the architecture that replaces intuition-as-guardrail with structure-as-guardrail is the most important unsolved problem in engineering AI.

---

### Cairn is the AI engineering workbench for systems that matter.

Sign up and start building for free.

[Try Cairn free](/app)
