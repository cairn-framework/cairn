/**
 * The three prompt contracts of the creation journey.
 *
 * Each is a closed contract: what the harness may write, what it must not
 * write, and the exact JSON it replies with. The register rules (plain
 * language, no jargon, no em-dashes, British spelling) are stated in the
 * prompt because the harness authors copy the person reads.
 */

const REGISTER = `REGISTER RULES, they apply to every word you write for the person to read:
- Plain language a non-technical reader follows. No jargon, no ids in prose,
  no framework vocabulary, no marketing voice.
- British spelling.
- Never use an em-dash. Use a full stop, a colon, a comma, or brackets.
- Short sentences. Say the thing, then stop.`;

/**
 * Step 1, describe: the person's words become the blueprint.
 *
 * Ghost paths are the point. The harness declares modules whose files do
 * not exist, which is what makes the map form as ghost structure.
 */
export function describePrompt(description) {
  return `You are the decode step of a guided console. A person described the
software they want, in their own words. Turn that description into a cairn
blueprint. You are working in a fresh project directory.

THEIR WORDS:
${description}

WHAT TO DO:
1. Write \`cairn.blueprint\` in the current directory, shaped exactly like this:

    System <Name> "<one plain sentence of purpose>" id "<slug>" {
        decisions "./meta/decisions"
        research "./meta/research"
        todos "./meta/todos"

        Container <LayerName> "<one plain sentence a non-technical reader understands>" id "<slug>.<layer>" {
            Module <PartName> "<one plain sentence>" id "<slug>.<layer>.<part>" {
                path "./src/<layer>/<part>.ts"
            }
        }
    }

    <slug>.<layer>.<part> -> <slug>.<other-layer>.<other-part> "<plain reason>"

   That grammar is complete. Do not go looking for more of it, and do not
   read cairn's own documentation or source.

2. Rules:
   - Three layers, ordered so that one rests on the next: what the person
     touches, the middle that turns their input into an answer, and the
     foundations both of those stand on.
   - Between 8 and 12 modules in total.
   - Every module \`path\` MUST point at a file that does not exist. Create no
     source files at all. Unbuilt parts are the whole point of this step.
   - Names and descriptions are plain language, in the register below.
   - Declare dependency edges after the closing brace so the layers really
     do rest on each other. Edges point from the part that needs something
     to the part it needs.

3. Run \`cairn scan\` once. If it reports an Error or a Warning that is not
   about a missing contract, fix the blueprint and scan again. Stop after
   three attempts and report what remains in your summary.

${REGISTER}

THEN REPLY WITH ONLY THIS JSON. No prose around it, no code fence:
{"summary": "<two or three short sentences, at most 45 words in total, telling the person what you mapped. Address them as you. Name the three layers in their own words. Say that nothing is built yet.>"}
`;
}

/**
 * Step 2, grill: doubts surface as plain questions with selectable answers.
 *
 * Read-only. The questions are about calls the description does not settle,
 * and each one names the part of the map it decides.
 */
export function grillPrompt(description, blueprint) {
  return `You are the grill step of a guided console. The person's description has
been mapped into a blueprint. Your job is to find the calls the description
does not settle, and put them to the person as plain questions they can
answer by picking an option.

THEIR ORIGINAL WORDS:
${description}

THE MAP AS IT STANDS:
${blueprint}

WHAT TO DO:
1. Find between two and four genuine open calls. A call is genuine when the
   description leaves it open, the answer changes how a named part behaves,
   and a reasonable person could pick either way. Do not ask about anything
   the description already settles, and do not ask the person to choose
   between technologies.
2. Exactly one of them is the call most worth arguing with: the one whose
   answer reaches deepest into the foundations. Mark that one.
3. Write nothing. Read the blueprint, do not change it, and create no files.

${REGISTER}

THEN REPLY WITH ONLY THIS JSON. No prose around it, no code fence:
{"questions": [
  {
    "id": "<short kebab-case slug>",
    "question": "<the question in plain words, one sentence, no jargon>",
    "options": [
      {"id": "<short kebab-case slug>", "label": "<the answer as a person would say it, under 8 words>"},
      {"id": "<short kebab-case slug>", "label": "<the other answer, under 8 words>"}
    ],
    "why": "<one or two short sentences saying which part of the map this decides, using the part's plain name, and what changes either way>",
    "node": "<the id of the node this decides>",
    "loadBearing": <true for the one call most worth arguing with, false otherwise>
  }
]}
`;
}

/**
 * Step 3, settle: the answers become recorded decisions in the target
 * project, and the map is amended where an answer changed it.
 *
 * This is the step that makes the working drawer's ruling count true.
 */
export function settlePrompt(description, blueprint, answered) {
  const chosen = answered.map((entry) => `- ${entry.question}\n  Their answer: ${entry.answer}\n  This decides: ${entry.node}`).join("\n");

  return `You are the settle step of a guided console. The person has answered every
open question. Write their answers into the project as recorded decisions,
and amend the map where an answer changed it.

THEIR ORIGINAL WORDS:
${description}

WHAT THEY ANSWERED:
${chosen}

THE MAP AS IT STANDS:
${blueprint}

WHAT TO DO:
1. Write exactly one decision per answer, and not one more. A decision records
   a call the person made. Recording anything they did not choose puts words in
   their mouth, in a file that then carries authority over the build, so do not
   do it however sensible the extra call looks.
2. For each answer, run \`cairn decision new <slug> --node <node-id>\` in the
   current directory, using a short kebab-case slug that names the call. Then
   open the file it scaffolded at \`meta/decisions/<slug>.md\` and write the
   body: what was asked, what the person chose, and what that binds. Keep the
   frontmatter the scaffold gave you, and set \`status: accepted\`.
3. Before you finish, list \`meta/decisions/\` and count. There must be exactly
   ${answered.length} ${answered.length === 1 ? "file" : "files"}. Delete any other decision you created.
4. If an answer changes the map, amend \`cairn.blueprint\`: add, remove, or
   re-point a module or an edge. Keep every module a path that does not
   exist. Create no source files. If an answer changes nothing structural,
   leave the blueprint alone and say so.
5. Run \`cairn scan\` once and fix any Error or any Warning that is not about
   a missing contract.

${REGISTER}

THEN REPLY WITH ONLY THIS JSON. No prose around it, no code fence:
{"note": "<one or two short sentences telling the person their answers are written in and what, if anything, moved on the map>"}
`;
}
