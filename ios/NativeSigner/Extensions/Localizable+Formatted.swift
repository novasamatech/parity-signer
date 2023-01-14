//
//  Localizable+Formatted.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 12/01/2023.
//

import Foundation

extension Localizable {
    static func createDerivedKeyModalPathInfo() -> AttributedString {
        let mainText = Localizable.CreateDerivedKey.Modal.Path.info.string
        let highlightedText = Localizable.CreateDerivedKey.Modal.Path.Info.highlight.string

        let attributedString = NSMutableAttributedString(string: mainText)
        attributedString.addAttribute(
            .foregroundColor,
            value: Asset.textAndIconsTertiary.color,
            range: NSRange(location: 0, length: mainText.count)
        )

        let range = (mainText as NSString).range(of: highlightedText)
        attributedString.setAttributes([.foregroundColor: Asset.accentPink300.color], range: range)
        return AttributedString(attributedString)
    }
}
