//
//  CameraView.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import AVFoundation
import SwiftUI

struct CameraView: View {
    @StateObject var model = CameraViewModel()
    @State private var total: Int? = 0
    @State private var captured: Int? = 0
    @State private var resetCameraTrigger: Bool = false
    let navigationRequest: NavigationRequest
    let size = UIScreen.main.bounds.size.width
    var body: some View {
        ZStack {
            VStack {
                CameraPreview(session: model.session)
                    .onAppear {
                        model.configure()
                    }
                    .onDisappear {
                        print("shutdown camera")
                        UIApplication.shared.isIdleTimerDisabled = false
                        model.shutdown()
                    }
                    .onReceive(model.$payload, perform: { payload in
                        if payload != nil {
                            DispatchQueue.main.async {
                                navigationRequest(.init(action: .transactionFetched, details: payload))
                            }
                        }
                    })
                    .onChange(of: resetCameraTrigger, perform: { newResetCameraTrigger in
                        if newResetCameraTrigger {
                            model.reset()
                            resetCameraTrigger = false
                        }
                    })
                    .onReceive(model.$total, perform: { rTotal in
                        total = rTotal
                    })
                    .onReceive(model.$captured, perform: { rCaptured in
                        captured = rCaptured
                        if rCaptured ?? 0 > 0 {
                            UIApplication.shared.isIdleTimerDisabled = true
                        } else {
                            UIApplication.shared.isIdleTimerDisabled = false
                        }
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
                if model.total ?? 0 > 0 {
                    MenuStack {
                        HeadingOverline(text: "PARSING MULTIPART DATA").padding(.top, 12)
                        ProgressView(value: min(Float(captured ?? 0) / (Float(total ?? -1) + 2), 1))
                            .border(Asset.crypto400.swiftUIColor)
                            .foregroundColor(Asset.crypto400.swiftUIColor)
                            .padding(.vertical, 8)
                        Text(constructFrameCountMessage(captured: model.captured, total: model.total))
                            .font(Fontstyle.subtitle1.base)
                            .foregroundColor(Asset.text600.swiftUIColor)
                        Text("Please hold still")
                            .font(Fontstyle.subtitle2.base)
                            .foregroundColor(Asset.text400.swiftUIColor)
                        MenuButtonsStack {
                            BigButton(
                                text: "Start over",
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

func constructFrameCountMessage(captured: Int?, total: Int?) -> String {
    "From "
        + String(captured ?? 0)
        + " / "
        + String(total ?? 0)
        + " captured frames"
}

// struct CameraView_Previews: PreviewProvider {
// static var previews: some View {
// CameraView()
// }
// }
