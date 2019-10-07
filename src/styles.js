// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

import { StyleSheet } from 'react-native';
import colors from './colors';
import fonts from "./fonts";

export default StyleSheet.create({
  bodyH: {
    backgroundColor: colors.bg,
    flex: 1,
    flexDirection: 'column',
    padding: 20,
    overflow: 'hidden'
  },
  bodyContent: {
    paddingBottom: 40
  },
  bodyContainer: {
    flex: 1,
    flexDirection: 'column',
    // justifyContent: 'space-between'
  },
  scrollViewContainer: {
    // backgroundColor: colors.bg,
    // flex: 1,
    // padding: 20,
    //should be changed to b_flex +
  },
  top: {
    flex: 1
  },
  bottom: {
    marginTop: 24,
  },
  safeAreaView: {
    //flex: 1
    //should be changed to b_flex
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    paddingTop: 12,
  },
  menuView: {},
  onboardingWrapper: {
    flex: 1,
    flexDirection: 'row',
    alignItems: 'flex-end'
  },
  onboardingText: {
    fontFamily: fonts.regular,
    fontSize: 20,
    color: colors.bg_text_sec
  },
  seedText: {
    paddingVertical: 12,
    paddingHorizontal: 16, 
    minHeight: 120,
    borderWidth: 0.3,
    borderColor: colors.bg_button,
    color: colors.bg_button,
    backgroundColor: colors.bg,
  },
  seedText_invalid: {
    //borderColor: colors.bg_alert,
    borderColor: colors.bg_text_sec,
  },
  derivationText: {
    minHeight: 30,
    padding: 10,
    fontFamily: fonts.regular,
    backgroundColor: colors.bg,
    borderWidth: 0.3,
    borderColor: colors.bg_button,
  },
  deleteText: {
    color: 'red'
  },
  nextStep: {
    marginTop: 20
  },
  deleteButton: {
    backgroundColor: colors.bg_alert
  },
  button: {
    justifyContent: 'center',
    alignItems: 'center',
    elevation: 4,
    height: 62,
    backgroundColor: colors.bg_button,
    borderRadius: 60
  },
  buttonDisabled: {
    elevation: 0,
    backgroundColor: colors.bg_button_inactive
  },
  pinInput: {
    paddingTop: 12,
    marginBottom: 24,
    color: colors.bg_button,
    backgroundColor: colors.bg,
    borderWidth: 0.3,
    borderColor: colors.bg_button
  },
  changePinText: {
    textAlign: 'left',
    color: 'green'
  },
  qr: {
    marginBottom: 20,
    backgroundColor: colors.card_bg
  },
  card: {
    backgroundColor: colors.card_bg,
    padding: 20,
    borderBottomWidth: 1,
    borderBottomColor: '#000000',
  },
  cardText: {
    // color: colors.card_text,
    // fontFamily: fonts.bold,
    // textAlign: 'center'
    // should be t_h2 + t_bold + t_center
  },
  transactionDetails: {
    flex: 1,
    backgroundColor: colors.card_bg
  },
  message: {
    marginBottom: 20,
    padding: 10,
    height: 120,
    lineHeight: 26,
    fontSize: 20,
    fontFamily: fonts.regular,
    backgroundColor: colors.card_bg
  },
  wrapper: {
    borderRadius: 5
  },
  address: {
    flex: 1
  },
  actionsContainer: {
    flex: 1,
    flexDirection: 'row'
  },
  actionButtonContainer: {
    flex: 1
  },
  input: {
    // fontSize: 24,
    // height: 60,
    // fontFamily: fonts.regular,
  },
  checkbox: {
    flexDirection: 'row',
    alignItems: 'center'
  },
  //boxes
  b_paddingH: {
    paddingHorizontal: 16,
    backgroundColor: colors.bg,
  },
  b_paddingV: {
    paddingTop: 8,
    paddingBottom: 8,
  },
  b_marginBottom: {
    marginBottom: 16,
  },
  b_marginV_xs: {
    marginTop: 2,
    marginBottom: 3
  },
  b_flex: {
    flex: 1,
    backgroundColor: colors.bg,
  },
  b_row: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    backgroundColor: colors.bg,
  },
  b_bg: {
    backgroundColor: colors.bg,
  },
  b_borderBottom: {
    borderBottomWidth: 1,
    borderBottomColor: colors.bg_text_sec
  },
  b_textInput: {
    marginTop:0, 
    paddingLeft:0, 
    paddingBottom:4,
    borderBottomWidth: 1,
    borderBottomColor: colors.bg_text_sec
  },
  //elements
  el_icon: {
    width: 56,
    height: 56,
  },
  el_iconBorder: {
    height:64, // height = icon height + borderWidth * 2
    width: 64,
    marginRight: 4,
    borderWidth: 4,
    borderRadius:64,
    borderColor: colors.bg,
  },
  el_iconBorder_selected: {
    borderColor: colors.bg_button
  },
  //typography
  t_text: {
    fontFamily: fonts.roboto,
    fontSize: 12,
    color: colors.bg_text,
  },
  t_h1: {
    fontFamily: fonts.robotoBold,
    fontSize: 24,
    color: colors.bg_text,
    marginBottom: 16,
    // textTransform: 'uppercase'
  },
  t_h2: {
    fontFamily: fonts.roboto,
    fontSize: 18,
    color: colors.bg_text,
  },
  t_hintText: {
    fontFamily: fonts.robotoBold,
    fontSize: 13,
    marginBottom: 16,
    color: colors.bg_text,
  },
  t_errorText: {
    color: colors.bg_alert,
  },
  t_btn: {
    fontFamily: fonts.robotoBold,
    fontSize: 24,
    marginBottom: 4,
    color: colors.bg_text,
  },
  t_quote: {
    fontFamily: fonts.robotoLight,
    fontSize: 28,
    color: colors.bg_text,
  },
  t_parityS: {
    fontFamily: fonts.regular,
    fontSize: 18,
    lineHeight: 22,
    color: colors.bg_text,
  },
  t_parityXL: {
    fontFamily: fonts.regular,
    fontSize: 24,
    marginBottom: 8,
    color: colors.bg_text,
  },
  t_bold: {
    fontFamily: fonts.robotoBold,
  },
  t_center: {
    textAlign: 'center',
  },
  t_underline: {
    textDecorationLine: 'underline',
  },
  t_color_sec: {
    color: colors.bg_text_sec,
  }
});
