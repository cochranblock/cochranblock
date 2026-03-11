# cochranblock

Monorepo: approuter, cochranblock, oakilydokily, rogue-repo, kova, and related projects. Railway deployment.

Proof of Artifacts: see each project README (cochranblock, oakilydokily, rogue-repo, etc.) and [docs/PROOF_OF_ARTIFACTS.md](docs/PROOF_OF_ARTIFACTS.md).

## Build from workspace

```bash
./scripts/build-monorepo.sh /path/to/workspace/root
```

## Railway setup

1. Create project at [railway.com](https://railway.com)
2. Add 4 services + Postgres
3. Connect this repo (cochranblock/cochranblock)
4. Set Root Directory per service: approuter, cochranblock, oakilydokily, rogue-repo
5. Add env vars per [approuter/docs/RAILWAY.md](approuter/docs/RAILWAY.md)

## GitHub

Push to cochranblock/cochranblock (create repo first if needed).
