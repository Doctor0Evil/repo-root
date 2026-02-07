# ALN Advanced Rego & Text Game Integration v1.5

## Overview
This module binds Rego policy comprehensions, sets, and document models into an ALN-driven game separation layer, while mapping text navigation and turn-based mechanics into chat-frame rendering primitives.

## Features
- Rego comprehensions for dense policy data filtering in game separation logic.
- Sets for unique membership checks across roles, groups, and sessions.
- Document models for queryable policy surfaces usable by ALN runtimes.
- Deny rules and conflict checks for infrastructure-level gatekeeping.
- Text adventure navigation and input parsing adapted to ALN chat interop.
- Turn-based battle loops separated as declarative policy state machines.

## Usage
Load via an ALN-compatible runtime that mounts `advanced_rego_integration_v1.5.aln` and `advanced_separation_policy_v1.5.rego`, then connect chat surfaces to the ALN evolution service at version `1.0.1.5`.

## Deployment
Use the `deployment/aln_advanced_deploy_v1.5.Dockerfile` image as a policy sidecar next to retail POS, game servers, or chat backends, with full-service LAN networking enabled through the host platform.
