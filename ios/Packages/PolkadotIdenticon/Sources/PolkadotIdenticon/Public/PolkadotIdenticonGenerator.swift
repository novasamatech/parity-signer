//
//  PolkadotIdenticonGenerator.swift
//
//
//  Created by Krzysztof Rodak on 02/08/2023.
//

import UIKit

/// `PolkadotIdenticonGenerator` is a class that generates Polkadot Identicons.
/// An Identicon is a unique visual identifier, typically a colored geometric pattern,
/// that corresponds to a unique data input such as a public key. This class provides
/// a method to generate an Identicon based on a public key, which is represented in
/// various formats: as binary data, a hexadecimal string, or a base58 string.
public final class PolkadotIdenticonGenerator {
    private let colorsGenerator: IdenticonColorsGenerator
    private let imageRenderer: IdenticonImageRenderer
    private let publicKeyDecoder: PublicKeyDecoder

    /// Initializes a new instance of `PolkadotIdenticonGenerator`.
    ///
    /// - Parameters:
    ///   - colorsGenerator: An instance of `IdenticonColorsGenerator` used to generate colors
    ///   for the Identicon based on the input data. Defaults to a new instance of `IdenticonColorsGenerator`.
    ///   - imageRenderer: An instance of `IdenticonImageRenderer` used to render the final Identicon image
    ///   from the generated colors. Defaults to a new instance of `IdenticonImageRenderer`.
    ///   - publicKeyDecoder: An instance of `PublicKeyDecoder` used to decode the public key from its various possible
    /// formats.
    ///   Defaults to a new instance of `PublicKeyDecoder`.
    public init(
        colorsGenerator: IdenticonColorsGenerator = IdenticonColorsGenerator(),
        imageRenderer: IdenticonImageRenderer = IdenticonImageRenderer(),
        publicKeyDecoder: PublicKeyDecoder = PublicKeyDecoder()
    ) {
        self.colorsGenerator = colorsGenerator
        self.imageRenderer = imageRenderer
        self.publicKeyDecoder = publicKeyDecoder
    }

    /// Generates a Polkadot Identicon from the public key.
    ///
    /// The identicon is represented as an image, where the image's colors and patterns are uniquely
    /// determined by the public key. The size of the image can be specified.
    ///
    /// - Parameters:
    ///   - publicKey: The public key based on which the Identicon is generated. It can be in one of three formats:
    ///   raw binary data (`Data`), a hexadecimal string, or a base58 string.
    ///   - size: The size of the Identicon image to be generated.
    /// - Returns: A `UIImage` instance representing the generated Identicon, or nil if the image couldn't be generated.
    public func generateIdenticonImage(from publicKey: PublicKey, size: CGFloat) -> UIImage? {
        let inputAsData = publicKeyDecoder.keyAsData(publicKey)
        let colors = colorsGenerator.deriveColors(from: inputAsData)
        return imageRenderer.generateImage(size: size, colors: colors)
    }
}
