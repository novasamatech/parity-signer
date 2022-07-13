//
//  DocumentModal.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 11.8.2021.
//

import SwiftUI

struct DocumentModal: View {
    @State var document: ShownDocument = .toc

    // paint top toggle buttons
    init() {
        UISegmentedControl.appearance().selectedSegmentTintColor = UIColor(Color("Bg400"))
        UISegmentedControl.appearance().backgroundColor = UIColor(Color("Bg000"))
        UISegmentedControl.appearance()
            .setTitleTextAttributes([.foregroundColor: UIColor(Color("Text600"))], for: .selected)
        UISegmentedControl.appearance()
            .setTitleTextAttributes([.foregroundColor: UIColor(Color("Text400"))], for: .normal)
    }

    var body: some View {
        ZStack {
            VStack {
                Picker("", selection: $document) {
                    ForEach(ShownDocument.allCases) { doc in
                        Text(doc.label).tag(doc).font(FBase(style: .button))
                            .foregroundColor(Color(doc == document ? "Text600" : "Text400"))
                    }
                }.pickerStyle(.segmented).listItemTint(Color("Bg000"))
                    .padding(.horizontal)
                switch document {
                case .privacyPolicy:
                    ScrollView {
                        Text(getPP())
                            .font(FBase(style: .body1))
                            .foregroundColor(Color("Text600"))
                    }.padding()
                case .toc:
                    ScrollView {
                        InstructionsSquare().padding(.bottom)
                        Text(getTaC())
                            .font(FBase(style: .body1))
                            .foregroundColor(Color("Text600"))
                    }.padding()
                }
                Spacer()
            }
        }
    }
}

/*
 struct DocumentModal_Previews: PreviewProvider {
 static var previews: some View {
 DocumentModal()
 }
 }
 */
