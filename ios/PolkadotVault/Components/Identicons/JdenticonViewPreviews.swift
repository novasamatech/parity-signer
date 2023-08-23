//
//  JdenticonViewPreviews.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 23/08/2023.
//

import Jdenticon
import SwiftUI

struct JdenticonView_Previews: PreviewProvider {
    static var previews: some View {
        VStack {
            JdenticonView(
                hash: Data([
                    0xE7,
                    0xB0,
                    0xF1,
                    0x79,
                    0xA2,
                    0x1C,
                    0x48,
                    0x6B,
                    0xCF,
                    0x84,
                    0x41,
                    0x04,
                    0xFE,
                    0x6E,
                    0x5B,
                    0x9F,
                    0x3C,
                    0x19,
                    0x9F,
                    0x84
                ]),
                size: 100
            )
            .padding()
            JdenticonView(
                hash: Data([
                    0x9F,
                    0xAF,
                    0xF4,
                    0xF3,
                    0xD6,
                    0xD7,
                    0xD7,
                    0x55,
                    0x77,
                    0xCE,
                    0x81,
                    0x0E,
                    0xC6,
                    0xD6,
                    0xA0,
                    0x6B,
                    0xE4,
                    0x9C,
                    0x3A,
                    0x5A
                ]),
                size: 100
            )
            .padding()
            JdenticonView(
                hash: Data([
                    0xFB,
                    0xB0,
                    0xF1,
                    0x2E,
                    0xD6,
                    0x40,
                    0x1D,
                    0x30,
                    0x46,
                    0x73,
                    0x1B,
                    0x54,
                    0x69,
                    0x08,
                    0x91,
                    0x5C,
                    0x89,
                    0xA1,
                    0x4E,
                    0x8F
                ]),
                size: 100
            )
            .padding()
        }
    }
}
