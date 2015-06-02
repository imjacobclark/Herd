echo 1000000000000 > /proc/sys/kernel/pid_max
echo 1000000000000 > /proc/sys/kernel/threads-max
echo '*       soft    nofile  1000000000000 \n*       hard    nofile  1000000000000' >>  /etc/security/limits.conf
echo 1000000000000 > /proc/sys/fs/nr_open
echo 1000000000000 > /proc/sys/fs/file-max