//
//  NetworkIdenticon.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 03/02/2023.
//

import SVGView
import SwiftUI

/// UI container to display identicon
/// Can take `[UInt8]`, `Data` or `SignerImage` as input
struct NetworkIdenticon: View {
    let identicon: Data
    let network: String?
    let background: Color
    let size: CGFloat

    init(identicon: [UInt8], network: String? = nil, background: Color, size: CGFloat = Heights.identiconInCell) {
        self.identicon = Data(identicon)
        self.network = network
        self.background = background
        self.size = size
    }

    init(identicon: SignerImage, network: String? = nil, background: Color, size: CGFloat = Heights.identiconInCell) {
        self.identicon = Data(identicon.svgPayload)
        self.network = network
        self.background = background
        self.size = size
    }

    init(identicon: Data, network: String? = nil, background: Color, size: CGFloat = Heights.identiconInCell) {
        self.identicon = identicon
        self.network = network
        self.background = background
        self.size = size
    }

    var body: some View {
        ZStack(alignment: .bottomTrailing) {
            SVGView(data: identicon)
                .frame(width: size, height: size)
                .clipShape(Circle())
            if let network = network, !network.isEmpty {
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
                    identicon: PreviewData.exampleIdenticon,
                    network: "polkadot",
                    background: Asset.backgroundPrimary.swiftUIColor
                )
                .frame(width: Heights.identiconInCell, height: Heights.identiconInCell)
                NetworkIdenticon(
                    identicon: try! Data(
                        contentsOf: Bundle.main.url(
                            forResource: "identicon_example",
                            withExtension: "svg"
                        )!
                    ),
                    network: "kusama",
                    background: Asset.backgroundPrimary.swiftUIColor
                )
                .frame(width: Heights.identiconInCell, height: Heights.identiconInCell)
            }
            .background(Asset.backgroundPrimary.swiftUIColor)
            VStack(alignment: .center, spacing: 10) {
                NetworkIdenticon(
                    identicon: try! Data(
                        contentsOf: Bundle.main.url(
                            forResource: "identicon_example",
                            withExtension: "svg"
                        )!
                    ),
                    network: "polkadot",
                    background: Asset.backgroundPrimary.swiftUIColor,
                    size: 300
                )
            }
            .background(Asset.backgroundPrimary.swiftUIColor)
            .preferredColorScheme(.dark)
        }
    }
#endif
