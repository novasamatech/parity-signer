//
//  TransactionCardView.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 10.8.2021.
//

import SwiftUI

struct TransactionCardView: View {
    var card: TransactionCard
    var body: some View {
        VStack(alignment: .leading) {
                switch card.card {
                case .author(let author):
                    HStack {
                        Image(systemName: "circle.fill").foregroundColor(Color("AccentColor")).imageScale(.large)
                        VStack (alignment: .leading) {
                            Text("From: " + author.name)
                                .foregroundColor(Color("AccentColor"))
                            Text(author.seed + author.derivation_path)
                                .foregroundColor(Color("textMainColor"))
                            Text(author.base58)
                                .font(.caption2)
                                .foregroundColor(Color("textMainColor"))
                        }
                        Spacer()
                    }
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                case .authorPlain(let value):
                    HStack {
                        Text("From: ")
                            .foregroundColor(Color("textMainColor"))
                        Text(value.base58).foregroundColor(Color("textMainColor"))
                        Spacer()
                    }
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                case .authorPublicKey(let value):
                    HStack {
                        Image(systemName: "circle.fill").foregroundColor(Color("AccentColor")).imageScale(.large)
                        VStack (alignment: .leading) {
                            Text("Signed with " + value.crypto)
                                .foregroundColor(Color("AccentColor"))
                            Text(value.hex)
                                .font(.caption2)
                                .foregroundColor(Color("textMainColor"))
                        }
                        Spacer()
                    }
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                case .balance(let value):
                    HStack {
                        Text(value.amount)
                            .foregroundColor(Color("textMainColor"))
                        Text(value.units).foregroundColor(Color("textMainColor"))
                        Spacer()
                    }
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                case .blockHash(let text):
                    HStack {
                        Text("Block hash: ")
                            .foregroundColor(Color("AccentColor"))
                        Text(text)
                            .foregroundColor(Color("textMainColor"))
                        Spacer()
                    }
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                case .call(let value):
                    HStack {
                        Text(value.method)
                            .foregroundColor(Color("textMainColor"))
                        Text(" from ")
                            .foregroundColor(Color("AccentColor"))
                        Text(value.pallet)
                            .foregroundColor(Color("textMainColor"))
                        Spacer()
                    }
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                case .enumVariantName(let text):
                    HStack {
                        Text(text)
                            .foregroundColor(Color("textMainColor"))
                        Spacer()
                    }
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                case .eraMortalNonce(let eraMortalNonce):
                    HStack {
                        Spacer()
                        VStack {
                            Text("phase")
                                .foregroundColor(Color("AccentColor"))
                            Text(eraMortalNonce.phase)
                                .foregroundColor(Color("textMainColor"))
                        }
                        Spacer()
                        VStack {
                            Text("period")
                                .foregroundColor(Color("AccentColor"))
                            Text(eraMortalNonce.period)
                                .foregroundColor(Color("textMainColor"))
                        }
                        Spacer()
                        VStack {
                            Text("nonce")
                                .foregroundColor(Color("AccentColor"))
                            Text(eraMortalNonce.nonce)
                                .foregroundColor(Color("textMainColor"))
                        }
                        Spacer()
                    }
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                case .error(let text):
                    HStack {
                        Text("Error! ")
                            .foregroundColor(Color("textMainColor"))
                        Text(text)
                            .foregroundColor(Color("textMainColor"))
                        Spacer()
                    }
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("borderSignalColor")/*@END_MENU_TOKEN@*/)
                case .id(let text):
                    HStack {
                        Text(text)
                            .foregroundColor(Color("textMainColor"))
                        Spacer()
                    }
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                case .tip(let value):
                    HStack {
                        Text("Tip: ")
                            .foregroundColor(Color("AccentColor"))
                        Text(value.amount)
                            .foregroundColor(Color("textMainColor"))
                        Text(value.units).foregroundColor(Color("textMainColor"))
                        Spacer()
                    }
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                case .txSpec(let value):
                    HStack {
                        Spacer()
                        VStack {
                            Text("network")
                                .foregroundColor(Color("AccentColor"))
                            Text(value.network)
                                .foregroundColor(Color("textMainColor"))
                        }
                        Spacer()
                        VStack {
                            Text("spec version")
                                .foregroundColor(Color("AccentColor"))
                            Text(value.version)
                                .foregroundColor(Color("textMainColor"))
                        }
                        Spacer()
                        VStack {
                            Text("tx version")
                                .foregroundColor(Color("AccentColor"))
                            Text(value.tx_version)
                                .foregroundColor(Color("textMainColor"))
                        }
                        Spacer()
                    }
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                case .typesInfo(let text):
                    HStack {
                        Text("Types hash:")
                            .foregroundColor(Color("AccentColor"))
                        Text(text)
                            .foregroundColor(Color("textMainColor"))
                        Spacer()
                    }
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                case .varName(let text):
                    HStack {
                        Text(text)
                            .foregroundColor(Color("AccentColor"))
                        Spacer()
                    }
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                case .warning(let text):
                    HStack {
                        Text("Warning! ")
                            .foregroundColor(Color("textMainColor"))
                        Text(text)
                            .foregroundColor(Color("textMainColor"))
                        Spacer()
                    }
                    .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("AccentColor")/*@END_MENU_TOKEN@*/)
                default:
                    Text("fallback invoked")
                        .foregroundColor(Color("textMainColor"))
            }
        }
        .border(Color("borderSignalColor"))
        .padding(.leading, CGFloat(card.indent)*10.0)
    }
}
 /*
struct TransactionCardView_Previews: PreviewProvider {
    static var previews: some View {
        TransactionCardView(card: TransactionCard(index: 0, indent: 0, card: .error("this is a preview")))
    }
}*/
