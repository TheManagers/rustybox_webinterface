
FROM rust:1.24.1
LABEL Name=rustybox_webinterface Version=0.0.1
ADD . /app
WORKDIR /app
EXPOSE 3000
CMD [ "cargo" ,"run"]
