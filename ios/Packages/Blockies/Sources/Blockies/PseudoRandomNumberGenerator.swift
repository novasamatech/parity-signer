//
//  PseudoRandomNumberGenerator.swift
//
//
//  Created by Krzysztof Rodak on 17/07/2023.
//

import Foundation

/// `PseudoRandomNumberGenerator` generates a sequence of numbers that approximate the properties of random numbers.
public class PseudoRandomNumberGenerator {
    /// Array of seed values used for generating pseudo-random numbers.
    private var seedValues: [UInt32] = []

    /// Responsible for generating seed values from a given string.
    private let seedValueGenerator: SeedValueGenerator

    /// Initializes a new instance of `PseudoRandomNumberGenerator`.
    ///
    /// - Parameter seedValueGenerator: An instance of `SeedValueGenerator` used to generate seed values.
    public init(seedValueGenerator: SeedValueGenerator = SeedValueGenerator()) {
        self.seedValueGenerator = seedValueGenerator
    }

    /// Loads the seed values from a given seed string.
    ///
    /// - Parameter seed: A string value used to generate the seed values.
    public func loadSeed(from seed: String) {
        seedValues = seedValueGenerator.createSeeds(from: seed)
    }

    /// Generates the next pseudo-random number in the sequence.
    ///
    /// - Returns: A pseudo-random `Double` value.
    public func nextValue() -> Double {
        let shiftedSeed = seedValues[0] ^ (seedValues[0] << 11)

        seedValues[0] = seedValues[1]
        seedValues[1] = seedValues[2]
        seedValues[2] = seedValues[3]
        let currentSeedAsInt = Int32(bitPattern: seedValues[3])
        let shiftedSeedAsInt = Int32(bitPattern: shiftedSeed)
        seedValues[3] =
            UInt32(bitPattern: currentSeedAsInt ^ (currentSeedAsInt >> 19) ^ shiftedSeedAsInt ^ (shiftedSeedAsInt >> 8))

        let divisor = Int32.max

        return Double(UInt32(seedValues[3]) >> UInt32(0)) / Double(divisor)
    }
}
