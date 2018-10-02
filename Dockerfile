FROM ubuntu:16.04

ENV PATH "/root/.cargo/bin:$PATH"

RUN mkdir -p /rust/app
#ADD content/template /rust/app/content/template
#ADD content/js /rust/app/content/js
#ADD content/style /rust/app/content/style
#ADD content/data /rust/app/content/data
ADD build/release/vision /rust/app/target/release/vision
WORKDIR /rust/app

EXPOSE 8000

RUN mkdir -p content/images
CMD "./target/release/vision"
#CMD cargo run --release
