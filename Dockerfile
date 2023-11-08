FROM mcr.microsoft.com/vscode/devcontainers/python:3.11-bullseye as builder

FROM mcr.microsoft.com/vscode/devcontainers/rust:1-bullseye

COPY --from=builder /usr/local/ /usr/local/

COPY requirements.txt /tmp/pip-tmp/
RUN pip3 --disable-pip-version-check --no-cache-dir install -r /tmp/pip-tmp/requirements.txt \
   && rm -rf /tmp/pip-tmp