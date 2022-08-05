//
//  KeyList.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

struct KeyManager: View {
    @GestureState private var dragOffset = CGSize.zero
    let content: MKeys
    let alert: Bool
    let alertShow: () -> Void
    let increment: (String, String) -> Void
    let pushButton: (Action, String, String) -> Void
    @State var searchString: String = "" // This is supposed to be used with search, which is disabled now
    var body: some View {
        ZStack {
            VStack {
                ZStack {
                    Button(
                        action: {
                            pushButton(.selectKey, content.root.addressKey, "")
                        },
                        label: {
                            SeedKeyCard(
                                seedCard: content.root,
                                multiselectMode: content.multiselectMode
                            )
                                .gesture(DragGesture()
                                            .onEnded {drag in
                                    if abs(drag.translation.height) < 20 && abs(drag.translation.width) > 20 {
                                        if content.root.addressKey != "" {
                                            pushButton(.swipe, content.root.addressKey, "")
                                        }
                                    }
                                })
                                .gesture(LongPressGesture()
                                            .onEnded {_ in
                                    if content.root.addressKey != "" {
                                        pushButton(.longTap, content.root.addressKey, "")
                                    }
                                }
                                )
                        })
                        .disabled(content.root.addressKey == "")
                        .padding(2)
                    if content.root.swiped {
                        AddressCardControls(
                            seedName: content.root.seedName,
                            increment: { details in
                                increment(content.root.seedName, details)
                            },
                            pushButton: pushButton
                        )
                    }
                }
                Button(
                    action: {pushButton(.networkSelector, "", "")},
                    label: {
                        HStack {
                            NetworkCard(title: content.network.title, logo: content.network.logo)
                            Image(systemName: "chevron.down")
                            Spacer()
                        }
                    })
                HStack {
                    Text("DERIVED KEYS").foregroundColor(Color("Text300")).font(FBase(style: .overline))
                    Spacer()
                    Button(
                        action: {
                            if alert {
                                alertShow()
                            } else {
                                pushButton(.newKey, "", "")
                            }
                        },
                        label: {
                            Image(systemName: "plus.circle").imageScale(.large).foregroundColor(Color("Action400"))
                        })
                }.padding(.horizontal, 8)
                ScrollView {
                    LazyVStack {
                        ForEach(content.set.sorted(by: {$0.path < $1.path}).filter {card in
                            return card.path.contains(searchString) || searchString == ""
                        }, id: \.addressKey) {address in
                            ZStack {
                                Button(
                                    action: {
                                        pushButton(.selectKey, address.addressKey, "")
                                    },
                                    label: {
                                        AddressCard(
                                            address: Address(
                                                base58: address.base58,
                                                path: address.path,
                                                hasPwd: address.hasPwd,
                                                identicon: address.identicon,
                                                seedName: "",
                                                multiselect: address.multiselect
                                            ),
                                            multiselectMode: content.multiselectMode
                                        )
                                            .gesture(DragGesture().onEnded {drag in
                                                if abs(drag.translation.height) < 20 &&
                                                    abs(drag.translation.width) > 20 {
                                                    pushButton(.swipe, address.addressKey, "")
                                                }
                                            })
                                            .gesture(LongPressGesture()
                                                        .onEnded {_ in
                                                pushButton(.longTap, address.addressKey, "")
                                            }
                                            )
                                    }).padding(2)
                                if address.swiped {
                                    AddressCardControls(
                                        seedName: content.root.seedName,
                                        increment: { details in
                                            increment(content.root.seedName, details)
                                        },
                                        pushButton: pushButton
                                    )
                                }
                            }
                        }
                    }
                }.padding(.bottom, -20)
                Spacer()
                if content.multiselectMode {
                    MultiselectBottomControl(
                        selectedCount: content.multiselectCount,
                        pushButton: pushButton
                    )
                } else {
                    // SearchKeys(searchString: $searchString)
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
