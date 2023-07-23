//
//  PatternGenerator.swift
//
//
//  Created by Krzysztof Rodak on 17/07/2023.
//

import Foundation

/// `PatternGenerator` is responsible for generating a pseudorandom pattern based on a provided configuration.
public class PatternGenerator {
    private enum Constants {
        static let backgroundForegroundScale = 2.3
    }

    private let randomNumberGenerator: PseudoRandomNumberGenerator

    /// Initializes a new `PatternGenerator` instance.
    /// - Parameters:
    ///   - randomNumberGenerator: A pseudo random number generator used for creating the pattern.
    public init(
        randomNumberGenerator: PseudoRandomNumberGenerator = PseudoRandomNumberGenerator()
    ) {
        self.randomNumberGenerator = randomNumberGenerator
    }

    /// Generates a pseudorandom pattern based on the provided configuration during initialization.
    /// The pattern is generated in the form of a Double array.
    /// This function ensures that the foreground and background color have a 43% (1/2.3) probability,
    /// while the spot color has a 13% chance.
    /// - Returns: A pseudorandom pattern as an array of `Double`.
    public func generatePattern(blockSize: Int) -> [Double] {
        let patternWidth = blockSize
        let patternHeight = blockSize

        let dataWidth = Int(ceil(Double(patternWidth) / Double(2)))
        let mirrorWidth = patternWidth - dataWidth

        return (0 ..< patternHeight).flatMap { _ in
            var row: [Double] = (0 ..< dataWidth).map { _ in
                // Generate a random number and scale it by 2.3 before flooring it to control the color probability.
                floor(randomNumberGenerator.nextValue() * 2.3)
            }

            let mirroredSection = Array(row[0 ..< mirrorWidth].reversed())
            row.append(contentsOf: mirroredSection)

            return row
        }
    }
}
