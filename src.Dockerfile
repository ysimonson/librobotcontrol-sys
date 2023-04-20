# rustfmt disabled for now, as it seems to cause bindgen-cli to never finish
FROM --platform=linux/arm/v7 alpine:3.17 AS build
RUN apk update && apk add --no-cache binutils cmake make libgcc linux-headers musl-dev gcc g++ clang clang-dev rust cargo
RUN cargo install bindgen-cli
COPY ./librobotcontrol/library /app/

WORKDIR /app
RUN make
RUN make install
RUN ~/.cargo/bin/bindgen --help
RUN ~/.cargo/bin/bindgen include/robotcontrol.h -o bindings.rs \
    --with-derive-default \
    --no-prepend-enum-name \
    --allowlist-function="rc_.+" \
    --allowlist-type="rc_.+"

FROM scratch AS output
COPY --from=build /app/bindings.rs .
