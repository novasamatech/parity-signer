//
//  Color.swift
//
//
//  Created by Krzysztof Rodak on 22/08/2023.
//

import CoreGraphics
import Foundation

final class Color {
    class func decToHex(_ v: Int) -> String {
        let capped = min(max(v, 0), 255)
        return String(format: "%02X", capped)
    }

    class func hueToRgb(_ m1: CGFloat, _ m2: CGFloat, _ h: CGFloat) -> String {
        let h2 = (h < 0) ? h + 6 : (h > 6) ? h - 6 : h
        let rgbValue = Int(floor(255 * (
            (h2 < 1) ? m1 + (m2 - m1) * h2 :
                (h2 < 3) ? m2 :
                (h2 < 4) ? m1 + (m2 - m1) * (4 - h2) :
                m1
        )))
        return decToHex(rgbValue)
    }

    class func rgb(_ r: Int, _ g: Int, _ b: Int) -> String {
        "#" + decToHex(r) + decToHex(g) + decToHex(b)
    }

    class func parse(_ color: String) -> String {
        let re = try? NSRegularExpression(pattern: "^#[0-9a-f]{3,8}$", options: .caseInsensitive)
        if let matches = re?.matches(in: color, options: [], range: NSRange(location: 0, length: color.utf16.count)),
           let match = matches.first,
           let range = Range(match.range, in: color) {
            let matchedColor = String(color[range])

            if matchedColor.count < 6 {
                let r = matchedColor[matchedColor.index(matchedColor.startIndex, offsetBy: 1)]
                let g = matchedColor[matchedColor.index(matchedColor.startIndex, offsetBy: 2)]
                let b = matchedColor[matchedColor.index(matchedColor.startIndex, offsetBy: 3)]
                let a = (matchedColor.count > 4) ?
                    String(matchedColor[matchedColor.index(matchedColor.startIndex, offsetBy: 4)]) : ""
                return "#\(r)\(r)\(g)\(g)\(b)\(b)\(a)\(a)"
            }
            if matchedColor.count == 7 || matchedColor.count > 8 {
                return matchedColor
            }
        }
        return "#000000"
    }

    class func hsl(_ h: CGFloat, _ s: CGFloat, _ l: CGFloat) -> String {
        if s == 0 {
            let partialHex = decToHex(Int(floor(l * 255)))
            return "#" + partialHex + partialHex + partialHex
        } else {
            let m2 = (l <= 0.5) ? l * (s + 1) : l + s - l * s
            let m1 = l * 2 - m2
            return "#" +
                hueToRgb(m1, m2, h * 6 + 2) +
                hueToRgb(m1, m2, h * 6) +
                hueToRgb(m1, m2, h * 6 - 2)
        }
    }

    class func correctedHsl(_ h: CGFloat, _ s: CGFloat, _ l: CGFloat) -> String {
        let correctors: [CGFloat] = [0.55, 0.5, 0.5, 0.46, 0.6, 0.55, 0.55]
        let corrector = correctors[Int(floor(h * 6 + 0.5))]

        let l2: CGFloat = (l < 0.5) ? l * corrector * 2 : corrector + (l - 0.5) * (1 - corrector) * 2

        return hsl(h, s, l2)
    }
}
