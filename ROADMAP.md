# Service Roadmap

## Version 0.1

- Hardcoded challenge list
- Minimal API to return models
- Dockerfile for container platforms

## Version 0.2

- Full challenge model implementation
- Filtering searches with query parameters
- Runnable on K8's using Replicaset

## Version 0.3

- Admin API for imports and DB backups
- Basic auth configured with environment variables
- Separate SQL backend node with env var config

## Version 0.4

- Manually documented API in repo wiki
- Full CRUD through API with validation
- Pipeline to run linter and tests

## Version 0.5

- Docker image created through CI
- Token based authentication for admin ops
- Separate Redis backend node for session caching

## Version 0.6

- Health endpoint for server stats
- Helm chart for installation on K8's
- Documented operations and alert suggestions

## Version 1.0

- Full deployment code, including metrics, logging, and alerts
- Pipeline publish/roll-out to K8's cluster for public access
