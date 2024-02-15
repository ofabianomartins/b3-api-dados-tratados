FROM docker.io/ruby:3.1.0-alpine3.14

USER root

EXPOSE 3000
EXPOSE 5100

RUN apk add --no-cache bash && apk add --update \
      build-base \
      curl \
      curl-dev \
      libpthread-stubs \
      tzdata \
      zlib-dev \
      linux-headers \
      mysql-dev \
      && rm -rf /var/cache/apk/*

RUN gem install bundler -v 2.2.27

WORKDIR /app

COPY Gemfile /app
COPY Gemfile.lock /app

RUN bundle install

COPY start_process.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/start_process.sh

COPY ./entrypoint.sh /usr/bin/entrypoint.sh
RUN chmod +x /usr/bin/entrypoint.sh
ENTRYPOINT ["/usr/bin/entrypoint.sh"]

COPY . /app

CMD ["start_process.sh"]
