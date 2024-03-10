//
//  Localizable+Formatted.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 12/01/2023.
//

import Foundation
import SwiftUI
import UIKit

extension Localizable {
    static func createDerivedKeyModalPathInfo() -> AttributedString {
        let mainText = Localizable.CreateDerivedKey.Modal.Path.info.string
        let highlightedText = Localizable.CreateDerivedKey.Modal.Path.Info.highlight.string

        let attributedString = NSMutableAttributedString(string: mainText)
        attributedString.addAttribute(
            .foregroundColor,
            value: Color(.textAndIconsTertiary),
            range: NSRange(location: 0, length: mainText.count)
        )

        let range = (mainText as NSString).range(of: highlightedText)
        attributedString.setAttributes([.foregroundColor: UIColor(.accentPink300)], range: range)

        return AttributedString(attributedString)
    }

    static func signingOutdatedMetadataStepOne() -> AttributedString {
        let mainText = Localizable.TransactionSign.Error.OutdatedMetadata.step1
            .string
        let highlightedFont = Localizable.TransactionSign.Error.OutdatedMetadata.Step1.Highlight
            .font.string
        let highlightedPartOne = Localizable.TransactionSign.Error.OutdatedMetadata.Step1.Highlight
            .first.string
        let highlightedPartTwo = Localizable.TransactionSign.Error.OutdatedMetadata.Step1.Highlight
            .second.string
        let highlightedPartThree = Localizable.TransactionSign.Error.OutdatedMetadata.Step1.Highlight
            .third.string
        let attributedString = NSMutableAttributedString(string: mainText)
        let rangeFont = (mainText as NSString).range(of: highlightedFont)
        let rangePartOne = (mainText as NSString).range(of: highlightedPartOne)
        let rangePartTwo = (mainText as NSString).range(of: highlightedPartTwo)
        let rangePartThree = (mainText as NSString).range(of: highlightedPartThree)
        attributedString.setAttributes(
            [.foregroundColor: UIColor(.accentPink300)],
            range: rangePartOne
        )
        attributedString.setAttributes(
            [.foregroundColor: UIColor(.accentPink300)],
            range: rangePartTwo
        )
        attributedString.setAttributes(
            [.foregroundColor: UIColor(.textAndIconsTertiary)],
            range: rangePartThree
        )
        attributedString.addAttributes(
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
            .third.string
        let attributedString = NSMutableAttributedString(string: mainText)
        let rangeFont = (mainText as NSString).range(of: highlightedFont)
        let rangePartOne = (mainText as NSString).range(of: highlightedPartOne)
        let rangePartTwo = (mainText as NSString).range(of: highlightedPartTwo)
        let rangePartThree = (mainText as NSString).range(of: highlightedPartThree)
        attributedString.setAttributes(
            [.foregroundColor: UIColor(.accentPink300)],
            range: rangePartOne
        )
        attributedString.setAttributes(
            [.foregroundColor: UIColor(.accentPink300)],
            range: rangePartTwo
        )
        attributedString.setAttributes(
            [.foregroundColor: UIColor(.textAndIconsTertiary)],
            range: rangePartThree
        )
        attributedString.addAttributes(
            [.font: FontFamily.Inter.regular.font(size: 14)],
            range: rangeFont
        )

        return AttributedString(attributedString)
    }

    static func signingMetadataUnknownNetwork() -> AttributedString {
        let mainText = Localizable.TransactionSign.Error.MetadataUnknownNetwork.step1
            .string
        let highlightedFont = Localizable.TransactionSign.Error.MetadataUnknownNetwork.Step1.Highlight
            .font.string
        let highlightedPartOne = Localizable.TransactionSign.Error.MetadataUnknownNetwork.Step1.Highlight
            .first.string
        let highlightedPartTwo = Localizable.TransactionSign.Error.MetadataUnknownNetwork.Step1.Highlight
            .second.string
        let highlightedPartThree = Localizable.TransactionSign.Error.MetadataUnknownNetwork.Step1.Highlight
            .third.string
        let attributedString = NSMutableAttributedString(string: mainText)
        let rangeFont = (mainText as NSString).range(of: highlightedFont)
        let rangePartOne = (mainText as NSString).range(of: highlightedPartOne)
        let rangePartTwo = (mainText as NSString).range(of: highlightedPartTwo)
        let rangePartThree = (mainText as NSString).range(of: highlightedPartThree)
        attributedString.setAttributes(
            [.foregroundColor: UIColor(.accentPink300)],
            range: rangePartOne
        )
        attributedString.setAttributes(
            [.foregroundColor: UIColor(.accentPink300)],
            range: rangePartTwo
        )
        attributedString.setAttributes(
            [.foregroundColor: UIColor(.textAndIconsTertiary)],
            range: rangePartThree
        )
        attributedString.addAttributes(
            [.font: FontFamily.Inter.regular.font(size: 14)],
            range: rangeFont
        )
        return AttributedString(attributedString)
    }

    static func noMetadataForNetwork() -> AttributedString {
        let mainText = Localizable.TransactionSign.Error.NoMetadataForNetwork.step1
            .string
        let highlightedFont = Localizable.TransactionSign.Error.NoMetadataForNetwork.Step1.Highlight
            .font.string
        let highlightedPartOne = Localizable.TransactionSign.Error.NoMetadataForNetwork.Step1.Highlight
            .first.string
        let highlightedPartTwo = Localizable.TransactionSign.Error.NoMetadataForNetwork.Step1.Highlight
            .second.string
        let highlightedPartThree = Localizable.TransactionSign.Error.NoMetadataForNetwork.Step1.Highlight
            .third.string
        let attributedString = NSMutableAttributedString(string: mainText)
        let rangeFont = (mainText as NSString).range(of: highlightedFont)
        let rangePartOne = (mainText as NSString).range(of: highlightedPartOne)
        let rangePartTwo = (mainText as NSString).range(of: highlightedPartTwo)
        let rangePartThree = (mainText as NSString).range(of: highlightedPartThree)
        attributedString.setAttributes(
            [.foregroundColor: UIColor(.accentPink300)],
            range: rangePartOne
        )
        attributedString.setAttributes(
            [.foregroundColor: UIColor(.accentPink300)],
            range: rangePartTwo
        )
        attributedString.setAttributes(
            [.foregroundColor: UIColor(.textAndIconsTertiary)],
            range: rangePartThree
        )
        attributedString.addAttributes(
            [.font: FontFamily.Inter.regular.font(size: 14)],
            range: rangeFont
        )
        return AttributedString(attributedString)
    }

    static func createKeySetSeedPhraseInfo() -> AttributedString {
        let mainText = Localizable.NewSeed.Backup.Label.info.string
        let underlinedText = Localizable.NewSeed.Backup.Label.Info.underline.string

        let attributedString = NSMutableAttributedString(string: mainText)
        attributedString.addAttribute(
            .foregroundColor,
            value: UIColor(.accentPink300),
            range: NSRange(location: 0, length: mainText.count)
        )
        attributedString.addAttribute(
            .underlineStyle,
            value: NSUnderlineStyle.single.rawValue,
            range: (mainText as NSString).range(of: underlinedText)
        )
        return AttributedString(attributedString)
    }

    static func bananaSplitExplanation() -> AttributedString {
        let mainText = Localizable.NewSeed.Backup.BananaSplit.Label.content.string
        let highlightedPartOne = Localizable.NewSeed.Backup.BananaSplit.Label.Content.Highlight._1.string
        let highlightedPartTwo = Localizable.NewSeed.Backup.BananaSplit.Label.Content.Highlight._2.string
        let attributedString = NSMutableAttributedString(string: mainText)
        let rangePartOne = (mainText as NSString).range(of: highlightedPartOne)
        let rangePartTwo = (mainText as NSString).range(of: highlightedPartTwo)
        attributedString.setAttributes(
            [.foregroundColor: UIColor(.accentPink300)],
            range: rangePartOne
        )
        attributedString.setAttributes(
            [.foregroundColor: UIColor(.accentPink300)],
            range: rangePartTwo
        )
        attributedString.addAttribute(
            .underlineStyle,
            value: NSUnderlineStyle.single.rawValue,
            range: rangePartTwo
        )
        return AttributedString(attributedString)
    }

    static func setUpNetworkStepOneStepPartTwo() -> AttributedString {
        let mainText = Localizable.Onboarding.SetUpNetworks.Step1.Label.Step1.two.string
        let highlightedPart = Localizable.Onboarding.SetUpNetworks.Step1.Label.Step1.Two.highlight.string
        let attributedString = NSMutableAttributedString(string: mainText)
        let highlightedRange = (mainText as NSString).range(of: highlightedPart)
        attributedString.setAttributes(
            [.foregroundColor: UIColor(.accentPink300)],
            range: highlightedRange
        )
        return AttributedString(attributedString)
    }

    static func setUpNetworkStepOneStepPartThree() -> AttributedString {
        let mainText = Localizable.Onboarding.SetUpNetworks.Step1.Label.Step1.three.string
        let highlightedPart = Localizable.Onboarding.SetUpNetworks.Step1.Label.Step1.Three.highlight.string
        let attributedString = NSMutableAttributedString(string: mainText)
        let highlightedRange = (mainText as NSString).range(of: highlightedPart)
        attributedString.setAttributes(
            [.foregroundColor: UIColor(.accentPink300)],
            range: highlightedRange
        )
        return AttributedString(attributedString)
    }

    static func setUpNetworkStepTwoStepPartTwo() -> AttributedString {
        let mainText = Localizable.Onboarding.SetUpNetworks.Step2.Label.Step1.two.string
        let highlightedPart = Localizable.Onboarding.SetUpNetworks.Step2.Label.Step1.Two.highlight.string
        let attributedString = NSMutableAttributedString(string: mainText)
        let highlightedRange = (mainText as NSString).range(of: highlightedPart)
        attributedString.setAttributes(
            [.foregroundColor: UIColor(.accentPink300)],
            range: highlightedRange
        )
        return AttributedString(attributedString)
    }

    static func setUpNetworkStepTwoStepPartThree() -> AttributedString {
        let mainText = Localizable.Onboarding.SetUpNetworks.Step2.Label.Step1.three.string
        let highlightedPart = Localizable.Onboarding.SetUpNetworks.Step2.Label.Step1.Three.highlight.string
        let attributedString = NSMutableAttributedString(string: mainText)
        let highlightedRange = (mainText as NSString).range(of: highlightedPart)
        attributedString.setAttributes(
            [.foregroundColor: UIColor(.accentPink300)],
            range: highlightedRange
        )
        return AttributedString(attributedString)
    }

    static func applicationUpdateRequiredInfo() -> AttributedString {
        var attributedString = AttributedString(
            Localizable.Error.ApplicationUpdateRequired.Label.info.string,
            attributes: .init([.font: PrimaryFont.bodyL.font])
        )
        if let range = attributedString
            .range(of: Localizable.Error.ApplicationUpdateRequired.Label.Info.highlight.string) {
            attributedString[range].foregroundColor = Color(.accentPink300)
            attributedString[range].font = PrimaryFont.titleS.font
        }
        return attributedString
    }

    static func bananaSplitBackupQRCodeInfo() -> AttributedString {
        let mainText = Localizable.BananaSplitBackupQRCode.Label.info.string
        let highlightedText = Localizable.BananaSplitBackupQRCode.Label.Info.highlight.string

        let attributedString = NSMutableAttributedString(string: mainText)
        attributedString.addAttribute(
            .foregroundColor,
            value: Color(.textAndIconsTertiary),
            range: NSRange(location: 0, length: mainText.count)
        )

        let range = (mainText as NSString).range(of: highlightedText)
        attributedString.setAttributes([.foregroundColor: UIColor(.accentPink300)], range: range)

        return AttributedString(attributedString)
    }
}
