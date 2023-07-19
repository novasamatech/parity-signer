//
//  BlockiesIdenticonViewPreviews.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 19/07/2023.
//

import Blockies
import SwiftUI

struct BlockiesIdenticonView_Previews: PreviewProvider {
    static var previews: some View {
        VStack {
            BlockiesIdenticonView(
                configuration: .init(seed: "0xc0ffee254729296a45a3885639AC7E10F9d54979"),
                width: 96,
                height: 96
            )
            BlockiesIdenticonView(
                configuration: .init(seed: "0x999999cf1046e68e36E1aA2E0E07105eDDD1f08E"),
                width: 48,
                height: 48
            )
            BlockiesIdenticonView(
                configuration: .init(seed: "0xD2AAD5732c980AaddDe38CEAD950dBa91Cd2C726"),
                width: 32,
                height: 32
            )
        }
    }
}
