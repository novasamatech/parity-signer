//
//  Identicon.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 28.12.2021.
//

import SVGView
import SwiftUI

/// Parse identicon from backend into picture
struct Identicon: View {
    let identicon: [UInt8]
    var rowHeight: CGFloat = 28
    var body: some View {
        Image(uiImage: UIImage(data: Data(identicon)) ?? UIImage())
            .resizable(resizingMode: .stretch)
            .frame(width: rowHeight, height: rowHeight)
    }
}

/// Parse identicon from backend into picture
struct SVGIdenticon: View {
    private let svgAsData: Data
    private let maxSize: CGSize

    init(svgAsData: Data,
         maxSize: CGSize = .init(width: 36, height: 36)) {
        self.svgAsData = svgAsData
        self.maxSize = maxSize
    }

    var body: some View {
        SVGView(data: svgAsData)
            .frame(width: maxSize.width, height: maxSize.height, alignment: .center)
    }
}

// swiftlint: disable all
struct Identicon_Previews: PreviewProvider {
    static var previews: some View {
        VStack(alignment: .center, spacing: 10) {
            Identicon(identicon: PreviewData.exampleIdenticon)
            SVGIdenticon(
                svgAsData: try! Data(
                    contentsOf: Bundle.main.url(
                        forResource: "identicon_example",
                        withExtension: "svg"
                    )!
                ),
                maxSize: .init(width: 200, height: 200)
            )
        }
        .preferredColorScheme(.dark)
        .previewLayout(.sizeThatFits)
    }
}
