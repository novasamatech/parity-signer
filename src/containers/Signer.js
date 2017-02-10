import React from 'react'
import { connect } from 'react-redux'
import Scanned from '../components/Scanned'

const Signer = connect(state => ({
  transactions: state.transactions
}))(Scanned)

export default Signer
