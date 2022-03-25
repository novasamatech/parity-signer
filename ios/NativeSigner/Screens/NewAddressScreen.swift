//
//  NewIdentityScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 4.8.2021.
//

import SwiftUI

struct NewAddressScreen: View {
    
    @EnvironmentObject var data: SignerDataModel
    @State var path: String = ""
    @FocusState private var focusedField: Bool
    @State var derivationCheck: DerivationCheck? = nil
    
    var content: MDeriveKey
    
    var body: some View {
        ZStack {
            ScrollView {
                HeaderBar(line1: "Create new key", line2: "For seed " + content.seed_name)
                //SeedCardForManager(seedName: data.selectedSeed)
                NetworkCard(title: content.network_title, logo: content.network_logo)
                VStack (alignment: .leading) {
                    //Text("DERIVATION PATH").foregroundColor(Color("Text500")).font(.footnote)
                    ZStack {
                        RoundedRectangle(cornerRadius: 8).stroke(Color("Crypto400")).frame(height: 39)
                        HStack {
                            Text(content.seed_name.decode64())
                            TextField("Path", text: $path, prompt: Text("//<network>//input"))
                                .foregroundColor(Color("Crypto400"))
                                .font(FCrypto(style: .body2))
                                .disableAutocorrection(true)
                                .autocapitalization(.none)
                                .keyboardType(.asciiCapable)
                                .submitLabel(.done)
                                .onChange(of: path) {pathNew in
                                    derivationCheck = content.updateDerivationCheck(path: pathNew, dbName: data.dbName)
                                    path = pathNew
                                }
                                .onSubmit {
                                    switch (derivationCheck?.where_to) {
                                    case .pin:
                                        data.createAddress(path: path, seedName: content.seed_name)
                                        break
                                    case .pwd:
                                        data.pushButton(buttonID: .CheckPassword, details: path)
                                        break
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
                            switch (derivationCheck?.where_to) {
                            case .pin:
                                data.createAddress(path: path, seedName: content.seed_name)
                                break
                            case .pwd:
                                data.pushButton(buttonID: .CheckPassword, details: path)
                                break
                            default:
                                break
                            }
                        },
                        isDisabled: derivationCheck?.button_good != true)
                }
            }.padding(.horizontal)
        }
        .onAppear {
            path = content.suggested_derivation
            derivationCheck = content.derivation_check
            focusedField = content.keyboard
        }
        .onChange(of: content) { newContent in
            path = newContent.suggested_derivation
            derivationCheck = newContent.derivation_check
            focusedField = newContent.keyboard
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
