FROM python:3-onbuild

EXPOSE 3000
COPY ./src /usr/src/app
WORKDIR /usr/src/app/src

CMD ["gunicorn", "--bind", "0.0.0.0:3000", "main:app"]
