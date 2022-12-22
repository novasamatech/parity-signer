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
    static func build(fromDocs string: String, allowsEmptyValue: Bool = false) -> AttributedString? {
        if let result = try? self.init(
            markdown: string,
            options: AttributedString.MarkdownParsingOptions(
                interpretedSyntax: .inlineOnlyPreservingWhitespace,
                failurePolicy: .returnPartiallyParsedIfPossible
            )
        ) {
            if !allowsEmptyValue, result.characters.isEmpty {
                return nil
            }
            return result
        } else {
            return nil
        }
    }
}
