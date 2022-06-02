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
                        data.pushButton(action: .selectKey, details: content.root.addressKey)
                    }){
                        SeedKeyCard(seedCard: content.root, multiselectMode: content.multiselectMode).gesture(DragGesture()
                                                                                                                .onEnded {drag in
                            if abs(drag.translation.height) < 20 && abs(drag.translation.width) > 20 {
                                if content.root.addressKey != "" {
                                    data.pushButton(action: .swipe, details: content.root.addressKey)
                                }
                            }
                        })
                            .gesture(LongPressGesture()
                                        .onEnded {_ in
                                if content.root.addressKey != "" {
                                    data.pushButton(action: .longTap, details: content.root.addressKey)
                                }
                            }
                            )
                    }
                    .disabled(content.root.addressKey == "")
                    .padding(2)
                    if content.root.swiped {
                        AddressCardControls(seed_name: content.root.seedName)
                    }
                }
                Button(action: {data.pushButton(action: .networkSelector)}) {
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
                            data.pushButton(action: .newKey)
                        }
                    }) {
                        Image(systemName: "plus.circle").imageScale(.large).foregroundColor(Color("Action400"))
                    }
                }.padding(.horizontal, 8)
                ScrollView {
                    LazyVStack {
                        ForEach(content.set.sorted(by: {$0.path < $1.path}).filter{card in
                            return card.path.contains(searchString) || searchString == ""
                        }, id: \.addressKey) {
                            address in
                            ZStack {
                                Button(action: {
                                    data.pushButton(action: .selectKey, details: address.addressKey)
                                }){
                                    AddressCard(address: Address(base58: address.base58 , path: address.path, hasPwd: address.hasPwd, identicon: address.identicon, seedName: "", multiselect: address.multiselect), multiselectMode: content.multiselectMode).gesture(DragGesture()
                                                                                                                                    .onEnded {drag in
                                        if abs(drag.translation.height) < 20 && abs(drag.translation.width) > 20 {
                                            data.pushButton(action: .swipe, details: address.addressKey)
                                        }
                                    })
                                        .gesture(LongPressGesture()
                                                    .onEnded {_ in
                                            data.pushButton(action: .longTap, details: address.addressKey)
                                        }
                                        )
                                }.padding(2)
                                if address.swiped {
                                    AddressCardControls(seed_name: content.root.seedName)
                                }
                            }
                        }
                    }
                }.padding(.bottom, -20)
                Spacer()
                if (content.multiselectMode) {
                    MultiselectBottomControl(selectedCount: content.multiselectCount)
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
