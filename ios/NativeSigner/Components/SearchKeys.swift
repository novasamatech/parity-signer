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
            ZStack {
                RoundedRectangle(cornerRadius: 8).stroke(Color(data.searchKey == "" ? "buttonPassiveImage" : "buttonActive")).foregroundColor(Color("backgroundColor")).frame(height: 39)
            TextField("find keys", text: $data.searchKey)
                .autocapitalization(.none)
                .disableAutocorrection(true)
                .font(.system(size: 12, design: .monospaced))
                .foregroundColor(Color("textEntryColor"))
                .padding(8)
            }
            if (data.searchKey != "") {
                Button(action:{data.searchKey = ""}) {
                    Image(systemName: "clear").imageScale(.large)
                }
            } else {
                Image(systemName: "doc.text.magnifyingglass").imageScale(.large).foregroundColor(Color("AccentColor"))
            }
        }
        .onDisappear {
            data.searchKey = ""
        }
        .background(Color("backgroundUtility"))
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
