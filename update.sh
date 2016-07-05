wget https://sourceforge.net/projects/tinyfiledialogs/files/tinyfiledialogs.h/download -O tinyfiledialogs.h
tr -d '\r' < tinyfiledialogs.h > libtinyfiledialogs/tinyfiledialogs.h
wget https://sourceforge.net/projects/tinyfiledialogs/files/tinyfiledialogs.c/download -O tinyfiledialogs.c
tr -d '\r' < tinyfiledialogs.c > libtinyfiledialogs/tinyfiledialogs.c
