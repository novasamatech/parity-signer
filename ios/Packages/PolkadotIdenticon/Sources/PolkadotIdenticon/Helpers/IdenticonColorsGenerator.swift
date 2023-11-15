//
//  IdenticonColorsGenerator.swift
//
//
//  Created by Krzysztof Rodak on 31/07/2023.
//

import Blake2
import Foundation

public final class IdenticonColorsGenerator {
    private enum Constants {
        static let byteHashLength = 64
        static let arrayZeroBytesLength = 32
        static let derivedIDRotationFactorMultiplier: UInt8 = 6
        static let derivedIDRotationFactorModulo: UInt8 = 3
        static let hueDegrees = 360
        static let colorArrayLength = 19
        static let lightnessPercentages = [53, 15, 35, 75]
    }

    public init() {}

    /// Returns an array of 19 colors based on the input data.
    ///
    /// The function first hashes the input data and a zero-filled array with the Blake2b hashing algorithm.
    /// It then derives a unique ID by subtracting the hashed zero-filled array from the hashed input data.
    /// The ID is used to calculate a saturation component and derive a palette of 19 colors.
    /// Finally, it selects a color scheme based on the derived ID and rotates the colors in the scheme.
    ///
    /// - Parameter inputData: A byte array to derive the colors from.
    /// - Returns: An array of 19 derived colors.
    func deriveColors(from inputData: [UInt8]) -> [Color] {
        let zeroBytes = Array(repeating: UInt8(0), count: Constants.arrayZeroBytesLength)

        guard let hashedInput = try? Blake2b.hash(size: Constants.byteHashLength, data: inputData),
              let hashedZeroBytes = try? Blake2b.hash(size: Constants.byteHashLength, data: zeroBytes) else {
            return []
        }

        // Create an ID array by subtracting elements of hashedInput and zeroBytes.
        var derivedID: [UInt8] = []
        for (index, byte) in hashedInput.enumerated() {
            let newValue = byte &- hashedZeroBytes[index]
            derivedID.append(newValue)
        }

        let colorPalette = deriveColors(derivedID: derivedID)

        // Choose the color scheme based on the 30th and 31st byte of the derived ID.
        let colorSchemes = ColorScheme.defaultColorSchemes
        let totalFrequency: Int = colorSchemes.reduce(0) { $0 + $1.frequency }
        let selectionFactor = (UInt(derivedID[30]) + UInt(derivedID[31]) * 256) % UInt(totalFrequency)
        let selectedScheme = chooseScheme(schemes: colorSchemes, selectionFactor: selectionFactor)

        // Calculate rotation factor for the color scheme.
        let rotationFactor = (derivedID[28] % Constants.derivedIDRotationFactorMultiplier) * Constants
            .derivedIDRotationFactorModulo

        // Generate the final array of colors using selected color scheme and rotation factor.
        return (0 ..< Constants.colorArrayLength).map { index in
            let colorIndex = index < Constants
                .colorArrayLength - 1 ? (index + Int(rotationFactor)) % (Constants.colorArrayLength - 1) : Constants
                .colorArrayLength - 1
            let paletteIndex = selectedScheme.colorPaletteIndices[colorIndex]
            return colorPalette[paletteIndex]
        }
    }

    private func deriveColors(derivedID: [UInt8]) -> [Color] {
        // Calculate saturation component using 29th byte of the derived ID.
        let sat = UInt8((((UInt(derivedID[29]) * 70) / 256 + 26) % 80) + 30)
        let saturationComponent: Double = Double(sat) / 100.0

        // Generate palette of colors using derived ID and saturation component.
        return derivedID
            .enumerated()
            .map { index, byte in
                let byteColor = byte &+ (UInt8(index % 28) &* 58)
                switch byteColor {
                case 0:
                    return Color(red: 4, green: 4, blue: 4, alpha: 255)
                case 255:
                    return Color.foregroundColor
                default:
                    return derive(fromByte: byteColor, saturationComponent: saturationComponent)
                }
            }
    }

    /// Derives a color from a byte and a saturation component.
    ///
    /// - Parameters:
    ///   - byte: The byte to derive the color from.
    ///   - saturationComponent: The saturation component to use.
    /// - Returns: The derived color.
    private func derive(fromByte byte: UInt8, saturationComponent: Double) -> Color {
        // HSL color hue in degrees
        let hueModulus = 64
        let hue = Int(byte % UInt8(hueModulus)) * Constants.hueDegrees / hueModulus
        // HSL lightness in percents
        let lightnessIndexFactor: UInt8 = 64
        let lightnessIndex = byte / lightnessIndexFactor
        let lightnessPercentage = lightnessIndex < Constants.lightnessPercentages.count ? Constants
            .lightnessPercentages[Int(lightnessIndex)] : 0
        let lightnessComponent: Double = Double(lightnessPercentage) / 100.0
        let (red, green, blue) = hslToRgb(
            hue: Double(hue),
            saturation: saturationComponent,
            lightness: lightnessComponent
        )

        return Color(red: red, green: green, blue: blue, alpha: 255)
    }

    /// Choose a color scheme based on a selection factor.
    ///
    /// - Parameters:
    ///   - schemes: An array of color schemes.
    ///   - selectionFactor: A selection factor to determine which color scheme to use.
    /// - Returns: The selected color scheme.
    private func chooseScheme(schemes: [ColorScheme], selectionFactor: UInt) -> ColorScheme {
        var sum: UInt = 0
        var foundScheme: ColorScheme?
        for scheme in schemes {
            sum += UInt(scheme.frequency)
            if selectionFactor < sum {
                foundScheme = scheme
                break
            }
        }
        return foundScheme!
    }

    /// Converts HSL color space values to RGB color space values.
    ///
    /// - Parameters:
    ///   - hue: The hue value of the HSL color, specified as a degree between 0 and 360.
    ///   - saturation: The saturation value of the HSL color, specified as a double between 0 and 1.
    ///   - lightness: The lightness value of the HSL color, specified as a double between 0 and 1.
    /// - Returns: A tuple representing the RGB color values, each a UInt8 between 0 and 255.
    private func hslToRgb(
        hue: Double,
        saturation: Double,
        lightness: Double
    ) -> (red: UInt8, green: UInt8, blue: UInt8) {
        var redComponent: Double = 0.0
        var greenComponent: Double = 0.0
        var blueComponent: Double = 0.0

        let normalizedHue = hue / 360.0

        if saturation == 0.0 {
            // Achromatic color (gray scale)
            redComponent = lightness
            greenComponent = lightness
            blueComponent = lightness
        } else {
            let qValue = lightness < 0.5 ? lightness * (1 + saturation) : lightness + saturation - lightness *
                saturation
            let pValue = 2 * lightness - qValue

            redComponent = convertHueToRgbComponent(p: pValue, q: qValue, hueShift: normalizedHue + 1 / 3)
            greenComponent = convertHueToRgbComponent(p: pValue, q: qValue, hueShift: normalizedHue)
            blueComponent = convertHueToRgbComponent(p: pValue, q: qValue, hueShift: normalizedHue - 1 / 3)
        }

        return (
            red: UInt8(max(min(floor(redComponent * 256), 255), 0)),
            green: UInt8(max(min(floor(greenComponent * 256), 255), 0)),
            blue: UInt8(max(min(floor(blueComponent * 256), 255), 0))
        )
    }

    /// Calculates a single RGB color component from HSL values.
    ///
    /// - Parameters:
    ///   - p: The first helper value derived from the lightness value of the HSL color.
    ///   - q: The second helper value derived from the lightness and saturation values of the HSL color.
    ///   - hueShift: The hue value of the HSL color, shifted by a certain amount.
    /// - Returns: A double representing the calculated RGB color component.
    private func convertHueToRgbComponent(p: Double, q: Double, hueShift: Double) -> Double {
        var shiftedHue = hueShift

        if shiftedHue < 0 { shiftedHue += 1 }
        if shiftedHue > 1 { shiftedHue -= 1 }

        if shiftedHue < 1 / 6 { return p + (q - p) * 6 * shiftedHue }
        if shiftedHue < 1 / 2 { return q }
        if shiftedHue < 2 / 3 { return p + (q - p) * (2 / 3 - shiftedHue) * 6 }

        return p
    }
}
