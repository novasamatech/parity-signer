'use strict'

import React from 'react'
import { connect } from 'react-redux'
import QrView from '../components/QrView'

const mapStateToPropsTransaction = (state, ownProps) => ({
	text: state.transactions.signedTx,
})

export const QrViewTransaction = connect(mapStateToPropsTransaction)(QrView)

const mapStateToPropsAddress = (state, ownProps) => ({
  text: state.accounts.selected.address
})

export const QrViewAddress = connect(mapStateToPropsAddress)(QrView)
