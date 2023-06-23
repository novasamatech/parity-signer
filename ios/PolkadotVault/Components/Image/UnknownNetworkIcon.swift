//
//  UnknownNetworkIcon.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 19/06/2023.
//

import SwiftUI

struct UnknownNetworkIcon: View {
    let networkName: String
    let size: CGFloat
    let colorGenerator: UnknownNetworkColorsGenerator

    init(
        networkName: String,
        size: CGFloat = Heights.networkLogoInCell,
        colorGenerator: UnknownNetworkColorsGenerator = ServiceLocator.networkColorsGenerator
    ) {
        self.networkName = networkName
        self.size = size
        self.colorGenerator = colorGenerator
    }

    var body: some View {
        unknownNetworkPlaceholder()
    }

    @ViewBuilder
    func unknownNetworkPlaceholder() -> some View {
        let colorRenderable = colorGenerator.renderable(for: networkName)
        Text(networkName.prefix(1).uppercased())
            .foregroundColor(colorRenderable.text)
            .font(FontFamily.Inter.bold.swiftUIFont(size: fontSize))
            .frame(width: size, height: size)
            .background(colorRenderable.background)
            .clipShape(Circle())
    }

    private var fontSize: CGFloat {
        max(floor(size * 0.7142) - 1, 12)
    }
}

#if DEBUG
    struct UnknownNetworkIcon_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                UnknownNetworkIcon(networkName: "polkadot")
                UnknownNetworkIcon(networkName: "kusama")
                UnknownNetworkIcon(networkName: "westend")
                UnknownNetworkIcon(networkName: "acala")
                UnknownNetworkIcon(networkName: "frequency")
                UnknownNetworkIcon(networkName: "moonbeam")
            }
            VStack {
                UnknownNetworkIcon(networkName: "astar")
                UnknownNetworkIcon(networkName: "moonriver")
                UnknownNetworkIcon(networkName: "BigNewNetwork")
                UnknownNetworkIcon(networkName: "Random")
                UnknownNetworkIcon(networkName: "GLMR")
            }
            .previewLayout(.sizeThatFits)
        }
    }
#endif
