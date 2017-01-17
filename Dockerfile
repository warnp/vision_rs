FROM ubuntu:16.04

RUN apt-get update \
    && apt-get install -y curl file build-essential


ENV PATH "/root/.cargo/bin:$PATH"

RUN mkdir -p /rust/app
ADD . /rust/app
WORKDIR /rust/app
RUN mkdir -p content/images

CMD "./target/release/vision"
