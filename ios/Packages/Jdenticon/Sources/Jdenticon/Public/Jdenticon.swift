//
//  Jdenticon.swift
//
//
//  Created by Krzysztof Rodak on 23/08/2023.
//

import CoreGraphics
import UIKit

final class Jdenticon {
    private let hashGenerator: HashGenerator

    init(hashGenerator: HashGenerator = HashGenerator()) {
        self.hashGenerator = hashGenerator
    }

    func toSvg(hashOrValue: String, size: Int, padding: CGFloat? = nil) -> String {
        let hash = hashGenerator.keepOrCreateHash(hashOrValue)
        let writer = SvgWriter(size: size)
        let renderer = SvgRenderer(target: writer)
        _ = IconGenerator(
            renderer: renderer,
            hash: hash,
            x: 0,
            y: 0,
            size: CGFloat(size),
            padding: padding,
            config: getCurrentConfig()
        )
        return writer.description
    }

    func getCurrentConfig() -> Config {
        let backColor = "#FFFFFF"

        func lightness(defaultMin: CGFloat, defaultMax: CGFloat) -> (CGFloat) -> CGFloat {
            let range: [CGFloat] = [defaultMin, defaultMax]

            return { value in
                let value2 = range[0] + value * (range[1] - range[0])
                if value2 < 0 {
                    return 0
                } else if value2 > 1 {
                    return 1
                } else {
                    return value2
                }
            }
        }

        return Config(
            saturation: 0.5,
            colorLightness: lightness(defaultMin: 0.4, defaultMax: 0.8),
            grayscaleLightness: lightness(defaultMin: 0.3, defaultMax: 0.9),
            backColor: Color.parse(backColor)
        )
    }
}
