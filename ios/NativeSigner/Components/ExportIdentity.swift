//
//  ExportIdentity.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 4.8.2021.
//

import SwiftUI

struct ExportIdentity: View {
    @EnvironmentObject var data: SignerDataModel
    @State var image: UIImage?
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 50).foregroundColor(/*@START_MENU_TOKEN@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
            VStack {
                Text("Public address")
                    .font(.largeTitle)
                    .foregroundColor(Color("AccentColor"))
                Text("Scan to export")
                    .foregroundColor(Color("textMainColor"))
                if image != nil {
                    Image(uiImage: image!)
                        .resizable()
                        .aspectRatio(contentMode: .fit)
                }
                Text(data.selectedIdentity?.name ?? "none")
                Text("for network")
                NetworkCard(network: data.selectedNetwork).padding()
                Spacer()
                Button(action: {data.exportIdentity = false})
                    {
                    Text("Done")
                        .font(.largeTitle)
                        .foregroundColor(Color("AccentColor"))
                }
            }
            .foregroundColor(/*@START_MENU_TOKEN@*/Color("textMainColor")/*@END_MENU_TOKEN@*/)
        }
        .onAppear {
            data.lastError = ""
            image = data.exportIdentityQR()
        }
        .padding(.bottom, 120)
    }
}

struct ExportIdentity_Previews: PreviewProvider {
    static var previews: some View {
        ExportIdentity()
    }
}
