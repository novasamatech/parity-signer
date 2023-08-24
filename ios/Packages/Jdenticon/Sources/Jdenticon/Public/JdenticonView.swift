//
//  JdenticonView.swift
//
//
//  Created by Krzysztof Rodak on 23/08/2023.
//

import SVGView
import SwiftUI

public struct JdenticonView: View {
    private enum Constants {
        static let svgSize = 1_024
    }

    private let size: CGFloat
    private let svgContent: String

    public init(hash: String, size: CGFloat) {
        svgContent = Jdenticon().toSvg(hashOrValue: hash, size: Constants.svgSize)
        self.size = size
    }

    public var body: some View {
        SVGView(string: svgContent)
            .frame(width: size, height: size)
    }
}
