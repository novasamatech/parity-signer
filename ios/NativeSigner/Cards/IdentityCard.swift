//
//  IdentityCard.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 3.8.2021.
//

import SwiftUI

struct IdentityCard: View {
    @EnvironmentObject var data: SignerDataModel
    var identity: Identity
    @State var delete = false
    var body: some View {
        VStack {
            Button(action:{
                if data.selectedIdentity == identity {
                    data.selectedIdentity = nil
                } else {
                    data.selectedIdentity = identity
                }
            }) {
                HStack {
                    //TODO: always fetch and mode into model; requires rust code modifications; this is a stub
                    Image(uiImage: UIImage(data: Data(fromHexEncodedString: String(cString: development_test(nil, identity.ss58)))!)!)
                    VStack (alignment: .leading) {
                        Text(identity.name)
                            .foregroundColor(Color("AccentColor"))
                        HStack {
                            Text("seed")
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
            if data.selectedIdentity == identity {
                HStack{
                    Button(action: {
                        //
                        delete = true
                    }) {
                        Text("Delete")
                    }
                    .alert(isPresented: $delete, content: {
                        Alert(
                            title: Text("Delete identity?"),
                            message: Text("You are about to delete identity " + data.selectedIdentity!.name),
                            primaryButton: .cancel(),
                            secondaryButton: .destructive(
                                Text("Delete"),
                                action: { data.deleteActiveIdentity()
                                }
                            )
                        )
                    })
                    Spacer()
                    Button(action: {
                        data.exportIdentity = true
                    }) {
                        Text("Export")
                    }
                    Spacer()
                    Button(action: {
                        data.proposeIncrement()
                        data.newIdentity = true
                    }) {
                        Text("N+1")
                    }
                    Spacer()
                    Button(action: {
                        data.proposeDerive()
                        data.newIdentity = true
                    }) {
                        Text("Derive")
                    }
                }
            }
        }.padding(5)
        .background(Color(data.selectedIdentity == identity ? "backgroundActive" : "backgroundCard"))
    }
}

/*
struct IdentityCard_Previews: PreviewProvider {
    static var previews: some View {
        IdentityCard(identity: Identity.identityData[0]).previewLayout(.sizeThatFits)
    }
}
*/
