//
//  IdentityCard.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 3.8.2021.
//

import SwiftUI

/// Card for showing any address.
/// Accepts Address object
struct AddressCard: View {
    @EnvironmentObject var data: SignerDataModel
    var address: Address
    var multiselectMode: Bool = false
    @GestureState private var dragOffset = CGSize.zero
    let rowHeight: CGFloat = 28
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 4).foregroundColor(Asset.bg200.swiftUIColor).frame(height: 44)
            HStack {
                ZStack {
                    Identicon(identicon: address.identicon)
                    if multiselectMode {
                        VStack {
                            Spacer()
                            HStack {
                                Spacer()
                                Image(
                                    systemName: address.multiselect == true ?
                                        "checkmark.circle.fill" :
                                        "circle"
                                ).imageScale(.large)
                            }
                        }
                    }
                }.frame(width: 30, height: 30)
                VStack(alignment: .leading) {
                    HStack {
                        Text(address.seedName).foregroundColor(Asset.text600.swiftUIColor)
                            .font(Fontstyle.subtitle1.base)
                        Text(address.path)
                        if address.hasPwd {
                            Text("///").foregroundColor(Asset.crypto400.swiftUIColor)
                                .font(Fontstyle.body2.crypto)
                            Image(.lock).foregroundColor(Asset.crypto400.swiftUIColor)
                                .font(Fontstyle.body2.crypto)
                        }
                    }.foregroundColor(Asset.crypto400.swiftUIColor)
                        .font(Fontstyle.body2.crypto)
                    // Here we could have shortened base58 address when buttons are shown, but we don't need to
                    Text(address.base58.truncateMiddle(length: 8)).foregroundColor(Asset.text400.swiftUIColor)
                        .font(Fontstyle.body1.crypto)
                }
                Spacer()
            }.padding(.horizontal, 8)
        }
    }
}
