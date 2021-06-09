# Konnect Comment Parser

This is the source code for algorithms that I developed for sorting and processing
comments on the frontend part of [Konnect](https://alpha.konnect.dev) obtained from
its [API](https://api.konnect.dev).

Comments are linked to each other by means of their `reply_to` field. However, to
render them on the front in a reddit-like style, I had to:
- Sort them by date
- Find and move replies to a `replies` replies of their parent comment
- Flatten this new structure (some recursion here)
- Introduce a `depth` field for the frontend to 'understand' how to space the comments out
- Remove duplicates

[Konnect](https://konnect.dev) is currently a proprietary private project although
there is a probability of it becoming open-source in the future. To participate in
its development (whether direct code contributions or just by being a q/a tester),
reach out to me on [kishan@konnect.dev](mailto:kishan@konnect.dev).

PRs made and merged to this repo will be considered in the actual project.
