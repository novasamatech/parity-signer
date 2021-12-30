//
//  KeyList.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

struct KeyManager: View {
    @EnvironmentObject var data: SignerDataModel
    @GestureState private var dragOffset = CGSize.zero
    var content: MKeys
    @State var searchString: String = ""
    var body: some View {
        ZStack {
            VStack {
                ZStack{
                    Button(action: {
                        data.pushButton(buttonID: .SelectKey, details: content.root.address_key)
                    }){
                        SeedKeyCard(seedCard: content.root, multiselectMode: content.multiselect_mode).gesture(DragGesture()
                                                                        .onEnded {drag in
                            if abs(drag.translation.height) < 20 && abs(drag.translation.width) > 20 {
                                data.pushButton(buttonID: .Swipe, details: content.root.address_key)
                            }
                        })
                            .gesture(LongPressGesture()
                                        .onEnded {_ in
                                data.pushButton(buttonID: .LongTap, details: content.root.address_key)
                            }
                            )
                    }.padding(2)
                    if content.root.swiped {
                        AddressCardControls(seed_name: content.root.seed_name)
                    }
                }
                Button(action: {data.pushButton(buttonID: .NetworkSelector)}) {
                    NetworkCard(title: content.network.title, logo: content.network.logo)
                }
                HStack {
                    Text("DERIVED KEYS").foregroundColor(Color("Text600"))
                    Spacer()
                    Button(action: {
                        data.pushButton(buttonID: .NewKey)
                    }) {
                        Image(systemName: "plus.circle").imageScale(.large).foregroundColor(Color("Action400"))
                    }
                }.padding(.horizontal, 8)
                ScrollView {
                    LazyVStack {
                        ForEach(content.set.sorted(by: {$0.path < $1.path}).filter{card in
                            return card.path.contains(searchString) || searchString == ""
                        }, id: \.address_key) {
                            address in
                            ZStack {
                                Button(action: {
                                    data.pushButton(buttonID: .SelectKey, details: address.address_key)
                                }){
                                    AddressCard(address: address.intoAddress(), multiselectMode: content.multiselect_mode).gesture(DragGesture()
                                                                                            .onEnded {drag in
                                        if abs(drag.translation.height) < 20 && abs(drag.translation.width) > 20 {
                                            data.pushButton(buttonID: .Swipe, details: address.address_key)
                                        }
                                    })
                                        .gesture(LongPressGesture()
                                                    .onEnded {_ in
                                            data.pushButton(buttonID: .LongTap, details: address.address_key)
                                        }
                                        )
                                }.padding(2)
                                if address.swiped {
                                    AddressCardControls(seed_name: content.root.seed_name)
                                }
                            }
                        }
                    }
                }
                Spacer()
                if (content.multiselect_mode) {
                    MultiselectBottomControl(selectedCount: content.multiselect_count)
                } else {
                SearchKeys(searchString: $searchString)
                }
            }
        }
    }
}

/*
 struct KeyManager_Previews: PreviewProvider {
 static var previews: some View {
 NavigationView {
 KeyManager()
 }
 }
 }
 */
