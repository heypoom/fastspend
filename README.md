# FastSpend

Log your daily spending lightning fast with short text snippets! FastSpend is a tool to log your spending in seconds, powered by a lightning fast API written in Rust.

FastSpend aims to be service-agnostic, so you can log your spending to multiple sinks. Currently, it is implemented for the You Need A Budget API (YNAB API). Soon, we may offer plugins for logging to Airtable, Firebase, Plain Text, etc.

FastSpend also aims to provide various frontends to log your spending. Currently, we offer a Svelte-powered progressive web app. Soon, we may implement a Rust-powered CLI, Raycast Extension and a chatbot,

## Syntax

- Use keywords to add spending quickly

  - Example: `100f`
  - The keyword can either represent a category (e.g. `f` is food), or a payee (e.g. `kfc` is Kentucky Fried Chicken)

- Use the exclamation mark (`!`) to add modifiers:

  - Example: `100f!t`
  - Outflow Modifiers:
    - credit card: `!c` (default payment method)
    - bank transfer: `!t`
    - debit card: `!d`
  - Inflow Modifiers:
    - income: `!i`
    - refund or reimbursement: `!r`

- Use the colon (`:`) sign to associate a payee. This is useful when the payee does not yet have a keyword alias.

  - Example: `400s : Starbucks` associates the transaction with the Starbucks payee.

- Use the hyphen sign (`-`) to add a memo, so you can remember what the transaction is for.

  - Example: `100f - ice cream truck`
  - You can use it with the payee syntax: `400s : Starbucks - venti caramel macchiato`

- Use the at sign (`@`) in conjunction with the colon sign (`:`) to register a new payee alias

  - Example: `100f@sb : Starbucks` associates the `sb` keyword with the `Starbucks` payee, and utilizes the "Food" (`f`) category

- Batch add multiple transactions with the comma sign (`,`):
  - Example: `780kfc, 550kfc, 110s` registers 3 transactions at once, with different amount and keywords.
  - Spaces are optional

## Inspiration

The initial inspiration is from [@dtinth](https://dt.in.th)'s excellent talk on "[Let’s build a personal assistant and level-up your coding skills!](https://dt.in.th/personal-assistant.html)". In this talk, Thai creates a LINE chatbot to log his spending and do various things.

I'm using the You Need a Budget (YNAB) app to manage my budgets, but their app takes 4 - 5 taps to register one transaction - I'm way too lazy to do that.

## Technology

- The frontend is built with Svelte and TypeScript as a PWA (Progressive Web Application).
- The API is built with Rust and Cloudflare Workers.

## Roadmap

- [x] Setup Cloudflare Workers locally
- [x] Deploy Cloudflare Workers to production for personal usage
- [x] Maintain database of keywords (budget, payee)
- [x] Use a parser generator to parse our syntax, e.g. with Pest, Tree-sitter, or just plain 'ol Regex!
- [x] Write some unit tests in Rust as a starting point
- [ ] Extract controller to it's own testable unit, with unit tests
  - [ ] Create a standalone Rust library for parsing commands and invoking event handlers given the configuration
- [ ] Let's try TDD! (test-driven development)
- [ ] Host the configuration data in Durable Objects or KV
- [ ] Allow configuring different YNAB Token and Budget ID
  - [ ] Generate personal access token for each FastSpend users

**Frontends**

- [ ] Raycast Command
- [ ] Rust-powered CLI to call the API, built with Rust. e.g. `sf 1000sb`
- [ ] Chatbot Integration - Messenger or LINE?
- [ ] PWA to add and modify keyword mapping dynamically; should export as JSON for configuration.
- [ ] iOS widget or application, powered by Swift?

**Syntax Support**

- [x] Batch transactions with `,`
- [x] Use colons (`:`) to add payee name
- [x] Use hyphen (`-`) to add a memo
- [x] Use `!` to add modifier: `!t` for transfer, `!r` for refund/reimburse, `!d` for debit, `!i` for income
- [x] Keyword can be either budget or payee.
- [x] Use `$` to add tags, e.g. `$5` is 5 star rating
- [ ] Use emojis as keyword - e.g. ☕️ for Starbucks
- [ ] Command to register budget categories - e.g. `!c f Food, 1689063412`
- [ ] Command to register payee - e.g. `!p fc Factory Coffee $ Drinks`

**Logging Sources**

- [x] YNAB API (You Need A Budget API)
- [ ] Airtable
- [ ] Firebase

**Webapp Features**

- [ ] Implement Syntax Highlighting in PWA
- [ ] Implement Autocompletion in PWA
- [ ] Implement standard PWA features (e.g. offline-first support, manifest)

**Write-up**

- [ ] Write a Blog on this

**Features Ideas**

- [ ] Automatically log spending from notification interception or web scraping, automatically sync and match
- [ ] Pre-populated widget UI with spending behaviour, use previous transaction data or logged widget/spending group. One-tap to log spending from history.
- [ ] Multiple logging backends as Rust plugin: YNAB API, Airtable, Plain Text, Firebase. Should be able to write unit test
- [ ] Natural language input instead of command-based input
