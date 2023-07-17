//
//  SeedValueGenerator.swift
//  
//
//  Created by Krzysztof Rodak on 14/07/2023.
//

import Foundation

public struct SeedValueGenerator {

    public init() { }
    /// Creates an array of seeds based on the input string.
    ///
    /// The function computes seeds by cycling through each character of the input string,
    /// and performing bit manipulation operations on the ASCII values of the characters.
    ///
    /// - Parameter seed: The input string to create seeds from.
    /// - Returns: An array of four 32-bit unsigned integers computed from the input string.
    public func createSeeds(from seed: String) -> [UInt32] {
        let seedArraySize = 4
        let bitShiftEquivalentMultiplier = UInt32(32) // Bit shift equivalent as Swift has no overflow left shift
        var seedArray = [UInt32](repeating: 0, count: seedArraySize)

        for (index, character) in seed.enumerated() {
            let asciiValue = UInt32(character.asciiValue ?? 0)
            let seedArrayPosition = index % seedArraySize

            seedArray[seedArrayPosition] = ((seedArray[seedArrayPosition] &* bitShiftEquivalentMultiplier) &- seedArray[seedArrayPosition])
            seedArray[seedArrayPosition] = seedArray[seedArrayPosition] &+ asciiValue
        }

        return seedArray
    }
}
