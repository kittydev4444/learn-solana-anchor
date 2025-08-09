# Solana Anchor Monorepo

A single repo with many small projects to learn Solana + Anchor step by step — like LEGO blocks 🧱.  
Each project lives in `projects/<project-name>` and is self-contained (code, tests, deploy steps).

---

## Table of Contents

1. Purpose
2. Project Roadmap
3. Folder Structure
4. Prerequisites
5. Quick Start
6. Add a New Project
7. Run & Test a Project
8. Devnet Deploy
9. Git Rules (Monorepo)
10. Troubleshooting
11. Learning Notes
12. Resources
13. FAQ (Kid Mode)

---

## Purpose

- Learn Anchor + Solana by building non-repetitive mini-projects.
- Keep every project isolated so it’s easy to focus and test.
- Use one Git repo at the root (no nested `.git` inside projects).
- Move from beginner → intermediate → advanced with real program concepts.

---

## Project Roadmap

(We’ll open a separate ChatGPT conversation for each project with the project title.)

| #   | Project               | Purpose (kid-friendly) | What you’ll learn                         |
| --- | --------------------- | ---------------------- | ----------------------------------------- |
| 1   | hello-solana          | Say “hi” on-chain      | Program entrypoint, logs, build/test flow |
| 2   | counter-program       | Count 1, 2, 3…         | PDAs, state accounts, Borsh               |
| 3   | todo-list             | Save a list of tasks   | Vectors, account resizing                 |
| 4   | token-mint            | Make your own coin     | SPL Token CPI, PDA authority              |
| 5   | vault-program         | Piggy bank rules       | PDA vaults, authority checks              |
| 6   | nft-minter            | Mint NFTs + metadata   | Metaplex CPI basics                       |
| 7   | multisig-wallet       | Team approvals         | Multisig logic, `require!`                |
| 8   | escrow-marketplace    | Safe swaps             | Escrow, timelocks                         |
| 9   | dao-voting            | Vote on-chain          | Voting/tally, PDA seeds                   |
| 10  | lending-protocol-lite | Borrow/lend            | Math safety, liquidation basics           |
| 11  | defi-dex-lite         | Swap tokens            | Pools, constant-product curve             |
| 12  | game-on-chain         | Scoreboard fun         | Multiple PDAs, events                     |
| 13  | final-capstone        | Full-stack dApp        | Combine everything                        |

We’ll generate each project’s own README.md (purpose, learning goals, steps) when you start it.

---

## Folder Structure

    solana-anchor-monorepo/
    ├─ .git/                     # single root Git repo
    ├─ .gitignore                # ignore rules for all projects
    ├─ README.md                 # THIS file
    └─ projects/
       ├─ hello-solana/
       │  ├─ Anchor.toml
        │  ├─ Cargo.toml
       │  ├─ programs/
       │  │  └─ hello-solana/
       │  │     ├─ Cargo.toml
       │  │     └─ src/lib.rs
       │  ├─ tests/              # integration tests (TS/JS)
       │  │  └─ hello-solana.ts
       │  └─ migrations/
       └─ counter-program/
          ├─ Anchor.toml
          ├─ Cargo.toml
          ├─ programs/
          │  └─ counter-program/
          │     ├─ Cargo.toml
          │     └─ src/lib.rs
          ├─ tests/
          │  └─ counter-program.ts
          └─ migrations/

---

## Prerequisites

- macOS (Apple Silicon OK)
- Rust: `rustup`, `cargo`
- Solana CLI: `solana --version`
- Anchor CLI: `anchor --version`
- Node.js + pnpm: `pnpm -v`
- Wallet: Phantom (for devnet deploys)

Tip: Use devnet first. You can airdrop SOL for testing.

---

## Quick Start

    # 1) clone your repo
    git clone <your-repo-url> solana-anchor-monorepo
    cd solana-anchor-monorepo

    # 2) confirm toolchain
    solana --version
    anchor --version
    rustc --version
    pnpm -v

    # 3) set Solana cluster to devnet for deploys (we'll use local validator during tests)
    solana config set --url https://api.devnet.solana.com

    # 4) create the projects workspace folder if you don’t have it
    mkdir -p projects

---

## Add a New Project

(Always create projects inside `projects/`. Anchor will create a `.git` inside — delete it. We only keep the root `.git`.)

    cd projects
    anchor init <project-name>
    cd <project-name>

    # Remove nested git (Anchor auto-inits it)
    rm -rf .git

    # Go back to root and commit
    cd ../..
    git add .
    git commit -m "feat: add <project-name> scaffold"

Naming rules:

- Program directory in `programs/<program-name>` should match the crate/program name in code.
- Update `Anchor.toml` if you rename anything.

---

## Run & Test a Project

(Anchor tests run against a local validator spun up for you — clean sandbox.)

    cd projects/<project-name>
    pnpm i        # if your tests use TS deps; otherwise skip
    anchor build
    anchor test   # spins up local validator, builds, deploys locally, runs tests

Test files live here:
projects/<project-name>/tests/\*.ts

Common test imports (TypeScript):
import \* as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
// import { <TypeName> } from "../target/types/<snake_case_program_name>";

---

## Devnet Deploy

(Use this after your tests pass locally.)

    # Set devnet (once)
    solana config set --url https://api.devnet.solana.com

    # Fund your wallet (if needed)
    solana airdrop 2

    # Build + Deploy
    cd projects/<project-name>
    anchor build
    anchor deploy

Where’s the program ID?

- After `anchor deploy`, you’ll see a program ID (public key).
- Put it in your client/test code as needed or in `Anchor.toml` under `[programs.devnet]`.

---

## Git Rules (Monorepo)

- One repo at root only. No `.git` inside project folders.
- Commit messages use Conventional Commits:
  - `feat: add counter increment instruction`
  - `fix: handle zero lamports`
  - `chore: bump anchor deps`
  - `docs: add project README`
  - `refactor: split PDA helpers`
- Keep commits scoped per project when possible.

---

## Troubleshooting

Anchor created a `.git` inside my project

- Remove it: `rm -rf projects/<project-name>/.git`

Transaction simulation failed: insufficient funds for rent

- Airdrop more devnet SOL: `solana airdrop 2`

Tests can’t find program types

- Run `anchor build` first to generate `target/types/...`

Different Node versions between projects

- Prefer a root `.nvmrc` or use `volta`. Pin versions if needed.

---

## Learning Notes

Each project has its own `README.md` explaining:

- Purpose (what we’re building, like “a counter”)
- What we’ll learn (e.g., PDAs, CPIs)
- Step-by-step instructions
- Tests (`anchor test`)
- Devnet deploy
- Recap (what to remember before next project)

We’ll keep explanations simple and add comments directly in code where helpful.

---

## Resources

- Solana Docs: high-level concepts (accounts, rent, txs)
- Solana Cookbook: copy‑paste friendly snippets
- Anchor Book: macros, accounts, testing
- SPL Token + Metaplex docs: for tokens/NFTs (later projects)

---

## FAQ (Kid Mode)

- Why one repo? So everything stays together and tidy.
- Why delete `.git` inside projects? Because we already have one at the top. Double Git = messy.
- Why tests in `tests/`? Anchor likes integration tests there. It auto-spins a local playground for you.
- Is devnet real money? Nope. It’s play money for practice.

---

Happy building. 🚀
