# petabroad project

## Quick start

### Via Docker

1. Create DB persistant storage: `mkdir -p $(pwd)/data/db`
1. Build frontend container: `cd frontend && docker build . -t alexmikhalevich/patabroad-frontend:dev`
1. Build backend container: `cd backend && docker build . -t alexmikhalevich/patabroad-backend:dev`
1. Use docker compose to execute services: `docker-compose up`

### Locally

1. Create DB persistant storage: `mkdir -p $(pwd)/data/db`
2. Run MongoDB: `docker run -v $(pwd)/data/db:/data/db mongo:5.0.4`
3. Set backend environment variables:
```bash
export PETABROAD_BACKEND_ADDRESS=0.0.0.0:8081
export MONGODB_DB_NAME=petabroad
export MONGODB_USER=test
export MONGODB_PASSWORD=test
export MONGODB_ENDPOINT=127.0.0.1:27017
```
4. Run backend: `cd backend && cargo run`
5. Run frontend: `cd frontend && trunk serve`
