'use strict'

import React from 'react'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import Send from '../components/Send'

const mapStateToProps = (state, ownProps) => ({
  nextButtonTitle: 'Next',
  nextButtonDescription: 'Choose account',
})

const mapDispatchToProps = (dispatch, ownProps) => ({
  nextButtonAction: () => {
    Actions.select()
  }
})

const ConfirmTransaction = connect(mapStateToProps, mapDispatchToProps)(Send)

export default ConfirmTransaction
