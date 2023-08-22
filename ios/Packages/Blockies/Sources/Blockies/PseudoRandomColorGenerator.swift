//
//  PseudoRandomColorGenerator.swift
//
//
//  Created by Krzysztof Rodak on 27/07/2023.
//

import Foundation

/// A structure that holds the color configuration for Blockies identicon.
///
/// This struct encapsulates the primary color, background color, and spot color for the identicon.
/// Each color is represented by a `PlatformColor` object.
public struct ColorsConfiguration {
    /// The primary color for the identicon.
    /// This color is used to paint the main pattern of the identicon.
    public let color: PlatformColor

    /// The background color for the identicon.
    /// This color is used as the background of the identicon.
    public let bgcolor: PlatformColor

    /// The spot color for the identicon.
    public let spotcolor: PlatformColor
}

/// A class that generates pseudo-random colors for a Blockies identicon.
///
/// This class uses a pseudo-random number generator to create the colors needed for an identicon.
/// It produces a deterministic sequence of colors based on the seed used for the number generator.
public final class PseudoRandomColorGenerator {
    /// The pseudo-random number generator used to create the color sequence.
    private let randomNumberGenerator: PseudoRandomNumberGenerator

    /// Initializes a new instance of a pseudo-random color generator.
    ///
    /// This initializer takes a pseudo-random number generator which is used to create the sequence of colors.
    ///
    /// - Parameters:
    ///   - randomNumberGenerator: A pseudo-random number generator used to create the color sequence.
    public init(randomNumberGenerator: PseudoRandomNumberGenerator) {
        self.randomNumberGenerator = randomNumberGenerator
    }

    /// Generates a color configuration for an identicon.
    ///
    /// This method creates a `ColorsConfiguration` object that contains a primary color, a background color,
    /// and a spot color for the identicon. Each color is created using the pseudo-random number generator.
    ///
    /// - Returns: A `ColorsConfiguration` object that represents the color configuration for an identicon.
    public func generateColors() -> ColorsConfiguration {
        .init(
            color: createColor(),
            bgcolor: createColor(),
            spotcolor: createColor()
        )
    }

    /// Creates a pseudo random color.
    ///
    /// This method generates a color using the pseudo-random number generator. It calculates the hue, saturation,
    /// and lightness of the color to ensure a wide range of colors.
    ///
    /// - Returns: A `PlatformColor` object that represents a pseudo randomly generated color.
    private func createColor() -> PlatformColor {
        let h = randomNumberGenerator.nextValue() * 360
        let s = ((randomNumberGenerator.nextValue() * 60) + 40) / 100
        let l = (
            randomNumberGenerator.nextValue() + randomNumberGenerator.nextValue() +
                randomNumberGenerator.nextValue() + randomNumberGenerator.nextValue()
        ) * 25 / 100

        return PlatformColor(hue: h, saturation: s, lightness: l) ?? PlatformColor.black
    }
}
