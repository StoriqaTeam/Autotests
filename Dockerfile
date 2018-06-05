FROM python:3-slim

COPY . /root
WORKDIR /root
RUN pip install -r deps.txt

ENTRYPOINT ["python"]
