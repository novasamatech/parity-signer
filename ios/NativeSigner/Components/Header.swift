//
//  Header.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.9.2021.
//

import SwiftUI

struct Header: View {
    let back: Bool
    let screenLabel: String
    let screenNameType: ScreenNameType?
    let rightButton: RightButton?
    let alert: Bool
    let canaryDead: Bool
    let alertShow: () -> Void
    let pushButton: (Action, String, String) -> Void
    var body: some View {
        VStack {
            Spacer()
            HStack {
                HStack(spacing: 8.0) {
                    if back {
                        Button(
                            action: {
                                pushButton(.goBack, "", "")
                            },
                            label: {
                                Image(systemName: rightButton == .multiSelect ? "xmark" : "chevron.left")
                                    .imageScale(.large)
                                    .foregroundColor(Color("Text500"))
                            })
                    }
                    Spacer()
                }
                .frame(width: 72.0)
                Spacer()
                Text(screenLabel)
                    .foregroundColor(Color("Text600"))
                    .font(screenNameType == .h1 ? FBase(style: .h2) : FBase(style: .h4))
                    .tracking(0.1)
                if rightButton == .multiSelect {
                    Button(
                        action: {
                            pushButton(.selectAll, "", "")
                        },
                        label: {
                            SmallButton(text: "Select all")
                        })
                }
                Spacer()
                HStack(spacing: 8.0) {
                    Spacer()
                    Button(
                        action: {
                            if alert && rightButton == .newSeed {
                                alertShow()
                            } else {
                                pushButton(.rightButtonAction, "", "")
                            }
                        },
                        label: {
                            switch rightButton {
                            case .newSeed:
                                Image(systemName: "plus.circle")
                                    .imageScale(.large)
                                    .foregroundColor(Color("Action400"))
                            case .backup:
                                Image(systemName: "ellipsis")
                                    .imageScale(.large)
                                    .foregroundColor(Color("Action400"))
                            case .logRight:
                                Image(systemName: "ellipsis")
                                    .imageScale(.large)
                                    .foregroundColor(Color("Action400"))
                            case .multiSelect:
                                EmptyView()
                            case .none:
                                EmptyView()
                            default:
                                Image(systemName: "ellipsis")
                                    .imageScale(.large)
                                    .foregroundColor(Color("Action400"))
                            }
                        })
                    NavbarShield(
                        canaryDead: canaryDead,
                        alert: alert,
                        pushButton: pushButton)
                }
                .frame(width: 72.0)
            }
        }
        .frame(height: 32.0)
        .padding(.all, 8.0)
    }
}

/*
 struct Header_Previews: PreviewProvider {
 static var previews: some View {
 Header().previewLayout(.sizeThatFits)
 }
 }
 */
