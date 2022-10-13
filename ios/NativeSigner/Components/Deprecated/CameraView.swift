//
//  CameraView.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import AVFoundation
import SwiftUI

struct CameraView: View {
    @StateObject var model = CameraService()
    @EnvironmentObject private var navigation: NavigationCoordinator
    let size = UIScreen.main.bounds.size.width
    @Binding var isPresented: Bool

    var body: some View {
        ZStack {
            VStack {
                NavigationBarView(
                    viewModel: .init(leftButton: .xmark, backgroundColor: .clear),
                    actionModel: .init(
                        leftBarMenuAction: {
                            isPresented.toggle()
                        }
                    )
                )
                CameraPreview(session: model.session)
                    .onAppear {
                        model.configure()
                    }
                    .onDisappear {
                        UIApplication.shared.isIdleTimerDisabled = false
                        model.shutdown()
                    }
                    .onReceive(model.$payload, perform: { payload in
                        if payload != nil {
                            DispatchQueue.main.async {
                                navigation.perform(navigation: .init(action: .transactionFetched, details: payload))
                            }
                        }
                    })
                    .onChange(of: model.captured, perform: { newValue in
                        UIApplication.shared.isIdleTimerDisabled = newValue > 0
                    })
                    .mask(
                        VStack {
                            ZStack {
                                RoundedRectangle(cornerRadius: 8).padding(12)
                            }
                            .frame(width: size, height: size)
                            Spacer()
                        }
                    )
                    .overlay(
                        VStack {
                            RoundedRectangle(cornerRadius: 8)
                                .stroke(Asset.crypto400.swiftUIColor)
                                .padding(12)
                                .frame(width: size, height: size)
                            Spacer()
                        }
                    )
                Spacer()
                if model.total > 1 {
                    MenuStack {
                        HeadingOverline(text: Localizable.CameraView.parsingMultidata.key).padding(.top, 12)
                        ProgressView(value: min(Float(model.captured) / Float(model.total), 1))
                            .border(Asset.crypto400.swiftUIColor)
                            .foregroundColor(Asset.crypto400.swiftUIColor)
                            .padding(.vertical, 8)
                        Text(Localizable.Scanner.Label.progress(model.captured, model.total))
                            .font(Fontstyle.subtitle1.base)
                            .foregroundColor(Asset.text600.swiftUIColor)
                        Localizable.pleaseHoldStill.text
                            .font(Fontstyle.subtitle2.base)
                            .foregroundColor(Asset.text400.swiftUIColor)
                        MenuButtonsStack {
                            BigButton(
                                text: Localizable.CameraView.startOver.key,
                                isShaded: true,
                                action: {
                                    model.reset()
                                }
                            )
                        }
                    }.padding(.bottom, -20)
                }
            }
        }.background(Asset.bg100.swiftUIColor)
    }
}

// struct CameraView_Previews: PreviewProvider {
// static var previews: some View {
// CameraView()
// }
// }
