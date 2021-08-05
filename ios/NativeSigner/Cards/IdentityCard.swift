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
                    Image(systemName: "circle.fill").foregroundColor(Color("AccentColor")).imageScale(.large)
                    VStack (alignment: .leading) {
                        Text(identity.name)
                            .foregroundColor(Color("AccentColor"))
                        Text("root" + identity.path)
                            .foregroundColor(Color("textMainColor"))
                        Text(identity.ss58)
                            .font(.caption2)
                            .foregroundColor(Color("textMainColor"))
                    }
                }
            }
            if data.selectedIdentity == identity {
                HStack{
                    Button(action: {
                        data.deleteActiveIdentity()
                    }) {
                        Text("Delete")
                    }
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

struct IdentityCard_Previews: PreviewProvider {
    static var previews: some View {
        IdentityCard(identity: Identity.identityData[0]).previewLayout(.sizeThatFits)
    }
}
