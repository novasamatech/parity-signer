//
//  SearchKeys.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.10.2021.
//

import SwiftUI

/// Text entry to search for keys in keys menu
/// Gets cleared on button press
struct SearchKeys: View {
    @Binding var searchString: String
    var body: some View {
        HStack {
            ZStack {
                RoundedRectangle(cornerRadius: 8)
                    .stroke(searchString.isEmpty ? Asset.accentPink300.swiftUIColor : Asset.accentPink500.swiftUIColor)
                    .frame(height: 39)
                TextField(Localizable.findKeys.string, text: $searchString)
                    .autocapitalization(.none)
                    .disableAutocorrection(true)
                    .font(.system(size: 12, design: .monospaced))
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .padding(8)
            }
            if !searchString.isEmpty {
                Button(
                    action: { searchString = "" },
                    label: {
                        Image(.clear).imageScale(.large)
                    }
                )
            } else {
                Image(.doc, variants: [.text, .magnifyingglass]).imageScale(.large)
                    .foregroundColor(Asset.accentPink300.swiftUIColor)
            }
        }
        .onDisappear {
            searchString = ""
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
    }
}

// struct SearchKeys_Previews: PreviewProvider {
// static let data = SignerDataModel()
// static var previews: some View {
// SearchKeys()
// .environmentObject(data)
// .previewLayout(.sizeThatFits)
// }
// }
