target remote :2331
#set tdesc filename target-m3.xml
mon flash download = 1
#mon flash device = EFM32GG990F1024
set mem inaccessible-by-default off
#set remote memory-read-packet-size 1200
#set remote memory-read-packet-size fixed
mon speed 4000
mon endian little
mon reset 1
tbreak main
cont
