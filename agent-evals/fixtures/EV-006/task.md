Author `protocol.fsl` for a complete request/reply exchange using the supplied
read-only `eval/handshake` domain package and initial state. The domain is
problem data; you must write all protocol behavior yourself.

Declare `Handshake` with one origin instance over the supplied Job. Capture
its ticket. Bind Caller to Client and Worker to Server. Declare two distinct
capacity-one, unordered channels with exactly-once-premise delivery and block
overflow: Requests from Caller to Worker, and Replies in the reverse direction.
Both carry Message records with the job's ticket.

The exchange sends a request, receives that request, sends its reply, and
receives the reply, in that order. Each receive must refer to its corresponding
send and preserve the ticket. Finish only with a Closure whose complete field
is true. Do not add terminal-any, omit any message, weaken the finish condition,
change the domain or initial state, or replace the exchange with an empty run.

Use the shipped public FSL CLI to compile the authored protocol and analyze
its deadlock freedom. Save actual JSON outputs in `compile.json` and
`analysis.json`. A safe result must be an exhaustive certified proof, with
nonzero transitions, for this protocol and initial subject. An unsupported,
inconclusive or incomplete result is not a proof.

Read only this workspace and the supplied shipped documentation/grammar.
Do not read evaluator code, reference answers or source checkouts. Once all
required work succeeds, write `FSL-EVAL-DONE` only at the end of your response.
