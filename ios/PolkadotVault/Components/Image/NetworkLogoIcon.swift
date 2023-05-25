//
//  NetworkLogoIcon.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 12/12/2022.
//

import SwiftUI

struct NetworkLogoIcon: View {
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
        let image = UIImage(named: networkName)
        Group {
            if let image = image {
                Image(uiImage: image)
                    .resizable()
                    .aspectRatio(contentMode: .fit)
                    .frame(width: size, height: size)
                    .clipShape(Circle())
            } else {
                unknownNetworkPlaceholder()
            }
        }
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
    struct NetworkLogoIcon_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                NetworkLogoIcon(networkName: "polkadot")
                NetworkLogoIcon(networkName: "kusama")
                NetworkLogoIcon(networkName: "westend")
                NetworkLogoIcon(networkName: "acala")
                NetworkLogoIcon(networkName: "moonbeam")
                NetworkLogoIcon(networkName: "astar")
                NetworkLogoIcon(networkName: "moonriver")
                NetworkLogoIcon(networkName: "BigNewNetwork")
                NetworkLogoIcon(networkName: "Random")
                NetworkLogoIcon(networkName: "GLMR")
            }
            .previewLayout(.sizeThatFits)
        }
    }
#endif
