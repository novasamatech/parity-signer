//
//  SvgWriter.swift
//
//
//  Created by Krzysztof Rodak on 22/08/2023.
//

import Foundation

class SvgWriter {
    let size: Int
    private var _s: String

    init(size: Int) {
        self.size = size
        _s =
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"\(size)\" height=\"\(size)\" viewBox=\"0 0 \(size) \(size)\" preserveAspectRatio=\"xMidYMid meet\">"
    }

    func setBackground(fillColor: String, opacity: CGFloat?) {
        if let opacity {
            _s +=
                "<rect width=\"100%\" height=\"100%\" fill=\"\(fillColor)\" opacity=\"\(String(format: "%.2f", opacity))\"/>"
        }
    }

    func append(color: String, dataString: String) {
        _s += "<path fill=\"\(color)\" d=\"\(dataString)\"/>"
    }

    var description: String {
        _s + "</svg>"
    }
}

extension Float {
    func format(digits: Int) -> String {
        String(format: "%.\(digits)f", self)
    }
}
