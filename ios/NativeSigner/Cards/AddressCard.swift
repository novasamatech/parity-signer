//
//  IdentityCard.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 3.8.2021.
//

import SwiftUI

struct AddressCard: View {
    @EnvironmentObject var data: SignerDataModel
    var identity: Address
    @State var delete = false
    var body: some View {
        VStack {
            Button(action:{
                if data.selectedAddress == identity {
                    data.selectedAddress = nil
                } else {
                    data.selectedAddress = identity
                }
            }) {
                HStack {
                    //TODO: always fetch and mode into model; requires rust code modifications; this is a stub
                    Image(uiImage: UIImage(data: Data(fromHexEncodedString: String(cString: base58_identicon(nil, identity.ss58, 32)))!)!)
                    VStack (alignment: .leading) {
                        Text(identity.name)
                            .foregroundColor(Color("AccentColor"))
                        HStack {
                            Text(identity.seed_name)
                                .foregroundColor(Color("AccentColor"))
                            Text(identity.path)
                                .foregroundColor(Color("textMainColor"))
                            if identity.has_password == "true" {
                                Image(systemName: "lock")
                                    .foregroundColor(Color("AccentColor"))
                            }
                        }
                        Text(identity.ss58)
                            .font(.caption2)
                            .foregroundColor(Color("textMainColor"))
                    }
                    Spacer()
                }
            }
            if data.selectedAddress == identity {
                HStack{
                    Button(action: {
                        //
                        delete = true
                    }) {
                        Text("Delete")
                    }
                    .alert(isPresented: $delete, content: {
                        Alert(
                            title: Text("Delete key?"),
                            message: Text("You are about to delete key " + data.selectedAddress!.name),
                            primaryButton: .cancel(),
                            secondaryButton: .destructive(
                                Text("Delete"),
                                action: { data.deleteSelectedAddress()
                                }
                            )
                        )
                    })
                    Spacer()
                    Button(action: {
                        data.keyManagerModal = .showKey
                    }) {
                        Text("Export")
                    }
                    Spacer()
                    Button(action: {
                        data.selectSeed(seedName: data.selectedAddress!.seed_name)
                        data.proposeIncrement()
                        data.keyManagerModal = .newKey
                    }) {
                        Text("N+1")
                    }
                    Spacer()
                    Button(action: {
                        data.selectSeed(seedName: data.selectedAddress!.seed_name)
                        data.proposeDerive()
                        data.keyManagerModal = .newKey
                    }) {
                        Text("Derive")
                    }
                }
            }
        }.padding(5)
        .background(Color(data.selectedAddress == identity ? "backgroundActive" : "backgroundCard"))
    }
}

/*
struct IdentityCard_Previews: PreviewProvider {
    static var previews: some View {
        IdentityCard(identity: Identity.identityData[0]).previewLayout(.sizeThatFits)
    }
}
*/
