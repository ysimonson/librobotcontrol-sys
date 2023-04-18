FROM --platform=linux/arm/v7 alpine:3.17 AS build
RUN apk update && apk add --no-cache binutils cmake make libgcc linux-headers musl-dev gcc g++ clang clang-dev rust cargo
RUN cargo install bindgen-cli
COPY ./librobotcontrol/library /app/
WORKDIR /app
RUN make
RUN make install
RUN ar rcs lib/librobotcontrol.a build/led.o build/pthread.o build/mpu/mpu.o build/deprecated.o build/model.o build/cpu.o build/pru/servo.o build/pru/encoder_pru.o build/pru/pru.o build/button.o build/encoder.o build/mavlink_udp.o build/io/spi.o build/io/gpio.o build/io/i2c.o build/io/adc.o build/io/encoder_eqep.o build/io/pwm.o build/io/uart.o build/math/algebra.o build/math/vector.o build/math/filter.o build/math/other.o build/math/polynomial.o build/math/ring_buffer.o build/math/algebra_common.o build/math/kalman.o build/math/matrix.o build/math/quaternion.o build/time.o build/version.o build/dsm.o build/bmp/bmp.o build/pinmux.o build/motor.o build/start_stop.o
RUN ~/.cargo/bin/bindgen include/robotcontrol.h -o bindings.rs --with-derive-default

FROM scratch AS output
COPY --from=build /app/bindings.rs .
COPY --from=build /app/lib/librobotcontrol.a .
