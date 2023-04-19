//
//  Identicon.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 08/12/2022.
//

import SVGView
import SwiftUI

/// UI container to display identicon
/// Can take `[UInt8]`, `Data` or `SignerImage` as input
struct Identicon: View {
    let identicon: SignerImage
    var rowHeight: CGFloat?

    init(identicon: SignerImage, rowHeight: CGFloat? = Heights.identiconInCell) {
        self.identicon = identicon
        self.rowHeight = rowHeight
    }

    var body: some View {
        switch identicon {
        case let .svg(image: svgImage):
            if let rowHeight = rowHeight {
                SVGView(data: Data(svgImage))
                    .frame(width: rowHeight, height: rowHeight)
                    .clipShape(Circle())
            } else {
                SVGView(data: Data(svgImage))
                    .clipShape(Circle())
            }
        case let .png(image: pngImage):
            if let rowHeight = rowHeight {
                Image(uiImage: UIImage(data: Data(pngImage)) ?? UIImage())
                    .resizable(resizingMode: .stretch)
                    .frame(width: rowHeight, height: rowHeight)
                    .clipShape(Circle())
            } else {
                Image(uiImage: UIImage(data: Data(pngImage)) ?? UIImage())
                    .resizable(resizingMode: .stretch)
                    .clipShape(Circle())
            }
        }
    }
}

#if DEBUG
    // swiftlint: disable all
    struct Identicon_Previews: PreviewProvider {
        static var previews: some View {
            VStack(alignment: .center, spacing: 10) {
                Identicon(
                    identicon: .svg(image: PreviewData.exampleIdenticon)
                )
                Identicon(
                    identicon: .svg(
                        image: Array(
                            try! Data(
                                contentsOf: Bundle.main.url(
                                    forResource: "identicon_example",
                                    withExtension: "svg"
                                )!
                            )
                        )
                    ),
                    rowHeight: 200
                )
            }
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
            VStack(alignment: .center, spacing: 10) {
                Identicon(
                    identicon: .svg(
                        image: Array(
                            try! Data(
                                contentsOf: Bundle.main.url(
                                    forResource: "identicon_example",
                                    withExtension: "svg"
                                )!
                            )
                        )
                    ),
                    rowHeight: nil
                )
            }
            .frame(maxWidth: 150)
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
        }
    }
#endif
