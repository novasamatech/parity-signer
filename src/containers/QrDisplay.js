'use strict'

import React from 'react'
import { connect } from 'react-redux'
import QrView from '../components/QrView'

const mapStateToProps = (state, ownProps) => ({
	text: state.transactions.signedTx,
})

const QrDisplay = connect(mapStateToProps)(QrView)

export default QrDisplay
