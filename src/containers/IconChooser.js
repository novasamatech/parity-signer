'use strict'

import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import IconChooser from '../screens/IconChooser'

const IconChooserContainer = connect(
  undefined,
  (dispatch, ownProps) => ({
    onSelect: (seed) => {
      Actions.accountNew({ seed })
    }
  })
)(IconChooser)

export default IconChooserContainer
