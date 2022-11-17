//
//  Test+Markdown.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 15/11/2022.
//

import SwiftUI

extension Text {
    @ViewBuilder
    static func markdownWithFallback(_ value: String) -> Text {
        Text(
            AttributedString.build(fromHexDocs: value) ??
                AttributedString(Localizable.Error.docsParsing.string)
        )
    }
}
