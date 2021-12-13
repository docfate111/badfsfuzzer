FROM ubuntu:xenial
ENV LLVM_CONFIG=llvm-config-6.0
RUN apt update -y && apt install -y make xz-utils wget bison flex g++ bc git cmake python && \
	wget https://releases.llvm.org/6.0.0/cfe-6.0.0.src.tar.xz && \
	wget https://releases.llvm.org/6.0.0/llvm-6.0.0.src.tar.xz && \
	tar xJf cfe-6.0.0.src.tar.xz && tar xJf llvm-6.0.0.src.tar.xz && rm *xz && \
	cd cfe-6.0.0.src && mkdir build && cd build && cmake ../../llvm-6.0.0.src/ && \
	cmake -j8 --build .
#g++ libboost-dev && \
#    git clone https://github.com/sslab-gatech/janus.git && \ 
#	cd janus/ff-gcc && make && cd ../core  && make && \
#	cd ../lkl && ./compile -t btrfs -c && \
