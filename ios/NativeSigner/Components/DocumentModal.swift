//
//  DocumentModal.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 11.8.2021.
//

import SwiftUI

struct DocumentModal: View {
    @EnvironmentObject var data: SignerDataModel
    @State var document: ShownDocument = .about
    var body: some View {
        ZStack {
            VStack {
                HStack {
                    Button(action: {document = .about}) {
                        Text("About")
                    }
                    Spacer()
                    Button(action: {document = .toc}) {
                        Text("Terms")
                    }
                    Spacer()
                    Button(action: {document = .pp}) {
                        Text("Privacy")
                    }
                }
                .padding()
                switch document {
                case .pp:
                    ScrollView {
                        Text(data.getPP())
                            .foregroundColor(Color("Text600"))
                    }
                case .toc:
                    ScrollView {
                        Text(data.getTaC())
                            .foregroundColor(Color("Text600"))
                    }.padding()
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
                }
                Spacer()
            }.padding()
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
