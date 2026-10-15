# SkillRoster v1.8.46 release receipt

CLI **1.8.46** bundles Bootstrap **1.8.50**. This record separates the merged
source, formal tag artifacts, anonymous public downloads, Cargo installation,
and Homebrew delivery.

## User-visible changes

- Empty and informational-only reports have a clear stopping point while
  preserving scan scope, unknown usage coverage, and full Finding access.
- Duplicate Plans explain the retained source, replacement paths, affected
  Agents, and reversible impact. Deduplication does not itself reduce the Roster.
- Home distinguishes CLI readiness from optional Bootstrap status;
  `setup --check` inspects targets without saving a Plan.
- Bootstrap keeps analysis, deduplication, and Core reduction separate. No
  positive usage evidence means asking about upcoming work, with keeping the
  current Roster as a valid outcome.
- Changes already on main since v1.8.45 now reach public installers: ordinary-task
  fallback, Claude native visibility, explicit blocked Finding decisions, and
  overlap-candidate truncation disclosure.

## Source and validation

[PR #384](https://github.com/tt-a1i/skillroster/pull/384) merged as
`e1a4d60fa9fc3cb81cdcbf715e5e374ee76c8bd0`. Its Git tree matches the accepted
PR head `1a36b02fbfebb7d69f8e4d7d0335d1ec56ec3bf8` exactly.
The [PR CI](https://github.com/tt-a1i/skillroster/actions/runs/38700400423),
[merged-main CI](https://github.com/tt-a1i/skillroster/actions/runs/38702488373),
and [formal tag workflow](https://github.com/tt-a1i/skillroster/actions/runs/38702600677)
all passed. The tag workflow validates the exact source with Rust 1.85 and
runs native tests, builds, governance smoke, and extracted-archive first-use
acceptance on Linux x86_64, Windows x86_64, and both macOS architectures. It
also verifies the Linux archive in checksum-pinned Ubuntu WSL2.

The source-equivalent local gate passed 492 Rust tests and 152 Node tests, with
one pre-existing Rust test ignored; formatting, strict Clippy, and documentation
guards passed. The Node harness passed on local Node 24.18 and 26.8.

## Published artifacts

Annotated tag `v1.8.46` has tag object
`11e9b5ee9d904abe1860c7f6ae9580d843430be4` and resolves to the merged source
above. The [GitHub Release](https://github.com/tt-a1i/skillroster/releases/tag/v1.8.46)
was published on 2026-10-15 at 08:16:19 UTC and marked Latest. It has exactly
four platform archives and four adjacent checksum files.

| Target | SHA-256 |
| --- | --- |
| `aarch64-apple-darwin` | `0e97493153bb4787f62c2b0af2b54ed94319b15e07cef4e10d821c348246070a` |
| `x86_64-apple-darwin` | `90c56cc07f57924831d33634142da0d70d157d704c9a7318c5e472f559d2e08f` |
| `x86_64-unknown-linux-gnu` | `b5d2620549c116a62f49cdd4130266fdb9f8c80acb0d99b6ea293ac7219ab2cb` |
| `x86_64-pc-windows-msvc` | `e6463936a09d1d3e856ca97a455f3ff021d6436749906e17e0607418254ae1b5` |

Every archive contains only its binary, README, and LICENSE under the expected
versioned prefix. README and LICENSE match the exact tag source byte-for-byte.
All eight GitHub asset digests and sizes matched the verified tag artifacts
before publication. All eight files were then downloaded without authentication
from the public release URLs and matched those artifacts byte-for-byte.

The downloaded macOS ARM archive passed the release governance smoke and all
four first-use scenarios, including exact Bootstrap installation and Undo.
An independent `cargo install --locked --git ... --tag v1.8.46` into a temporary
prefix also passed all four scenarios and reported CLI 1.8.46 / Bootstrap 1.8.50.

## Homebrew

[Tap PR #21](https://github.com/tt-a1i/homebrew-skillroster/pull/21) prepares the
v1.8.46 Formula at `cb765eec1133ca389f67e290b16b512eade7dbc0`.
The source archive SHA-256 is
`bf4ac02bc8497f03bf5c2c2b863248d50342b2e35990cb3aeb591a59d1c662f6`.
The [test-bot run](https://github.com/tt-a1i/homebrew-skillroster/actions/runs/38704579468)
checks macOS and Linux builds, installation, and runtime behavior. Bottle
publication is pending; the GitHub binary Release alone does not establish
that the Homebrew tap has updated.

## Boundaries

SQLite and JSON envelope schema versions are unchanged. Compact reports omit
informational Findings; full and paged Finding interfaces preserve access.
Suggested-action order is not a usefulness ranking. Current Bootstrap files do
not prove that an already-running Agent loaded them.

These checks use temporary synthetic estates and establish controlled behavior
and recovery, not independent-user understanding or live Agent compliance.
The user's installed CLI and Agent files were not upgraded. WSL2 uses the Linux
archive; WSL1 mutation, native Linux arm64, and native Windows arm64 remain outside
the release platform matrix. Public website deployment is verified separately
from binary publication.
