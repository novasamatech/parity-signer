//
//  ManageMetadata.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 20.12.2021.
//

import SwiftUI

struct ManageMetadata: View {
    @EnvironmentObject var data: SignerDataModel
    //var content: MManageMetadata
    @State var removeMetadataAlert = false
    @State var offset: CGFloat = 0
    var body: some View {
        MenuStack {
            HeaderBar(line1: "MANAGE METADATA", line2: "Select action").padding(.top, 10)
            //MetadataCard(meta: content.forCard())
            HStack{
            Text("Used for:")
                VStack{
                    /*
                    ForEach(content.networks.sorted(by: {
                        $0.order<$1.order
                    }), id: \.order) {
                        network in
                        ZStack {
                            if network.current_on_screen {
                                RoundedRectangle(cornerRadius: 4).stroke().frame(height: 30)
                            }
                        NetworkCard(title: network.title, logo: network.logo)
                        }
                    }*/
                    EmptyView()
                }
            }
            MenuButtonsStack {
                BigButton(
                    text: "Sign this metadata",
                    isShaded: true,
                    isCrypto: true,
                    action:{data.pushButton(action: .signMetadata)}
                )
                BigButton(
                    text: "Delete this metadata",
                    isShaded: true,
                    isDangerous: true,
                    action: {removeMetadataAlert = true}
                )
            }
        }
        .offset(x: 0, y: offset)
        .gesture(DragGesture()
                    .onChanged{drag in
            self.offset = drag.translation.height
        }
                    .onEnded{drag in
            if drag.translation.height > 40 {
                self.offset = UIScreen.main.bounds.size.height
                data.pushButton(action: .goBack)
            }
        })
        .alert(isPresented: $removeMetadataAlert, content: {
            Alert(title: Text("Remove metadata?"), message: Text("This metadata will be removed for all networks"), primaryButton: .cancel(Text("Cancel")), secondaryButton: .destructive(Text("Remove metadata"), action: {data.pushButton(action: .removeMetadata)}))
        })
    }
}

/*
struct ManageMetadata_Previews: PreviewProvider {
    static var previews: some View {
        ManageMetadata()
    }
}
*/
