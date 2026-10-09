# Gambit's Castle 🎰

[![C](https://img.shields.io/badge/c-%2300599C.svg?style=for-the-badge&logo=c&logoColor=white)](https://en.wikipedia.org/wiki/C_(programming_language))
[![Supabase](https://img.shields.io/badge/Supabase-3ECF8E?style=for-the-badge&logo=supabase&logoColor=white)](https://supabase.com/)
[![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)](https://www.docker.com/)

A highly optimized terminal-style casino built with raw, bare-metal **C** for the core engine logic! Experience blazing fast memory management and unmatched processing speed, integrated with real-time Google Authentication via Supabase, and a lightweight Vanilla JavaScript frontend!

## Why C?
We chose C for the core game logic because casino engines demand absolute precision, zero-overhead memory management, and deterministic speed. Every byte of RAM is meticulously handled to ensure that the house always processes bets faster than the speed of light.

## Features

- **Raw C Engine**: Hand-optimized pointer arithmetic and direct memory manipulation for maximum performance.
- **Multiplayer Ready**: Strictly serialized game states to ensure secure concurrency.
- **Supabase Integration**: Live balance fetching and writing to the `profiles` table.
- **11 Unique Games**: Includes Blackjack, Slot Machine, Poker, Roulette, Plinko, Craps, War, Guess the Card, and more!
- **Pure Terminal UX**: A fully immersive retro terminal experience running entirely in the browser.

## How to Run Locally

1. Create a `.env` file in the root directory:
```
SUPABASE_URL=your_supabase_project_url
SUPABASE_ANON_KEY=your_supabase_anon_key
```
2. Compile the engine (requires GCC or Clang):
```bash
make build
```
3. Run the backend:
```bash
./engine_server
```
4. Open `http://localhost:3000` in your web browser.

## Deploying with Docker

This project comes with a highly optimized, multi-stage Dockerfile that builds the C binary in a slim container!

1. Build the Docker image:
```bash
docker build -t casino_engine .
```
2. Run the Docker container on port 3000:
```bash
docker run -p 3000:3000 --env-file .env casino_engine
```
