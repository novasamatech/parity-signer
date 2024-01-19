//
//  PolkadotIdenticonView.swift
//
//
//  Created by Krzysztof Rodak on 02/08/2023.
//

import SwiftUI

/// `PolkadotIdenticonView` is a SwiftUI view that renders a Polkadot Identicon.
///
/// An Identicon is a visual representation of a public key value, typically used to represent user identities in the
/// context of cryptographic applications.
/// This struct uses `PolkadotIdenticonGenerator` to generate a unique image, based on a provided public key input,
/// which can then be rendered within the SwiftUI view hierarchy.
public struct PolkadotIdenticonView: View {
    private let identiconGenerator: PolkadotIdenticonGenerator = .init()

    /// The public key input based on which the Identicon is generated.
    /// The public key can be in one of three formats: raw binary data (`Data`), a hexadecimal string, or a base58
    /// string.
    let publicKey: PublicKey

    /// The size of the Identicon image to be generated.
    let size: CGFloat

    /// Initializes a new instance of `PolkadotIdenticonView`.
    ///
    /// - Parameters:
    ///   - publicKey: The public key input based on which the Identicon is generated. This could be any unique public
    /// key.
    ///   - size: The size of the Identicon image to be generated.
    public init(publicKey: PublicKey, size: CGFloat) {
        self.publicKey = publicKey
        self.size = size
    }

    /// Defines the content and behavior of this view.
    ///
    /// The body property returns an `Image` view if an Identicon image can be generated from the public key,
    /// or an `EmptyView` if not.
    public var body: some View {
        if let identiconImage = createIdenticonImage() {
            Image(uiImage: identiconImage)
                .resizable()
                .frame(width: size, height: size)
                .aspectRatio(contentMode: .fit)
        } else {
            EmptyView()
        }
    }

    /// Helper function to generate an Identicon image.
    ///
    /// - Returns: A `UIImage` instance representing the generated Identicon, or nil if the image couldn't be generated.
    private func createIdenticonImage() -> UIImage? {
        identiconGenerator.generateIdenticonImage(from: publicKey, size: size)
    }
}
