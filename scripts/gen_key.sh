FILE=./android/app/debug.keystore
if [ -f "$FILE" ]; then
    echo "$FILE exist, skip.."
    else
    echo "generating andriod debug keystore file.."
    cd ./android/app && keytool -genkey -v -keystore debug.keystore -storepass android -alias androiddebugkey -keypass android -keyalg RSA -keysize 2048 -validity 10000

fi




