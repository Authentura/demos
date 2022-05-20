FROM php:7.4-cli

EXPOSE 3002
RUN mkdir /app
COPY . /app
WORKDIR /app/challenge

RUN apt update
RUN apt install apt-utils curl zip

RUN mv /app/php.ini "$PHP_INI_DIR/php.ini"

CMD [ "php", "-S", "0.0.0.0:3002" ]
