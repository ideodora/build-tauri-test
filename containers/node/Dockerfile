# base image to build a JRE
FROM openjdk:21-slim as java

# required for strip-debug to work
RUN apt-get update && apt install -fy binutils

# Build small JRE image
RUN $JAVA_HOME/bin/jlink \
  --verbose \
  --add-modules ALL-MODULE-PATH \
  --strip-debug \
  --no-man-pages \
  --no-header-files \
  --compress=2 \
  --output /customjre

FROM node:18-slim

ENV JAVA_HOME=/jre
ENV PATH="${JAVA_HOME}/bin:${PATH}"

# copy JRE from the base image
COPY --from=java /customjre $JAVA_HOME

# do whatever u want ....

# pubsub need ps to SIGKILL
RUN apt-get update && apt install -fy procps \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*

RUN npm i -g --omit=optional firebase-tools@^11.21.0
