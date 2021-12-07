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
                            .foregroundColor(Color("textMainColor"))
                    }
                case .toc:
                    ScrollView {
                        Text(data.getTaC())
                            .foregroundColor(Color("textMainColor"))
                    }.padding()
                case .about:
                    ScrollView {
                        Text("About")
                        Text("lorem ipsum")
                    }
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
