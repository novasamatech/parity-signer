// Copyright 2015-2017 Parity Technologies (UK) Ltd.
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

'use strict';

import React from 'react';
import PropTypes from 'prop-types';
import {
  Alert,
  ScrollView,
  View,
  Text,
  TouchableOpacity,
  Share,
  StyleSheet,
  Linking
} from 'react-native';
import Icon from 'react-native-vector-icons/MaterialCommunityIcons';
import Button from '../components/Button';
import TouchableItem from '../components/TouchableItem';
import colors from '../colors';

export default class TermsAndConditions extends React.PureComponent {
  static navigationOptions = {
    title: 'Terms and conditions',
    headerBackTitle: 'Back'
  };

  render() {
    const { navigation } = this.props;
    const isWelcome = navigation.getParam('isWelcome');
    return (
      <View style={styles.body}>
        <ScrollView contentContainerStyle={{}}>
          <Text style={styles.title}>TERMS & CONDITIONS - Parity Signer</Text>
          <Text style={styles.text}>LEGAL WARNING SHORT VERSION</Text>
          <Text style={styles.text}>
            Disclaimer of Liability and Warranties
          </Text>
          <Text style={styles.text}>
            The user expressly acknowledges and agrees that Parity Technologies
            Limited makes the Parity Signer application available to the user at
            the user's sole risk.
          </Text>
          <Text style={styles.text}>
            The user represents that the user has an adequate understanding of
            the risks, usage and intricacies of public and private key
            cryptography, cryptographic tokens, mobile wallet technology,
            blockchain-based open source software, the Ethereum platform and
            Ether (ETH).
          </Text>
          <Text style={styles.text}>
            The user acknowledges and agrees that, to the fullest extent
            permitted by any applicable law, the disclaimers of liability
            contained herein apply to any and all damages or injury whatsoever
            caused by or related to risks of, use of, or inability to use, the
            Parity Signer application under any cause or action whatsoever of
            any kind in any jurisdiction, including, without limitation, actions
            for breach of warranty, breach of contract or tort (including
            negligence) and that Parity Technologies Limited shall not be liable
            for any indirect, incidental, special, exemplary or consequential
            damages, including for loss of profits, goodwill or data.
          </Text>
          <Text style={styles.text}>
            Some jurisdictions do not allow the exclusion of certain warranties
            or the limitation or exclusion of liability for certain types of
            damages. Therefore, some of the above limitations in this section
            may not apply to a user. In particular, nothing in these terms shall
            affect the statutory rights of any user or limit or exclude
            liability for death or physical injury arising from the negligence
            or wilful misconduct of Parity Technologies Limited or for fraud or
            fraudulent misrepresentation.
          </Text>
          <Text style={styles.text}>
            All rights reserved by Parity Technologies Limited. Licensed to the
            public under the GNU General Public License Version 3:
            https://opensource.org/licenses/GPL-3.0
          </Text>
          <Text style={styles.text}>LEGAL WARNING LONG VERSION</Text>
          <Text style={styles.text}>
            The following Terms and Conditions ("Terms") govern the use of
            Parity Technologies Limited's open source software application
            Parity Signer and any published updates to that open source software
            application ("Parity Signer"). Prior to any use of Parity Signer or
            any of Parity Technologies Limited's products ("Parity Technologies'
            Products"), you ("User" or "you") confirm on your own behalf and on
            behalf of anyone who uses Parity Signer on your behalf that you (and
            they) understand, expressly agree to and will comply with all of the
            Terms. All capitalized words and expressions in these Terms will
            have the effect and meaning given to them in the Terms. The group of
            developers and other personnel that is now, or will be, employed by,
            or contracted with, or affiliated with, Parity Technologies Limited
            ("Parity Technologies" or "we") is termed the "Parity Technologies
            Team".
          </Text>
          <Text style={styles.text}>Acknowledgement of Risks</Text>
          <Text style={styles.text}>
            The User acknowledges the following serious risks to any users of
            Parity Signer and expressly agrees not to hold liable Parity
            Technologies or the Parity Technologies Team should any of these
            risks occur:
          </Text>
          <Text style={styles.text}>
            Risk of Security Weaknesses in the Parity Core Infrastructure
            Software
          </Text>
          <Text style={styles.text}>
            Parity Signer uses open-source libraries and components developed by
            third parties. While Parity Technologies Limited generally aims to
            use only widely adopted open-source technology and develop it in
            line with industry standards, such open-source technology may
            contain bugs and errors and may not function correctly in all
            circumstances. As a result, there is a risk that Parity Technologies
            or the Parity Technologies Team may have introduced unintentional
            weaknesses or bugs into the core infrastructural elements of Parity
            Signer causing the loss of private keys stored in one or more user
            accounts in the application, Ethereum tokens ("ETH") or sums of
            other valued tokens.
          </Text>
          <Text style={styles.text}>
            Risk of Weaknesses or Exploitable Breakthroughs in the Field of
            Cryptography
          </Text>
          <Text style={styles.text}>
            Cryptography is an art, not a science, and the state of the art can
            advance over time. Advances in code cracking, or technical advances
            such as the development of quantum computers, could present risks to
            cryptocurrencies and Parity Signer, which could result in the theft
            or loss of private keys stored in one or more user accounts in the
            Parity Signer application, ETH or sums of other valued tokens. To
            the extent possible, Parity Technologies intends to update the
            software underlying Parity Signer to account for any advances in
            cryptography and to incorporate additional security measures, but it
            cannot predict the future of cryptography or guarantee that any
            security updates will be made, timely or successful.
          </Text>
          <Text style={styles.text}>Risk of Mining Attacks</Text>
          <Text style={styles.text}>
            The blockchains for which Parity Signer creates accounts are
            susceptible to mining attacks, including but not limited to
            double-spend attacks, majority mining power attacks,
            "selfish-mining" attacks, and race condition attacks. Any successful
            attacks present a risk to the ecosystems of those blockchains, for
            example in respect of the Ethereum ecosystem a successful attack
            would present risks to the expected proper execution and sequencing
            of ETH transactions, and the expected proper execution and
            sequencing of contract computations. Despite the efforts of Parity
            Technologies and the Parity Technologies Team, known or novel mining
            attacks may be successful.
          </Text>
          <Text style={styles.text}>
            Risk of Rapid Adoption and Insufficiency of Computational
            Application Processing Power on the Ethereum Network
          </Text>
          <Text style={styles.text}>
            If Ethereum is rapidly adopted, the demand for transaction
            processing and distributed application computations could rise
            dramatically and at a pace that exceeds the rate with which ETH
            miners can bring online additional mining power. Under such a
            scenario, the entire Ethereum ecosystem could become destabilized,
            due to the increased cost of running distributed applications. In
            turn, this could dampen interest in the Ethereum ecosystem and ETH.
            Insufficiency of computational resources and an associated rise in
            the price of ETH could result in businesses being unable to acquire
            scarce computational resources to run their distributed
            applications. This would represent revenue losses to businesses or
            worst case, cause businesses to cease operations because such
            operations have become uneconomical due to distortions in the
            crypto-economy.
          </Text>
          <Text style={styles.text}>Use of Parity Signer by you</Text>
        </ScrollView>

        <TouchableItem
          style={{
            flexDirection: 'row',
            alignItems: 'center',
          }}
          onPress={() => {}}
        >
          <Icon
            name="checkbox-blank-outline"
            style={[styles.text, { fontSize: 30 }]}
          />
          <Text style={[styles.text, { fontSize: 16 }]}>
            {'  I agree to the terms and conditions'}
          </Text>
        </TouchableItem>

        <TouchableItem
          style={{
            flexDirection: 'row',
            alignItems: 'center',
          }}
          onPress={() => {}}
        >
          <Icon
            name="checkbox-blank-outline"
            style={[styles.text, { fontSize: 30 }]}
          />
          <Text style={[styles.text, { fontSize: 16 }]}>
            {'  I agree to the privacy policy'}
          </Text>
        </TouchableItem>
        <Button
            buttonStyles={{ marginTop: 10, height: 60 }}
            title="Next"
            onPress={() => {
              this.props.navigation.navigate('AccountAdd');
            }}
          />
      </View>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    flex: 1,
    flexDirection: 'column',
    overflow: 'hidden',
    backgroundColor: colors.bg,
    padding: 20
  },
  top: {
    flex: 1
  },
  bottom: {
    flexBasis: 50,
    paddingBottom: 15
  },
  titleTop: {
    color: colors.bg_text_sec,
    fontSize: 24,
    fontFamily: 'Manifold CF',
    fontWeight: 'bold',
    paddingBottom: 20,
    textAlign: 'center'
  },
  title: {
    fontFamily: 'Manifold CF',
    color: colors.bg_text_sec,
    fontSize: 18,
    fontWeight: 'bold',
    paddingBottom: 20
  },
  text: {
    marginTop: 10,
    fontFamily: 'Roboto',
    fontSize: 14,
    color: colors.card_bg
  }
});
