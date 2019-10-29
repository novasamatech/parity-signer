#!/bin/bash
#
# fix-rncamera-search-paths.sh
#
# Fix Frameworks/Headers Search Path build settings in react-native-camera/ios Xcode project
# First argument is path to RNCamera xcode project file, e.g.:
#   ./node_modules/react-native-camera/ios/RNCamera.xcodeproj/project.pbxproj
#

SRC_FILE="$1"

if [ ! -f ${SRC_FILE} ]; then
    echo "Error: RNCamera xcodeproj not found at path: ${SRC_FILE}"
    echo "Skipping fix"
    exit 0
fi

perl -i -p0e 's/FRAMEWORK_SEARCH_PATHS = (.*?);/FRAMEWORK_SEARCH_PATHS = "\$(SRCROOT)\/..\/..\/..\/ios\/Pods\/Headers\/**\/**";/gs' ${SRC_FILE}

perl -i -p0e 's/HEADER_SEARCH_PATHS = (.*?);/HEADER_SEARCH_PATHS = "\$(SRCROOT)\/..\/..\/react-native\/React\/**";/gs' ${SRC_FILE}

perl -i -p0e 's/LIBRARY_SEARCH_PATHS = (.*?);/LIBRARY_SEARCH_PATHS = "\$(SRCROOT)\/..\/..\/..\/ios\/Pods\/Headers\/**\/**";/gs' ${SRC_FILE}

echo "Fixed RNCamera xcodeproj at path: ${SRC_FILE}"
