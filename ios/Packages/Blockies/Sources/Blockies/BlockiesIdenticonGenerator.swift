//
//  BlockiesIdenticonGenerator.swift
//
//
//  Created by Krzysztof Rodak on 17/07/2023.
//

import Foundation

#if os(iOS) || os(tvOS) || os(watchOS)
    import UIKit
#elseif os(OSX)
    import AppKit
#endif

/// `BlockiesIdenticonGenerator` is a class that generates block-style identicons.
///
/// The class allows you to create images by providing a seed string. You can also customize the image generation by
/// changing the size, scale, colors, and patterns used.
///
/// The class uses a pseudo-random number generator, a pattern generator, an image renderer, and a color generator
/// to create the identicon image.
public final class BlockiesIdenticonGenerator {
    private let configuration: BlockiesConfiguration
    private let randomNumberGenerator: PseudoRandomNumberGenerator
    private let patternGenerator: PatternGenerator
    private let imageRenderer: BlockiesImageRenderer
    private let colorGenerator: PseudoRandomColorGenerator
    
    /// Initializes a new `BlockiesIdenticonGenerator` instance.
    ///
    /// - Parameters:
    ///   - configuration: The configuration to be used for the blockies identicon generation.
    ///   - randomNumberGenerator: A pseudo random number generator used for creating the pattern.
    public init(
        configuration: BlockiesConfiguration,
        randomNumberGenerator: PseudoRandomNumberGenerator = PseudoRandomNumberGenerator()
    ) {
        self.configuration = configuration
        self.randomNumberGenerator = randomNumberGenerator
        patternGenerator = PatternGenerator(randomNumberGenerator: randomNumberGenerator)
        imageRenderer = BlockiesImageRenderer(randomNumberGenerator: randomNumberGenerator)
        colorGenerator = PseudoRandomColorGenerator(randomNumberGenerator: randomNumberGenerator)
    }
    
    /// Creates a block-style identicon image from a seed string.
    ///
    /// - Parameters:
    ///   - seed: The seed string to be used for generating the identicon.
    ///   - customScale: An optional scaling factor for the size of the blocks.
    ///
    /// - Returns: The generated block-style identicon image.
    
    public func createImage(seed: String, customScale: Int = 1) -> PlatformImage? {
        randomNumberGenerator.loadSeed(from: seed)
        let colors = colorGenerator.generateColors()
        let imageData = patternGenerator.generatePattern(blockSize: configuration.size)
        return imageRenderer.renderImage(
            from: imageData,
            configuration: configuration,
            colors: colors,
            scalingFactor: customScale
        )
    }
}
