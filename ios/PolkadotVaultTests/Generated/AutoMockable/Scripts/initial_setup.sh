BASEDIR=$(dirname "$0")
HEADER=$BASEDIR/AutoMockableHeader.swift

for letter in {A..Z}
do
  FILE=$BASEDIR/../AutoMockable+$letter.generated.swift
  if [ ! -f "$FILE" ]; then
    touch $FILE
  fi
  if [[ -s "$FILE" ]]; then
    cat $HEADER $FILE >$BASEDIR/out && mv $BASEDIR/out $FILE
  fi
done
