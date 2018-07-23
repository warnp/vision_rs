FROM ubuntu:16.04

RUN apt-get update \
    && apt-get install -y curl file build-essential \
    && curl https://sh.rustup.rs -sSf | sh


ENV PATH "/root/.cargo/bin:$PATH"

RUN mkdir -p /rust/app
ADD content /rust/app/content
ADD target/release/vision /rust/app/target/release/vision
WORKDIR /rust/app
RUN mkdir -p content/images

CMD "./target/release/vision"
