#!/bin/sh
sudo docker build . -f Dockerfile -t fuzzy:latest
sudo docker run -it --rm --privileged --cap-add=SYS_PTRACE --security-opt seccomp=unconfined -v $(pwd):/home fuzzy:latest
