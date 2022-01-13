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
    @State var searchString: String = "" // This is supposed to be used with search, which is disabled now
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
                                if content.root.address_key != "" {
                                    data.pushButton(buttonID: .Swipe, details: content.root.address_key)
                                }
                            }
                        })
                            .gesture(LongPressGesture()
                                        .onEnded {_ in
                                if content.root.address_key != "" {
                                    data.pushButton(buttonID: .LongTap, details: content.root.address_key)
                                }
                            }
                            )
                    }
                    .disabled(content.root.address_key == "")
                    .padding(2)
                    if content.root.swiped {
                        AddressCardControls(seed_name: content.root.seed_name)
                    }
                }
                Button(action: {data.pushButton(buttonID: .NetworkSelector)}) {
                    HStack {
                        NetworkCard(title: content.network.title, logo: content.network.logo)
                        Image(systemName: "chevron.down")
                        Spacer()
                    }
                }
                HStack {
                    Text("DERIVED KEYS").foregroundColor(Color("Text300")).font(FBase(style: .overline))
                    Spacer()
                    Button(action: {
                        if data.alert {
                            data.alertShow = true
                        } else {
                            data.pushButton(buttonID: .NewKey)
                        }
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
                }.padding(.bottom, -20)
                Spacer()
                if (content.multiselect_mode) {
                    MultiselectBottomControl(selectedCount: content.multiselect_count)
                } else {
                    //SearchKeys(searchString: $searchString)
                    EmptyView()
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
