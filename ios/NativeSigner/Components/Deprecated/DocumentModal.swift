//
//  DocumentModal.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 11.8.2021.
//

import Foundation
import SwiftUI
import UIKit

struct DocumentModal: View {
    @State private var document: ShownDocument = .toc
    var documents: [ShownDocument] = ShownDocument.allCases

    // paint top toggle buttons
    init() {
        UISegmentedControl.appearance().selectedSegmentTintColor = UIColor(Asset.backgroundTertiary.swiftUIColor)
        UISegmentedControl.appearance().backgroundColor = UIColor(Asset.backgroundPrimary.swiftUIColor)
        UISegmentedControl.appearance()
            .setTitleTextAttributes([.foregroundColor: UIColor(Asset.textAndIconsPrimary.swiftUIColor)], for: .selected)
        UISegmentedControl.appearance()
            .setTitleTextAttributes([.foregroundColor: UIColor(Asset.textAndIconsTertiary.swiftUIColor)], for: .normal)
    }

    var body: some View {
        VStack {
            Picker("", selection: $document) {
                ForEach(documents, id: \.self) { current in
                    Text(current.label)
                        .tag(current.label)
                        .font(PrimaryFont.labelL.font)
                        .foregroundColor(
                            current == document ? Asset.textAndIconsPrimary.swiftUIColor : Asset
                                .textAndIconsTertiary.swiftUIColor
                        )
                }
            }
            .pickerStyle(SegmentedPickerStyle())
            .listItemTint(Asset.backgroundPrimary.swiftUIColor)
            .padding(.horizontal)
            ScrollView {
                if document == .toc {
                    VStack(alignment: .leading) {
                        Image(.airplane)
                        Localizable.useSignerInAirplaneMode.text
                            .font(PrimaryFont.bodyL.font)
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        Localizable.AirplaneMode.explanation.text
                            .font(PrimaryFont.bodyM.font)
                            .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                        Image(.wifi, variant: .slash)
                        Localizable.airgapYourPhone.text
                            .font(PrimaryFont.bodyL.font)
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        Localizable.Connectivity.explanation.text
                            .font(PrimaryFont.bodyM.font).foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    }
                    .padding(16)
                    .background(
                        RoundedRectangle(cornerRadius: 8)
                            .foregroundColor(Asset.backgroundSecondary.swiftUIColor)
                    )
                    .padding(.bottom)
                }
                Text(document.text)
                    .font(PrimaryFont.bodyL.font)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
            }
            .padding()
        }
    }
}

// struct DocumentModal_Previews: PreviewProvider {
// static var previews: some View {
// DocumentModal()
// }
// }
//
// AttributedString(
//    markdown: tac,
//    options: AttributedString.MarkdownParsingOptions(interpretedSyntax: .inlineOnlyPreservingWhitespace)
// )
