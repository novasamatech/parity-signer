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
                seed: "0xb00adb8980766d75518dfa8efa139fe0d7bb5e4e",
                width: 240,
                height: 240
            )
            BlockiesIdenticonView(
                seed: "0x7204ddf9dc5f672b64ca6692da7b8f13b4d408e7",
                width: 240,
                height: 240
            )
        }
    }
}
