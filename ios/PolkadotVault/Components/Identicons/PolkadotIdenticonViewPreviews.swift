//
//  PolkadotIdenticonViewPreviews.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 02/08/2023.
//

import PolkadotIdenticon
import SwiftUI

struct PolkadotIdenticonView_Previews: PreviewProvider {
    static var previews: some View {
        VStack {
            PolkadotIdenticonView(
                publicKey: .hex("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"),
                size: 100
            )
            PolkadotIdenticonView(
                publicKey: .base58("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"),
                size: 100
            )
            PolkadotIdenticonView(
                publicKey: .data(Data([
                    212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88,
                    133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125
                ])),
                size: 100
            )
        }
    }
}
