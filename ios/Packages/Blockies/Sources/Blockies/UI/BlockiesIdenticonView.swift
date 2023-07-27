//
//  BlockiesIdenticonView.swift
//
//
//  Created by Krzysztof Rodak on 19/07/2023.
//

import SwiftUI

/// `BlockiesIdenticonView` is a SwiftUI view that displays a block-style identicon.
///
/// The view uses a `BlockiesIdenticonGenerator` to create an identicon image based on a seed string.
/// The generated image is then displayed in the SwiftUI view.
public struct BlockiesIdenticonView: View {
    let configuration: BlockiesConfiguration
    let seed: String
    let width: CGFloat
    let height: CGFloat

    /// Initializes a new `BlockiesIdenticonView` instance.
    ///
    /// - Parameters:
    ///   - configuration: The configuration to be used for the blockies identicon generation.
    ///   - seed: The seed string to be used for generating the identicon.
    ///   - width: The desired width of the view.
    ///   - height: The desired height of the view.
    public init(configuration: BlockiesConfiguration = .ethAddress, seed: String, width: CGFloat, height: CGFloat) {
        self.configuration = configuration
        self.seed = seed
        self.width = width
        self.height = height
    }

    public var body: some View {
        if let identiconImage = createIdenticonImage() {
            Image(uiImage: identiconImage)
                .resizable()
                .frame(width: width, height: height)
                .aspectRatio(contentMode: .fit)
        } else {
            EmptyView()
        }
    }

    private func createIdenticonImage() -> UIImage? {
        let customScale = max(Int(width / CGFloat(configuration.size)), 1)
        let generator = BlockiesIdenticonGenerator(configuration: configuration)
        return generator.createImage(seed: seed, customScale: customScale)
    }
}
