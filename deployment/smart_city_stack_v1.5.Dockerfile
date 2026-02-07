FROM alpine:3.20
RUN apk add --no-cache ca-certificates
WORKDIR /smart_city_stack
COPY target/release/aln-terminal ./aln-terminal
ENV ALN_STACK_ROLE=smart_city_augmented_citizen
ENTRYPOINT ["./aln-terminal", "--command", "exec.acts.sys.maintenance_v1.5"]
