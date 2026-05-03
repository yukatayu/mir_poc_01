# practical-alpha1 docker

This directory holds the current practical alpha-1 Docker transport fixture.

- `docker-compose.practical-alpha1.yml`
  - used by `scripts/practical_alpha1_transport.py` for `TR-A1-02`
  - mounts the already-built `mir_practical_alpha1_transport` example binary,
    the selected practical package directory, and an output directory
  - runs a world server container plus a participant client container over
    Docker Compose TCP

This is a non-final practical transport floor only.
It does not imply WAN/federation, save/load, devtools export, product
prototype, or final public transport ABI completion.
