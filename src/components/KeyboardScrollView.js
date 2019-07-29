import React from "react";
import {Keyboard, Platform} from "react-native";
import { KeyboardAwareScrollView } from "react-native-keyboard-aware-scroll-view";


//TODO change to normal react element
class KeyboardScrollView extends React.Component {

  render() {
    const defaultProps = {enableAutomaticScroll: true, extraHeight: 200};
    return Platform.select({
      ios: <KeyboardAwareScrollView
        keyboardDismissMode="interactive"
        keyboardShouldPersistTaps="handled"
        {...defaultProps}
        {...this.props}
      />,
      android: <KeyboardAwareScrollView
        keyboardDismissMode="on-drag"
        onScrollEndDrag={ Keyboard.dismiss }
        keyboardShouldPersistTaps="handled"
        enableOnAndroid
        {...defaultProps}
        {...this.props}
      />
    })
  }
};

export default KeyboardScrollView;
