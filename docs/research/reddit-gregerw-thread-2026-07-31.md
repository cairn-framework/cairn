# Raw capture: Reddit thread with user gregerw (rounds one and two)

Captured 2026-07-31 from the maintainer's paste in session. Promotional ad
content and vote/share chrome elided; emojis elided. This is evidence for
`src.reddit-gregerw-first-user-test`; analysis lives in the source record
and downstream research.

## gregerw, 3 days before capture (pre-test, on the landing page)

Hi George, I find your project very interesting and would like to try it
out. Kudos for making it easy to try and then backtrack if it doesn't work!
However, I find it a bit difficult to understand what the workflow is. On
the landing page, I get the feeling that both the "outside" and internal
"inside" is covered and I struggle understanding "what do I need to know to
use this thing?"

To give you a bit more context: I typically organize also my personal
projects in a way that can scale across a team of people. Architectural
Decision Record is something that is known and very useful to drive
alignment across a team. A default choice is to keep ADRs in a directory in
the repo (that breaks down if you don't have a mono-repo). However, a lot
of design choices are emerging in code and coding agents infer a lot from
reading the code and also introduce new patterns (sometimes without the
developer knowing).

If I understand correctly, your tool addresses the two main difficulties:
how to turn code into an ADR/pattern and how to make an agent adhere to
these. The patterns need to be easily accessible to humans and reviewable
as adoption of a pattern/design choice has far wider impact than just a PR
in a corner of the code base.

The second difficulty is two-fold: how to make agents adhere to existing
patterns and how to make humans aware of these and help steer the agents
(as there will be interactions and choices on the way).

So two suggestions to the landing page: document the workflows and make it
clear what are concepts that you need to understand as a user vs concepts
that are internal (cairn architecture).

I will test it when I have the time.

## Maintainer reply

Hey Great feedback, I really appreciate that you cut through to a degree
and seem to get it.

These bigger communities I tried posting to seem to all downvote it.

I've actually started on a bigger change, with a switch to utilising cairn
both as the thing within a harness that helps keep the agents in line, but
also outside the harness, to use it as an orchestrator. And in this outside
perspective would be UX to better visualise as a human and be guided
towards helping solve load bearing problems.

I kept letting AI sessions talk me out of this for some time, but i
realised i wont be able to get a proper declarative workflow from within
the harnesses.

The other thing i need to do is some small coding project evals to do some
direct benchmarking and comparison of achieving project goal with and
without cairn, as well as analysing token usage etc.

Some of the results of my coding in general, are from also using the
oh-my-pi harness, with its advisor capability. Counter intuitive, but you
end up with less token usage with that, i recommend you check it out.

Anyways, so basically I'm trying to make this be the architecture that
allows software factories to actually somewhat work.

If i get many more people with comments like yours, id love to create a
discord to discuss some of this stuff. Actually ill just do one anyway and
add links, as im about to go to sleep.

Thank you gregerw!!

## gregerw follow-up

Great to hear that my comments were useful. Have you looked at what
Humanlayer is doing? The problem they are solving is tangent to what you
are doing (and maybe somewhat overlapping), but it may be an good
comparison. I don't use pi & friends myself, it will be interesting to see
if my setup is compatible.

## gregerw, 3 hours before capture (after installing on a real codebase)

Hi again, I have tried Cairn on one of my code bases, so a bit of feedback:

I wasn't quite sure (as mentioned in my last comment) what I would get
after install (i.e. what would be different and whether I could continue
working as before), but since you had explained so clearly how to clean up,
I did it in good faith on a separate branch. This may feel different for
others.

After having installed, I was a bit surprised with how many new files were
created and looking at them, it was not immediately clear to me why they
were there. It seemed like a lot of machinery.

From a value point of view, I was a bit at loss after installing. What now?
Then I remembered the UI, so was able to find the UI on localhost:3000
command.

I got a nice overview of modules and dependencies, but relative to the size
of the install, it felt a but underwhelming. I couldn't quite figure out
how this was connected to ADRs or higher-level designs/patterns. Also, I
couldn't figure out how to extract from the code base the decisions already
there and that should go into the graph. Maybe I missed it in the docs, but
as I used the prompt to install, I had expected some pointers to what is
next, how to extract the important designs from the code base as invariants
that would be gated later.

In the end, I uninstalled as the immediate value didn't feel high enough to
justify all the repo files and gating.

Just a note at the end: I may not have done Cairn justice, but I think my
test is somewhat akin to what most would be willing to spend to get a feel
for what this is.

## Maintainer reply

Thank you so much!

I think there is a lot that is ambiguous. So with cairn building WITH IT,
it connects to the ADRs on the graph, however the UI part of it, is a bit
broken, and not intuitive, and I would have fixed it... except i'm doing a
complete overhaul rather than just fixing it in place.

So when the AI is trying to create new items, if there is no connected
decision, it will have a warning when it lints with cairn (there is a tree
sitter for cairn). So then it forces the AI to write the decision behind
why it is working on something. And it forces the ai to create "ghost
nodes" of how the architecture should be, with that connected to the
ToDo/ADR/research (so research is also artifacts, so can go into the
provenance). One of the tensions this raises sometimes (which I guess is a
sign that it works!), is I would get annoyed when it would tell me, hey
George, we didn't do "x y" because we already made decision ADR#N. So it
was annoying but also showing me that by using cairn, its adding
guardrails, while still allowing it to create.

Currently, I've been using it in my terminal like: install cairn for
development and use it. And then I just tell the AI focusing on what I want
to build, and it uses cairn to structure it somewhat passively. But then
because the UI is not really used, and cairn is used at, IMO, the wrong
level, there is friction say between sessions, or growing towards creating
the bigger project vision.

TBH, I feel very lucky that you had a look at it, and shared your
perspective because I think most users would try it, see it not really add
any value, and not say anything.

If its ok, I'll hit you up again when I have it better organized and
messaging, and the product direction is more organized. (also with a demo
video).

I'm gonna pause posting about it for now, until I have the new UI done, as
tbh I've had some of the same problems.

You have gone above and beyond tbh, far beyond "most would be willing to
spend"....

this is my first real life detailed user feedback
