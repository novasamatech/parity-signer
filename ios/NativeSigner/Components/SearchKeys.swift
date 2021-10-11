//
//  SearchKeys.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.10.2021.
//

import SwiftUI

struct SearchKeys: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        HStack {
            TextField("find keys", text: $data.searchKey)
                .autocapitalization(.none)
                .disableAutocorrection(true)
                .font(.title)
                .textFieldStyle(.roundedBorder)
                .foregroundColor(Color("textEntryColor"))
            if (data.searchKey != "") {
                Button(action:{data.searchKey = ""}) {
                    Image(systemName: "clear").imageScale(.large)
                }
            } else {
                Image(systemName: "doc.text.magnifyingglass").imageScale(.large).foregroundColor(Color("AccentColor"))
            }
        }.padding(.horizontal)
    }
}

/*
 struct SearchKeys_Previews: PreviewProvider {
 static let data = SignerDataModel()
 static var previews: some View {
 SearchKeys()
 .environmentObject(data)
 .previewLayout(.sizeThatFits)
 }
 }
 */
