# Every System Has a Pyramid

**Source:** https://www.getcairn.dev/blog/pyramid
**Captured:** 2026-04-28

## How the Pyramid of Causality reveals why your system works, or why it doesn't.

**Greg** | March 2026 | 12 min read

There is a 190-page monograph sitting in the course materials of the Naval Postgraduate School that most practicing engineers have never encountered. Written by Professor Robert Harney for his systems engineering curriculum, _Technology Evaluation for Strategic Planning and Innovation_ builds a rigorous theory of how technologies emerge, mature, and die, and how to evaluate where any given technology sits on that arc. It is dense, methodical, and not available in any bookstore. It also contains one of the most useful mental models I've found for thinking about engineered systems.

Harney's central argument is deceptively simple: "every technology rests on a pyramid of prerequisite technologies." If any layer of the pyramid is missing, the capstone cannot be built. No matter how brilliant the concept, no matter how much funding is available, no matter how urgently the market demands it. The pyramid is not a metaphor. It is a structural dependency that governs what is possible and what is premature.

You cannot build what you cannot stand on. The pyramid of causality doesn't care about your timeline, your budget, or your ambition. It only cares whether the prerequisites exist.

## The Framework

### The Pyramid of Causality

Harney identifies six layers that every novel technology requires. At the base sits _knowledge_, the scientific understanding that makes the technology conceivable. Above that, _instruments and tools_, the ability to observe, measure, and manipulate at the relevant scales. Then _parts and materials_, the physical building blocks with the right properties. Then _supporting technologies_, the adjacent capabilities the system depends on. Then _domain technologies_, the mature implementations in the specific field. And at the apex, the _capstone_, the novel technology itself.

```
The Pyramid of Causality

        The Novel Technology
           /              \
      Domain Tech A    Domain Tech B
           \              /
            Supporting Tech
            Supporting Tech
            Supporting Tech
              /    |    \
           Parts Materials Parts
          Materials    /    |    \
         Instruments Tools Instruments
               Tools Instruments
              /    |    \
        Knowledge Knowledge Knowledge
        Knowledge Knowledge Knowledge
```

If any layer is missing or immature, the capstone cannot be placed.

The power of this model is diagnostic. When a project is failing (behind schedule, over budget, delivering less than promised) the instinct is to blame execution. More engineers, more money, more urgency. Harney's framework asks a different question: "is a layer of the pyramid missing?" Because if the supporting technology doesn't exist at the required maturity, no amount of execution discipline at the capstone level will compensate. You are building on an incomplete foundation.

Consider autonomous vehicles. The capstone (a fully self-driving car) has been "five years away" for over a decade. The knowledge exists (machine learning, computer vision, sensor fusion). The instruments exist (LiDAR, radar, cameras). The parts exist (GPUs, sensors, actuators). But the _supporting technologies_ (real-time edge inference at sufficient reliability, regulatory frameworks, mapping infrastructure at global scale) are not mature enough to support the capstone. The pyramid is incomplete. No quantity of venture capital will complete it faster than the prerequisite layers allow.

## The Lifecycle

### Every Technology Follows the Same Curve

Harney's second major framework is the technology S-curve, the observation that every technology traverses the same lifecycle: from precedents through conception, gestation, birth, exponential growth, maturation, maturity, senescence, decay, and eventually death. This isn't a vague analogy. Harney catalogs specific metrics for each stage and demonstrates the pattern across domains from vacuum tubes to aircraft carriers.

```
The Technology S-Curve

Prece-    Concep-   Gesta-    Birth    Growth   Procre-  Matur-   Matu-   Sene-    Decay    Death
dents     tion      tion                         ation    ation    rity    scence
```

The critical insight is the _growth phase_. During growth, performance improves exponentially, until it doesn't. Every S-curve has a ceiling, and that ceiling is defined by the limits the technology encounters. Understanding whether a limit is fundamental (physics, cannot be overcome), technological (engineering, can be overcome by switching implementation), or societal (economics, regulation, culture, soft but real) determines whether the S-curve will resume climbing after a breakthrough or flatten permanently.

## The Limits

### Three Kinds of Ceilings

This is where Harney's framework becomes most practically useful. He argues that the single most important skill in technology evaluation is distinguishing between limits that are real and limits that are inherited, between constraints imposed by nature and constraints imposed by convention, economics, or institutional inertia. An evaluator who mistakes a technological limit for a fundamental one gives up too early. An evaluator who mistakes a fundamental limit for a technological one wastes resources pursuing the impossible.

| **Fundamental** | **Technological** | **Societal** |
|---|---|---|
| Laws of nature | Current implementation | Economics, culture, regulation |
| Speed of light. Thermodynamic efficiency. Shannon's channel capacity. These cannot be overcome by any amount of engineering, only accommodated. | Battery energy density. Semiconductor feature size. Sensor resolution. These can be overcome by switching to a different physical principle or architecture. | Cost curves. Adoption resistance. Certification frameworks. Workforce availability. Real constraints, but soft. They yield to economic pressure and generational change. |
| Cannot be broken | Breakable with innovation | Yields to pressure over time |

The misidentification of limits is responsible for more failed programs than any engineering error. Harney documents the pattern repeatedly: a technology stalls, leadership concludes it has hit a fundamental wall, funding is redirected, and five years later, a competitor finds the technological workaround that was always available. Or conversely: a program pushes relentlessly against a limit that is genuinely fundamental, burning through budget and credibility until physics wins.

The evaluator who cannot distinguish a fundamental limit from a technological one will either give up on the possible or bankrupt themselves pursuing the impossible. Both failures look identical from the outside.

## The Application

### What Changes When You Can See the Pyramid

Most systems engineering tools model what a system _is_: its components, interfaces, requirements, behaviors. None of them model where the system _sits_ on the technology landscape. They can tell you the system has a battery subsystem. They cannot tell you that the battery chemistry is approaching a fundamental energy density limit, that the manufacturing process relies on a supporting technology with a 3-year maturation gap, or that the regulatory framework in the target market classifies the chemistry as hazardous (a societal limit that adds 18 months to certification).

#### 01 Visible Pyramid

Every node in your system model sits at the apex of its own prerequisite pyramid.

A "Battery Management System" node doesn't just have child nodes for monitoring, balancing, and thermal protection. It depends on cell chemistry (materials layer), battery management ICs (parts layer), thermal simulation tools (instruments layer), and electrochemistry research (knowledge layer). Making these dependencies visible (not just the engineering tree, but the technology pyramid beneath it) transforms decomposition into technology evaluation.

#### 02 Maturity Mapping

Color-code your system by technology readiness, not just completion status.

Imagine every node in your architecture carrying a maturity indicator derived from Harney's lifecycle stages. Green for mature technologies. Amber for growth phase. Red for gestation or earlier. Suddenly you can see the risk topology of your system at a glance, not where the engineering work remains, but where the technology uncertainty lives. A system that's 90% green and 10% red has a qualitatively different risk profile than one that's 60/40, even if both have the same completion percentage.

#### 03 Limits Analysis

When the AI flags a fundamental limit, you stop. When it flags a societal one, you plan around it.

An AI that understands Harney's taxonomy of limits could flag risk during decomposition: "This subsystem requires a battery energy density of 500 Wh/kg. Current technology is at 300 Wh/kg. This is a technological limit, potentially addressable through solid-state chemistry (TRL 4) on a 3 to 5 year horizon." That's not a chatbot suggesting you check your requirements. That's a technology evaluation integrated into the modeling process.

Harney wrote his framework for defense acquisition officers evaluating whether emerging technologies were ready for procurement. But the underlying insight is universal: "you cannot design well what you do not understand structurally." The pyramid of causality is a structural understanding of technological possibility. The S-curve is a temporal understanding of technological readiness. The limits taxonomy is a diagnostic for why progress stalls.

Together, they give engineers a vocabulary for the questions that decomposition trees and requirements databases never ask: not just "what is this system made of," but "what does this system stand on, how mature are its foundations, and what will prevent it from reaching its potential?"

Those are the questions we're building tools to answer.

_See how dendritic decomposition works in practice:_ [Watch the Animated Walkthrough](/concepts/dendritic-decomposition)

---

### Cairn is the AI engineering workbench for systems that matter.

Sign up and start building for free.

[Try Cairn free](/app)
