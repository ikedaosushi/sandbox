FROM python:3.6.5

WORKDIR /root/

ADD Pipfile Pipfile
RUN pip install -U pip \
    && pip install pipenv \
    && pipenv install --system --skip-lock

ADD slack_call.py slack_call.py