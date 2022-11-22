//
//  Test+Markdown.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 15/11/2022.
//

import SwiftUI

extension Text {
    @ViewBuilder
    static func markdownWithFallback(_ value: String, allowsEmptyValue: Bool = true) -> some View {
        Text(
            AttributedString.build(fromHexDocs: value, allowsEmptyValue: allowsEmptyValue) ??
                AttributedString(Localizable.Error.docsParsing.string)
        )
        .multilineTextAlignment(.leading)
    }
}
