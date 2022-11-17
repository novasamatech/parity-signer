//
//  AttributedString+Markdown.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 15/11/2022.
//

import Foundation

/// Decode markdown object from hex-encoded string passed in JSON
/// EDIT: I needed to correct it to return nil if string would end up empty, as without it,
/// no fallback would work as invalid markdown would result in valid but empty `AttributedString`
extension AttributedString {
    static func build(fromHexDocs string: String) -> AttributedString? {
        if let result = try? self.init(
            markdown: Data(fromHexEncodedString: string) ?? Data(),
            options: AttributedString.MarkdownParsingOptions(
                interpretedSyntax: .inlineOnlyPreservingWhitespace,
                failurePolicy: .returnPartiallyParsedIfPossible
            )
        ), !result.characters.isEmpty {
            return result
        } else {
            return nil
        }
    }
}
