FROM alpine:3.20 AS base
RUN apk add --no-cache opa godot-export-templates kafka postgresql-client redis
WORKDIR /aln_advanced
COPY advanced_separation_policy_v1.5.rego .
COPY advanced_rego_integration_v1.5.aln .
ENV ALN_VERSION=1.0.1.5
ENTRYPOINT ["opa", "run", "--server", "--bundle", "/aln_advanced", "--addr=0.0.0.0:8181"]
