# badfsfuzzer
fuzzing btrfs with Janus and LKL

```
git clone https://github.com/docfate111/badfsfuzzer.git
cd linux/tools/lkl
fallocate -l 128M disk.img
sudo mkfs.ext4 -F disk.img
make && ./fsfuzz -i disk.img -p -t ext4
```
