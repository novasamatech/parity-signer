//
//  Documents.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 11.8.2021.
//

/**
 * Handle decoding of large hardcoded docs
 */

import Foundation

/**
 * Screen model state for documents screen
 * Since this is hardcoded, we heep it in ios logic at least for now
 * Moving it to backend will of course have benetif of reducing code reuse
 * Let's consider it later as now it just works
 */
enum ShownDocument: String, CaseIterable, Identifiable {
    case toc
    case pp
    
    var id: String {
        self.rawValue
    }
    
    var label: String {
        switch (self) {
        case .toc: return "Terms of service"
        case .pp: return "Privacy policy"
        }
    }
}

/**
 * Fetch docs from assets
 */
extension SignerDataModel {
    func getTaC() -> AttributedString {
        if let path = Bundle.main.path(forResource: "terms-and-conditions", ofType: "txt") {
            do {
                let tac = try String(contentsOfFile: path, encoding: .utf8)
                let taCMD = try! AttributedString(markdown: tac, options: AttributedString.MarkdownParsingOptions(interpretedSyntax: .inlineOnlyPreservingWhitespace))
                return taCMD
            } catch {
                print("TaC file damaged")
                return "Terms and conditions text corrupted! Please report bug."
            }
        } else {
            print("TaC file not found!")
            return "Terms and conditions not found! Please report bug."
        }
    }
    
    func getPP() -> AttributedString {
        if let path = Bundle.main.path(forResource: "privacy-policy", ofType: "txt") {
            do {
                let pp = try String(contentsOfFile: path, encoding: .utf8)
                let ppMD = try! AttributedString(markdown: pp, options: AttributedString.MarkdownParsingOptions(interpretedSyntax: .inlineOnlyPreservingWhitespace))
                return ppMD
            } catch {
                print("PP file damaged")
                return "Privacy policy text corrupted! Please report bug."
            }
        } else {
            print("PP file not found!")
            return "Privacy policy not found! Please report bug."
        }
    }
}
