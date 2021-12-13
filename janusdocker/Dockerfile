FROM rsmmr/clang:6.0

# Setup environment.
ENV PATH /opt/clang/bin:$PATH

# Default command on startup.
CMD bash

# Setup packages.
RUN apt-get update && apt-get install -y bc cmake git build-essential vim python libncurses5-dev libedit-dev libpthread-stubs0-dev bison flex libboost-dev unzip python-pip && \
pip install xattr pathlib2

# Copy install-clang over.
ADD . /opt/install-clang

# Compile and install Clang/LLVM. We delete the source directory to
# avoid committing it to the image.
RUN /opt/install-clang/install-clang -j$(nproc) -C /opt/clang && git clone https://github.com/sslab-gatech/janus.git && \
cd janus && cd ff-gcc && make && cd ../core && make && \
cd ../lkl && ./compile -t btrfs -c && cd ../fs/btrfs && \
make && cd ../.. && unzip samples.zip && cd istat && \
# paste in the rest bc istat command usually gives error(can't fix this)
./istat -i ../samples/evaluation/btrfs-00.image -t btrfs -o btrfs.istat || cd .. && \
mkdir prog && ./core/create_corpus istat/btrfs.istat prog  && \
mkdir input output && \
echo core >/proc/sys/kernel/core_pattern && \
cd /sys/devices/system/cpu && \
echo performance | tee cpu*/cpufreq/scaling_governor && \
cd /janus && \
./core/afl-image-syscall/afl-fuzz -b btrfs -s fs/btrfs/btrfs_wrapper.so -e ./samples/evaluation/btrfs-00.image -S btrfs -y prog -i input -o output -m none -u 2 -- ./lkl/tools/lkl/btrfs-combined -t btrfs -p @@ && \
