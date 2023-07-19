//
//  IdenticonGenerator.swift
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

public final class IdenticonGenerator {
    private let configuration: BlockiesConfiguration
    private let randomNumberGenerator: PseudoRandomNumberGenerator
    private let patternGenerator: PatternGenerator
    private let imageRenderer: BlockiesImageRenderer

    public init(configuration: BlockiesConfiguration, randomNumberGenerator: PseudoRandomNumberGenerator) {
        self.configuration = configuration
        self.randomNumberGenerator = randomNumberGenerator
        patternGenerator = PatternGenerator(randomNumberGenerator: randomNumberGenerator)
        imageRenderer = BlockiesImageRenderer(randomNumberGenerator: randomNumberGenerator)
    }

    public func createImage(customScale: Int = 1) -> PlatformImage? {
        randomNumberGenerator.loadSeed(from: configuration.seed)
        let imageData = patternGenerator.generatePattern(blockSize: configuration.size)
        return imageRenderer.renderImage(from: imageData, configuration: configuration, scalingFactor: customScale)
    }
}
