sleep 1
chmod +x $1
cp -f "$1" "$2"
$3 &
rm -r $(dirname $1)

