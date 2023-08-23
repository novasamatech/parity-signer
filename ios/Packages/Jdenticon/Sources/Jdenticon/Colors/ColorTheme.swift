//
//  ColorTheme.swift
//
//
//  Created by Krzysztof Rodak on 22/08/2023.
//

import Foundation

func colorTheme(hue: CGFloat, config: Config) -> [String] {
    [
        // Dark gray
        Color.hsl(0, 0, config.grayscaleLightness(0)),
        // Mid color
        Color.correctedHsl(hue, config.saturation, config.colorLightness(0.5)),
        // Light gray
        Color.hsl(0, 0, config.grayscaleLightness(1)),
        // Light color
        Color.correctedHsl(hue, config.saturation, config.colorLightness(1)),
        // Dark color
        Color.correctedHsl(hue, config.saturation, config.colorLightness(0))
    ]
}
