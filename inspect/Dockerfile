FROM debian:stretch-20190910 as service

RUN apt-get update &&\
    apt-get install -y kafkacat &&\
    apt-get clean && rm -rf /var/lib/apt/lists/*

CMD tail -f /dev/null
