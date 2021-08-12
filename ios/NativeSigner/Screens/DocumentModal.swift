//
//  DocumentModal.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 11.8.2021.
//

import SwiftUI

struct DocumentModal: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 50).foregroundColor(/*@START_MENU_TOKEN@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
            VStack {
                switch data.document {
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
                    Text("About")
                case .none:
                    EmptyView()
                }
                Spacer()
            }.padding()
        }
    }
}

struct DocumentModal_Previews: PreviewProvider {
    static var previews: some View {
        DocumentModal()
    }
}
