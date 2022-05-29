FROM archlinux
RUN useradd -m app
USER app
WORKDIR /home/app/voki
COPY --chown=app:app ./voki ./
ENTRYPOINT ./http
