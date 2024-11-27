sleep 1
chmod +x $1
cp -f "$1" "$2"
rm -r $(dirname $1)
cd $(dirname $2)
$2 &
