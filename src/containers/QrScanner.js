import React, { Component } from 'react'
import { connect } from 'react-redux'
import Scanner from '../components/Scanner'

const onBarCodeRead = () => ({})

const readBarCode = (dispatch, ownProps) => ({
  onBarCodeRead: () => {
    dispatch(onBarCodeRead())
  }
})

const QrScanner = connect(
  readBarCode
)(Scanner)

export default QrScanner
