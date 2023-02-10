//
//  Localizable+Formatted.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 12/01/2023.
//

import Foundation
import UIKit

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

    static func signingInvalidNetworkVersionStepOne() -> AttributedString {
        let mainText = Localizable.TransactionSign.Error.InvalidNetworkVersion.step1
            .string
        let highlightedFont = Localizable.TransactionSign.Error.InvalidNetworkVersion.Step1.Highlight
            .font.string
        let highlightedPartOne = Localizable.TransactionSign.Error.InvalidNetworkVersion.Step1.Highlight
            .first.string
        let highlightedPartTwo = Localizable.TransactionSign.Error.InvalidNetworkVersion.Step1.Highlight
            .second.string
        let highlightedPartThree = Localizable.TransactionSign.Error.InvalidNetworkVersion.Step1.Highlight
            .second.string
        let attributedString = NSMutableAttributedString(string: mainText)
        let rangeFont = (mainText as NSString).range(of: highlightedFont)
        let rangePartOne = (mainText as NSString).range(of: highlightedPartOne)
        let rangePartTwo = (mainText as NSString).range(of: highlightedPartTwo)
        let rangePartThree = (mainText as NSString).range(of: highlightedPartThree)
        attributedString.setAttributes(
            [.foregroundColor: Asset.accentPink300.color],
            range: rangePartOne
        )
        attributedString.setAttributes(
            [.foregroundColor: Asset.accentPink300.color],
            range: rangePartTwo
        )
        attributedString.setAttributes(
            [.foregroundColor: Asset.textAndIconsTertiary.color],
            range: rangePartThree
        )
        attributedString.setAttributes(
            [.font: FontFamily.Inter.regular.font(size: 14)],
            range: rangeFont
        )

        return AttributedString(attributedString)
    }

    static func signingUnknownNetworkStepOne() -> AttributedString {
        let mainText = Localizable.TransactionSign.Error.UnknownNetwork.step1
            .string
        let highlightedFont = Localizable.TransactionSign.Error.UnknownNetwork.Step1.Highlight
            .font.string
        let highlightedPartOne = Localizable.TransactionSign.Error.UnknownNetwork.Step1.Highlight
            .first.string
        let highlightedPartTwo = Localizable.TransactionSign.Error.UnknownNetwork.Step1.Highlight
            .second.string
        let highlightedPartThree = Localizable.TransactionSign.Error.UnknownNetwork.Step1.Highlight
            .second.string
        let attributedString = NSMutableAttributedString(string: mainText)
        let rangeFont = (mainText as NSString).range(of: highlightedFont)
        let rangePartOne = (mainText as NSString).range(of: highlightedPartOne)
        let rangePartTwo = (mainText as NSString).range(of: highlightedPartTwo)
        let rangePartThree = (mainText as NSString).range(of: highlightedPartThree)
        attributedString.setAttributes(
            [.foregroundColor: Asset.accentPink300.color],
            range: rangePartOne
        )
        attributedString.setAttributes(
            [.foregroundColor: Asset.accentPink300.color],
            range: rangePartTwo
        )
        attributedString.setAttributes(
            [.foregroundColor: Asset.textAndIconsTertiary.color],
            range: rangePartThree
        )
        attributedString.setAttributes(
            [.font: FontFamily.Inter.regular.font(size: 14)],
            range: rangeFont
        )

        return AttributedString(attributedString)
    }

    static func signingMetadataUnknownNetwork() -> AttributedString {
        let mainText = Localizable.TransactionSign.Error.InvalidNetworkVersion.step1
            .string
        let highlightedFont = Localizable.TransactionSign.Error.MetadataUnknownNetwork.Step1.Highlight
            .font.string
        let highlightedPartOne = Localizable.TransactionSign.Error.MetadataUnknownNetwork.Step1.Highlight
            .first.string
        let highlightedPartTwo = Localizable.TransactionSign.Error.MetadataUnknownNetwork.Step1.Highlight
            .second.string
        let highlightedPartThree = Localizable.TransactionSign.Error.MetadataUnknownNetwork.Step1.Highlight
            .second.string
        let attributedString = NSMutableAttributedString(string: mainText)
        let rangeFont = (mainText as NSString).range(of: highlightedFont)
        let rangePartOne = (mainText as NSString).range(of: highlightedPartOne)
        let rangePartTwo = (mainText as NSString).range(of: highlightedPartTwo)
        let rangePartThree = (mainText as NSString).range(of: highlightedPartThree)
        attributedString.setAttributes(
            [.foregroundColor: Asset.accentPink300.color],
            range: rangePartOne
        )
        attributedString.setAttributes(
            [.foregroundColor: Asset.accentPink300.color],
            range: rangePartTwo
        )
        attributedString.setAttributes(
            [.foregroundColor: Asset.textAndIconsTertiary.color],
            range: rangePartThree
        )
        attributedString.setAttributes(
            [.font: FontFamily.Inter.regular.font(size: 14)],
            range: rangeFont
        )
        return AttributedString(attributedString)
    }
}
