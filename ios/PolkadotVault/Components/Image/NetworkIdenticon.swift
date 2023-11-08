//
//  NetworkIdenticon.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 03/02/2023.
//

import SwiftUI

/// UI container to display identicon
/// Can take `[UInt8]`, `Data` or `SignerImage` as input
struct NetworkIdenticon: View {
    let identicon: Identicon
    let network: String?
    let background: Color
    let size: CGFloat

    init(identicon: Identicon, network: String? = nil, background: Color, size: CGFloat = Heights.identiconInCell) {
        self.identicon = identicon
        self.network = network
        self.background = background
        self.size = size
    }

    var body: some View {
        ZStack(alignment: .bottomTrailing) {
            IdenticonView(identicon: identicon, rowHeight: size)
            if let network, !network.isEmpty {
                NetworkLogoIcon(
                    networkName: network,
                    size: size / 2
                )
                .padding(size / 36)
                .overlay(
                    Circle()
                        .stroke(background, lineWidth: size / 18)
                )
            }
        }
    }
}

#if DEBUG
    // swiftlint: disable all
    struct NetworkIdenticon_Previews: PreviewProvider {
        static var previews: some View {
            VStack(alignment: .center, spacing: 10) {
                NetworkIdenticon(
                    identicon: .stubIdenticon,
                    network: "polkadot",
                    background: .backgroundPrimary
                )
                .frame(width: Heights.identiconInCell, height: Heights.identiconInCell)
            }
            .background(.backgroundPrimary)
            VStack(alignment: .center, spacing: 10) {
                NetworkIdenticon(
                    identicon: .stubIdenticon,
                    network: "polkadot",
                    background: .backgroundPrimary
                )
                .frame(width: Heights.identiconInCell, height: Heights.identiconInCell)
            }
            .background(.backgroundPrimary)
            .preferredColorScheme(.dark)
        }
    }
#endif
