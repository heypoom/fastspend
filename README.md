# FastSpend

Log your daily spending lightning fast with short text snippets!

FastSpend is a tool to log your spending in seconds, powered by a lightning fast API written in Rust.

## Stack

- The frontend is built with Svelte and TypeScript as a PWA (Progressive Web Application).

- The API is built with Rust, Cloudflare Workers and PostgreSQL.
  - The only reason I went with PostgreSQL in this case is because I haven't wrote for a very long time, as I always use Prisma in my other projects. If I were to implement this as a production project, I would go with NoSQL instead as we don't need a lot of relations to store the configuration state.

## Usage

TBA

## Syntax

- Use keywords to add spending quickly
  - Example: "100f"
  - The keyword can either represent a category (e.g. f = food), or a payee (e.g. kfc = kentucky fried chicken)

- Use the exclamation mark (!) to add modifiers:
  - Example: "100f!t"
  - Outflow Modifiers:
    - credit card: !c (default)
    - bank transfer: !t
    - debit card: !d
  - Inflow Modifiers:
    - income: !i
    - refund or reimbursement: !r

- Use the at sign (@) to explicitly denote that the keyword represents a payee
  - Example: "100kfc"
  - this is used when the keyword is ambiguous, and we're not sure if it is a category or a payee

- Use the at sign (@) with the colon sign (:) to add a new payee
  - Example: "100f@sb : Starbucks" associates the "sb" keyword with the "Starbucks" payee, and utilizes the "Food" (f) category

- Batch add multiple transactions with comma (,):
  - Example: "780kfc, 550kfc, 110s" registers 3 transactions at once, with different amount and keywords.
  - Spaces are optional

## Inspiration

The initial inspiration is from [@dtinth](https://dt.in.th)'s excellent talk on "[Let’s build a personal assistant and level-up your coding skills!](https://dt.in.th/personal-assistant.html)". In this talk, Thai creates a LINE chatbot to log his spending and do various things.

I'm using the You Need a Budget (YNAB) app to manage my budgets, but their app takes 4 - 5 taps to register one transaction - I'm way too lazy to do that.

## Roadmap

- [ ] Setup Cloudflare Workers

- [ ] Implement Syntax Highlighting in PWA
- [ ] Implement Autocompletion in PWA
- [ ]


