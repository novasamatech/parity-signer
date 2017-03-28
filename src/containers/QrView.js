'use strict'

import { connect } from 'react-redux'
import QrView from '../components/QrView'

const mapStateToPropsTransaction = (state, ownProps) => ({
  text: state.transactions.signedTransaction.signature,
  view: true
})

export const QrViewTransaction = connect(mapStateToPropsTransaction)(QrView)
