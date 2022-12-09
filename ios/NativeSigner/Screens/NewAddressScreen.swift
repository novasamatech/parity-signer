//
//  NewIdentityScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 4.8.2021.
//

import SwiftUI

struct NewAddressScreen: View {
    @State private var path: String = ""
    @FocusState private var focusedField: Bool
    @State private var derivationCheck: DerivationCheck?
    var content: MDeriveKey
    let pathCheck: (String, String, String) -> DerivationCheck
    let createAddress: (String, String) -> Void
    let navigationRequest: NavigationRequest

    var body: some View {
        ZStack {
            ScrollView {
                HeaderBar(
                    line1: Localizable.createNewKey.key,
                    line2: LocalizedStringKey(Localizable.forSeed(content.seedName))
                )
                NetworkCard(title: content.networkTitle, logo: content.networkLogo)
                VStack(alignment: .leading) {
                    ZStack {
                        RoundedRectangle(cornerRadius: 8).stroke(Asset.crypto400.swiftUIColor).frame(height: 39)
                        HStack {
                            Text(content.seedName)
                            TextField(
                                Localizable.path.string,
                                text: $path,
                                prompt: Localizable.NetworkInput.prompt.text
                            )
                            .foregroundColor(Asset.crypto400.swiftUIColor)
                            .font(PrimaryFont.captionM.font)
                            .disableAutocorrection(true)
                            .autocapitalization(.none)
                            .keyboardType(.asciiCapable)
                            .submitLabel(.done)
                            .onChange(of: path) { pathNew in
                                derivationCheck = pathCheck(content.seedName, pathNew, content.networkSpecsKey)
                                path = pathNew
                            }
                            .onSubmit {
                                switch derivationCheck?.whereTo {
                                case .pin:
                                    createAddress(path, content.seedName)
                                case .pwd:
                                    navigationRequest(.init(action: .checkPassword, details: path))
                                default:
                                    break
                                }
                            }
                            .focused($focusedField)
                            .padding(8)
                        }
                    }
                }.padding(.vertical)
                if let collision = derivationCheck?.collision {
                    VStack {
                        HStack {
                            Localizable.thisKeyAlreadyExists.text
                                .foregroundColor(Asset.text300.swiftUIColor)
                            Spacer()
                        }
                        AddressCard(card: collision)
                    }.padding(.bottom)
                }
                HStack {
                    BigButton(
                        text: Localizable.next.key,
                        action: {
                            switch derivationCheck?.whereTo {
                            case .pin:
                                createAddress(path, content.seedName)
                            case .pwd:
                                navigationRequest(.init(action: .checkPassword, details: path))
                            default:
                                break
                            }
                        },
                        isDisabled: derivationCheck?.buttonGood != true
                    )
                }
            }.padding(.horizontal)
        }
        .onAppear {
            path = content.suggestedDerivation
            derivationCheck = content.derivationCheck
            focusedField = content.keyboard
        }
        .onChange(of: content) { _ in
            path = content.suggestedDerivation
            derivationCheck = content.derivationCheck
            focusedField = content.keyboard
        }
    }
}

// struct NewIdentityScreen_Previews: PreviewProvider {
// static var previews: some View {
// NewIdentityScreen().previewLayout(.sizeThatFits)
// }
// }
