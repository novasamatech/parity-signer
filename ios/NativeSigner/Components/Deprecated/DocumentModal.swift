//
//  DocumentModal.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 11.8.2021.
//

import SwiftUI

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
                    InstructionsSquare().padding(.bottom)
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
