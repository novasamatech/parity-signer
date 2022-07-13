//
//  SearchKeys.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.10.2021.
//

import SwiftUI

/**
 * Text entry to search for keys in keys menu
 * Gets cleared on button press
 */
struct SearchKeys: View {
    @Binding var searchString: String
    var body: some View {
        HStack {
            ZStack {
                RoundedRectangle(cornerRadius: 8)
                    .stroke(Color(searchString == "" ? "Action100" : "Action400")).frame(height: 39)
                TextField("find keys", text: $searchString)
                    .autocapitalization(.none)
                    .disableAutocorrection(true)
                    .font(.system(size: 12, design: .monospaced))
                    .foregroundColor(Color("Text400"))
                    .padding(8)
            }
            if searchString != "" {
                Button(
                    action: {searchString = ""},
                    label: {
                        Image(systemName: "clear").imageScale(.large)
                    })
            } else {
                Image(systemName: "doc.text.magnifyingglass").imageScale(.large).foregroundColor(Color("Action400"))
            }
        }
        .onDisappear {
            searchString = ""
        }
        .background(Color("Bg000"))
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
