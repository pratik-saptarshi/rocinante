# Phase 12a — Confidence-based verification tier draft

| ID | Claim | Draft tier | Signal |
|---|---|---|---|
| UX-01 | Empty/missing imports and blank Apply use sample data | Standard | High reviewer confidence and cross-file input/data flow. |
| UX-02 | Fixed audit/security metrics lack section-level provenance | Light | High confidence; fixed literals and render path are directly checkable. |
| UX-03 | Audit/SEO/Field-Lab controls do not perform implied data operation | Light | All reviewers agree; simple source/render paths. |
| UX-04 | Schema-invalid object stage name can crash rendering | Standard | Single auditor discovery; requires tracing across parser, insight model, pulse, and React render. |
| UX-05 | Healthy input still produces red critical recommendations and static routes | Standard | Single auditor discovery; cross-file computed vs hardcoded path. |
| UX-06 | Stage urgency thresholds conflict | Standard | Single auditor discovery; cross-file config/classifier comparison. |
| UX-07 | Opportunity explanation contradicts score formula | Standard | Single auditor discovery; cross-file explanation/formula comparison. |
| UX-08 | Primary button contrast is 4.288:1 against white | Standard | External WCAG criterion plus color calculation and MUI palette behavior; criterion web-verified. |
| UX-09 | MUI Switch accessible name is attached to wrapper rather than input | Standard | Single reviewer discovery; depends on installed component slot forwarding. |
| UX-10 | Async statuses and payload success have no live-region announcements | Standard | High markup confidence; specific screen-reader behavior remains untested. |
| UX-11 | Heading hierarchy/main/tab-panel semantics are flat/missing | Light | Markup and installed MUI mapping are directly checkable; runtime announcement not tested. |
| UX-12 | Browser/Tauri recovery wording mismatches preview/native architecture | Standard | Severity disputed by reviewers; cross-check implementation and codemap. |
| UX-13 | Fixed narrow panel/order hurts responsive/task usability | Deep | Impact requires rendered viewport/task validation; retain only as conditional design question. |

Tier definitions follow Overseer: Light = local fact/read; Standard = multiple files/import flow; Deep = runtime/external or unresolved intent. The reviewer team did not execute a browser, screen reader, or task test.
