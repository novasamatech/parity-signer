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
        UISegmentedControl.appearance().selectedSegmentTintColor = UIColor(Asset.bg400.swiftUIColor)
        UISegmentedControl.appearance().backgroundColor = UIColor(Asset.bg000.swiftUIColor)
        UISegmentedControl.appearance()
            .setTitleTextAttributes([.foregroundColor: UIColor(Asset.text600.swiftUIColor)], for: .selected)
        UISegmentedControl.appearance()
            .setTitleTextAttributes([.foregroundColor: UIColor(Asset.text400.swiftUIColor)], for: .normal)
    }

    var body: some View {
        ZStack {
            VStack {
                Picker("", selection: $document) {
                    ForEach(ShownDocument.allCases) { doc in
                        Text(doc.label).tag(doc).font(Fontstyle.button.base)
                            .foregroundColor(doc == document ? Asset.text600.swiftUIColor : Asset.text400.swiftUIColor)
                    }
                }.pickerStyle(.segmented).listItemTint(Asset.bg000.swiftUIColor)
                    .padding(.horizontal)
                switch document {
                case .privacyPolicy:
                    ScrollView {
                        Text(getPP())
                            .font(Fontstyle.body1.base)
                            .foregroundColor(Asset.text600.swiftUIColor)
                    }.padding()
                case .toc:
                    ScrollView {
                        InstructionsSquare().padding(.bottom)
                        Text(getTaC())
                            .font(Fontstyle.body1.base)
                            .foregroundColor(Asset.text600.swiftUIColor)
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
