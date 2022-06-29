//
//  NewIdentityScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 4.8.2021.
//

import SwiftUI

struct NewAddressScreen: View {
    @State var path: String = ""
    @FocusState private var focusedField: Bool
    @State private var derivationCheck: DerivationCheck?
    var content: MDeriveKey
    let pathCheck: (String, String, String) -> DerivationCheck
    let createAddress: (String, String) -> Void
    let pushButton: (Action, String, String) -> Void

    var body: some View {
        ZStack {
            ScrollView {
                HeaderBar(line1: "Create new key", line2: "For seed " + content.seedName)
                NetworkCard(title: content.networkTitle, logo: content.networkLogo)
                VStack(alignment: .leading) {
                    ZStack {
                        RoundedRectangle(cornerRadius: 8).stroke(Color("Crypto400")).frame(height: 39)
                        HStack {
                            Text(content.seedName)
                            TextField("Path", text: $path, prompt: Text("//<network>//input"))
                                .foregroundColor(Color("Crypto400"))
                                .font(FCrypto(style: .body2))
                                .disableAutocorrection(true)
                                .autocapitalization(.none)
                                .keyboardType(.asciiCapable)
                                .submitLabel(.done)
                                .onChange(of: path) {pathNew in
                                    derivationCheck = pathCheck(content.seedName, pathNew, content.networkSpecsKey)
                                    path = pathNew
                                }
                                .onSubmit {
                                    switch derivationCheck?.whereTo {
                                    case .pin:
                                        createAddress(path, content.seedName)
                                    case .pwd:
                                        pushButton(.checkPassword, path, "")
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
                            Text("This key already exists:").foregroundColor(Color("Text300"))
                            Spacer()
                        }
                        AddressCard(address: collision)
                    }.padding(.bottom)
                }
                HStack {
                    BigButton(
                        text: "Next",
                        action: {
                            switch derivationCheck?.whereTo {
                            case .pin:
                                createAddress(path, content.seedName)
                            case .pwd:
                                pushButton(.checkPassword, path, "")
                            default:
                                break
                            }
                        },
                        isDisabled: derivationCheck?.buttonGood != true)
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

/*
 struct NewIdentityScreen_Previews: PreviewProvider {
 static var previews: some View {
 NewIdentityScreen().previewLayout(.sizeThatFits)
 }
 }
 */
