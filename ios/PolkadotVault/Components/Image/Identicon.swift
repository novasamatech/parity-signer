//
//  IdenticonView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 08/12/2022.
//

import Blockies
import PolkadotIdenticon
import SwiftUI

/// UI container to display identicon
/// Can take `[UInt8]`, `Data` or `SignerImage` as input
struct IdenticonView: View {
    let identicon: Identicon
    var rowHeight: CGFloat

    init(identicon: Identicon, rowHeight: CGFloat = Heights.identiconInCell) {
        self.identicon = identicon
        self.rowHeight = rowHeight
    }

    var body: some View {
        switch identicon {
        case let .dots(identity):
            PolkadotIdenticonView(
                publicKey: .data(Data(identity)),
                size: rowHeight
            )
            .frame(width: rowHeight, height: rowHeight)
            .clipShape(Circle())
        case let .blockies(identity):
            BlockiesIdenticonView(
                seed: identity,
                width: rowHeight,
                height: rowHeight
            )
            .frame(width: rowHeight, height: rowHeight)
            .clipShape(Circle())
        case let .jdenticon(identity):
            Spacer()
                .frame(width: rowHeight, height: rowHeight)
        }
    }
}

#if DEBUG
    // swiftlint: disable all
    struct Identicon_Previews: PreviewProvider {
        static var previews: some View {
            VStack(alignment: .center, spacing: 10) {
                IdenticonView(
                    identicon: .stubIdenticon
                )
            }
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
            VStack(alignment: .center, spacing: 10) {
                IdenticonView(
                    identicon: .stubBlockiesIdenticon
                )
            }
            .frame(maxWidth: 150)
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
        }
    }
#endif
