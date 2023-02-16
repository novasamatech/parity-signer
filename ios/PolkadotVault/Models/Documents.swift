//
//  Documents.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 11.8.2021.
//

/// Handle decoding of large hardcoded docs

import Foundation

/// Screen model state for documents screen
/// Since this is hardcoded, we heep it in ios logic at least for now
/// Moving it to backend will of course have benetif of reducing code reuse
/// Let's consider it later as now it just works
enum ShownDocument: String, CaseIterable, Identifiable {
    case toc
    case privacyPolicy

    var id: String {
        rawValue
    }

    var label: String {
        switch self {
        case .toc:
            return "Terms of service"
        case .privacyPolicy:
            return "Privacy policy"
        }
    }

    var text: AttributedString {
        switch self {
        case .toc:
            return getTaC()
        case .privacyPolicy:
            return getPP()
        }
    }

    /// Fetch and format docs from assets
    private func getTaC() -> AttributedString {
        if let path = Bundle.main.path(forResource: "terms-and-conditions", ofType: "txt") {
            do {
                let tac = try String(contentsOfFile: path, encoding: .utf8)
                let taCMD = try? AttributedString(
                    markdown: tac,
                    options: AttributedString.MarkdownParsingOptions(interpretedSyntax: .inlineOnlyPreservingWhitespace)
                )
                return taCMD ?? "Terms and conditions text corrupted! Please report bug."
            } catch {
                return "Terms and conditions text corrupted! Please report bug."
            }
        } else {
            return "Terms and conditions not found! Please report bug."
        }
    }

    /// Fetch and format privacy policy from assets
    private func getPP() -> AttributedString {
        if let path = Bundle.main.path(forResource: "privacy-policy", ofType: "txt") {
            do {
                let privacyPolicy = try String(contentsOfFile: path, encoding: .utf8)
                let ppMD = try? AttributedString(
                    markdown: privacyPolicy,
                    options: AttributedString.MarkdownParsingOptions(interpretedSyntax: .inlineOnlyPreservingWhitespace)
                )
                return ppMD ?? "Privacy policy text corrupted! Please report bug."
            } catch {
                return "Privacy policy text corrupted! Please report bug."
            }
        } else {
            return "Privacy policy not found! Please report bug."
        }
    }
}
