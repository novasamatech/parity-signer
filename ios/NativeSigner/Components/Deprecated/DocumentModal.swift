//
//  DocumentModal.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 11.8.2021.
//

import SwiftUI

struct DocumentModal: View {
    @State private var document: ShownDocument = .toc

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
        ZStack {
            VStack {
                Picker("", selection: $document) {
                    ForEach(ShownDocument.allCases) { doc in
                        Text(doc.label).tag(doc).font(PrimaryFont.labelL.font)
                            .foregroundColor(
                                doc == document ? Asset.textAndIconsPrimary.swiftUIColor : Asset
                                    .textAndIconsTertiary.swiftUIColor
                            )
                    }
                }.pickerStyle(.segmented).listItemTint(Asset.backgroundPrimary.swiftUIColor)
                    .padding(.horizontal)
                switch document {
                case .privacyPolicy:
                    ScrollView {
                        Text(getPP())
                            .font(PrimaryFont.bodyL.font)
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    }.padding()
                case .toc:
                    ScrollView {
                        InstructionsSquare().padding(.bottom)
                        Text(getTaC())
                            .font(PrimaryFont.bodyL.font)
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    }.padding()
                }
                Spacer()
            }
        }
    }
}

// struct DocumentModal_Previews: PreviewProvider {
// static var previews: some View {
// DocumentModal()
// }
// }
