FROM debian:latest
MAINTAINER tobias.schottdorf@gmail.com

RUN apt-get update
RUN apt-get install -y libssl1.0.0

COPY target/debug /work

WORKDIR /work

ENTRYPOINT "/work/cockroach"
