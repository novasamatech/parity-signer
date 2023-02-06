//
//  NetworkLogoIcon.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 12/12/2022.
//

import SwiftUI

struct NetworkLogoIcon: View {
    let networkName: String
    let size: CGFloat

    init(networkName: String, size: CGFloat = Heights.networkLogoInCell) {
        self.networkName = networkName
        self.size = size
    }

    var body: some View {
        let image = UIImage(named: networkName.lowercased())
        Group {
            if let image = image {
                Image(uiImage: image)
                    .resizable()
                    .aspectRatio(contentMode: .fit)
                    .frame(width: size, height: size)
                    .clipShape(Circle())
            } else {
                Text(networkName.prefix(1).uppercased())
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .font(PrimaryFont.titleM.font)
                    .frame(width: size, height: size)
                    .background(Color.random())
                    .clipShape(Circle())
            }
        }
    }
}

private extension Color {
    static func random() -> Color {
        Color(
            red: .random(in: 0 ... 1),
            green: .random(in: 0 ... 1),
            blue: .random(in: 0 ... 1)
        )
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
