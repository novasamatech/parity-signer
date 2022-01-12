//
//  DocumentModal.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 11.8.2021.
//

import SwiftUI

struct DocumentModal: View {
    @EnvironmentObject var data: SignerDataModel
    @State var document: ShownDocument = .toc
    
    //paint top toggle buttons
    
    init() {
        UISegmentedControl.appearance().selectedSegmentTintColor = UIColor(Color("Bg400"))
        UISegmentedControl.appearance().backgroundColor = UIColor(Color("Bg000"))
        //UISegmentedControl.appearance().tintColor = UIColor(Color("Text600"))
        UISegmentedControl.appearance().setTitleTextAttributes([.foregroundColor: UIColor(Color("Text600"))], for: .selected)
        UISegmentedControl.appearance().setTitleTextAttributes([.foregroundColor: UIColor(Color("Text400"))], for: .normal)
    }
    
    var body: some View {
        ZStack {
            VStack {
                Picker ("", selection: $document) {
                    ForEach(ShownDocument.allCases) { doc in
                        Text(doc.label).tag(doc).font(FBase(style: .button))
                            .foregroundColor(Color(doc == document ? "Text600" : "Text400"))
                    }
                }.pickerStyle(.segmented).listItemTint(Color("Bg000"))
                    .padding(.horizontal)
                switch document {
                case .pp:
                    ScrollView {
                        Text(data.getPP())
                            .font(FBase(style: .body1))
                            .foregroundColor(Color("Text600"))
                    }.padding()
                case .toc:
                    ScrollView {
                        InstructionsSquare().padding(.bottom)
                        Text(data.getTaC())
                            .font(FBase(style: .body1))
                            .foregroundColor(Color("Text600"))
                    }.padding()
                    /*
                case .about:
                    ScrollView {
                        Text("About").font(FBase(style: .h1)).foregroundColor(Color("Text600")).padding(.bottom, 10)
                        Text("Parity Signer is an app for a device you keep offline.\nIt utilizes hardware-level security of your smartphone to turn it into a so-called cold storage for private keys from your blockchain accounts.\nWith Signer you can generate, manage and use blockchain credentials to sign blockchain transactions and otherwise use it as an air-gapped crypto wallet.").font(FBase(style: .body1)).foregroundColor(Color("Text600")).padding(.bottom, 10)
                        Text("For more information, tutorials and documentation, visit").font(FBase(style: .body1)).foregroundColor(Color("Text600")).padding(.bottom, 10)
                        Text("www.parity.io/technologies/signer").font(FBase(style: .button)).foregroundColor(Color("Action400")).padding(.bottom, 10)
                        SettingsCardTemplate(
                            text: "App version: " + (data.appVersion ?? "Unknown!"),
                            withIcon: false,
                            withBackground: false
                        )
                    }.padding(.horizontal)
                     */
                }
                Spacer()
            }//.padding()
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
