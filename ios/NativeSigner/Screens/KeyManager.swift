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
    let navigationRequest: NavigationRequest
    @State private var searchString: String = "" // This is supposed to be used with search, which is disabled now
    var body: some View {
        ZStack {
            VStack {
                ZStack {
                    Button(
                        action: {
                            navigationRequest(.init(action: .selectKey, details: content.root.addressKey))
                        },
                        label: {
                            SeedKeyCard(
                                seedCard: content.root,
                                multiselectMode: content.multiselectMode
                            )
                            .gesture(
                                DragGesture()
                                    .onEnded { drag in
                                        if abs(drag.translation.height) < 20, abs(drag.translation.width) > 20 {
                                            if !content.root.addressKey.isEmpty {
                                                navigationRequest(.init(
                                                    action: .swipe,
                                                    details: content.root.addressKey
                                                ))
                                            }
                                        }
                                    }
                            )
                            .gesture(
                                LongPressGesture()
                                    .onEnded { _ in
                                        if !content.root.addressKey.isEmpty {
                                            navigationRequest(.init(action: .longTap, details: content.root.addressKey))
                                        }
                                    }
                            )
                        }
                    )
                    .disabled(content.root.addressKey.isEmpty)
                    .padding(2)
                    if content.root.swiped {
                        AddressCardControls(
                            seedName: content.root.seedName,
                            increment: { details in
                                increment(content.root.seedName, details)
                            },
                            navigationRequest: navigationRequest
                        )
                    }
                }
                Button(
                    action: { navigationRequest(.init(action: .networkSelector)) },
                    label: {
                        HStack {
                            NetworkCard(title: content.network.title, logo: content.network.logo)
                            Image(.chevron, variant: .down)
                            Spacer()
                        }
                    }
                )
                HStack {
                    Text("DERIVED KEYS").foregroundColor(Asset.text300.swiftUIColor).font(Fontstyle.overline.base)
                    Spacer()
                    Button(
                        action: {
                            if alert {
                                alertShow()
                            } else {
                                navigationRequest(.init(action: .newKey))
                            }
                        },
                        label: {
                            Image(.plus, variant: .circle).imageScale(.large)
                                .foregroundColor(Asset.action400.swiftUIColor)
                        }
                    )
                }.padding(.horizontal, 8)
                ScrollView {
                    LazyVStack {
                        ForEach(content.set.sorted(by: { $0.path < $1.path }).filter { card in
                            card.path.contains(searchString) || searchString.isEmpty
                        }, id: \.addressKey) { address in
                            ZStack {
                                Button(
                                    action: {
                                        navigationRequest(.init(action: .selectKey, details: address.addressKey))
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
                                        .gesture(DragGesture().onEnded { drag in
                                            if abs(drag.translation.height) < 20,
                                               abs(drag.translation.width) > 20 {
                                                navigationRequest(.init(action: .swipe, details: address.addressKey))
                                            }
                                        })
                                        .gesture(
                                            LongPressGesture()
                                                .onEnded { _ in
                                                    navigationRequest(.init(
                                                        action: .longTap,
                                                        details: address.addressKey
                                                    ))
                                                }
                                        )
                                    }
                                ).padding(2)
                                if address.swiped {
                                    AddressCardControls(
                                        seedName: content.root.seedName,
                                        increment: { details in
                                            increment(content.root.seedName, details)
                                        },
                                        navigationRequest: navigationRequest
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
                        navigationRequest: navigationRequest
                    )
                } else {
                    // SearchKeys(searchString: $searchString)
                    EmptyView()
                }
            }
        }
    }
}

// struct KeyManager_Previews: PreviewProvider {
// static var previews: some View {
// NavigationView {
// KeyManager()
// }
// }
// }
