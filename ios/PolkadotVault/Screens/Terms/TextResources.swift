//
//  TextResources.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 14/03/2023.
//

import Foundation

enum TextResources: String, CaseIterable, Identifiable {
    case termsAndConditions = "terms-and-conditions"
    case privacyPolicy = "privacy-policy"

    var id: String {
        rawValue
    }

    var text: AttributedString {
        TextResources.loadResource(rawValue)
    }

    private static func loadResource(_ resource: String) -> AttributedString {
        guard let path = Bundle.main.path(forResource: resource, ofType: "txt"),
              let fileContent = try? String(contentsOfFile: path, encoding: .utf8),
              let formattedContent = try? AttributedString(
                  markdown: fileContent,
                  options: AttributedString.MarkdownParsingOptions(interpretedSyntax: .inlineOnlyPreservingWhitespace)
              ) else { return "" }
        return formattedContent
    }
}
