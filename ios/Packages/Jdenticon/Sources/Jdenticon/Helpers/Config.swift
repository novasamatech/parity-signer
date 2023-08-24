//
//  Config.swift
//
//
//  Created by Krzysztof Rodak on 22/08/2023.
//

import Foundation

final class Config {
    var saturation: CGFloat
    var colorLightness: (CGFloat) -> CGFloat
    var grayscaleLightness: (CGFloat) -> CGFloat
    var backColor: String

    init(
        saturation: CGFloat,
        colorLightness: @escaping (CGFloat) -> CGFloat,
        grayscaleLightness: @escaping (CGFloat) -> CGFloat,
        backColor: String
    ) {
        self.saturation = saturation
        self.colorLightness = colorLightness
        self.grayscaleLightness = grayscaleLightness
        self.backColor = backColor
    }
}
