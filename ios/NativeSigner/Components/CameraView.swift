//
//  CameraView.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI
import AVFoundation

struct CameraView: View {
    @StateObject var model = CameraViewModel()
    @EnvironmentObject var data: SignerDataModel
    @State var total: Int? = 0
    @State var captured: Int? = 0
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
                        model.shutdown()
                    }
                    .onReceive(model.$payload, perform: { payload in
                        if payload != nil {
                            DispatchQueue.main.async {
                                data.pushButton(action: .transactionFetched, details: payload ?? "")
                            }
                        }
                    })
                    .onReceive(data.$resetCamera, perform: { resetCamera in
                        if resetCamera {
                            model.reset()
                            data.resetCamera = false
                        }
                    })
                    .onReceive(model.$total, perform: {rTotal in
                        total = rTotal
                    })
                    .onReceive(model.$captured, perform: {rCaptured in
                        captured = rCaptured
                    })
                    .mask(
                        VStack {
                            ZStack {
                                RoundedRectangle(cornerRadius: 8).padding(12)
                            }
                            .frame(width:size, height: size)
                            Spacer()
                        }
                    )
                    .overlay(
                        VStack {
                            RoundedRectangle(cornerRadius: 8).stroke(Color("Crypto400")).padding(12).frame(width: size, height: size)
                            Spacer()
                        }
                    )
                Spacer()
                
                if model.total ?? 0 > 0 {
                    
                    MenuStack {
                        HeadingOverline(text: "PARSING MULTIPART DATA").padding(.top, 12)
                        ProgressView(value: min(Float(captured ?? 0)/(Float(total ?? -1) + 2), 1))
                            .border(Color("Crypto400"))
                            .foregroundColor(Color("Crypto400"))
                            .padding(.vertical, 8)
                        Text(constructFrameCountMessage(captured: model.captured, total: model.total))
                            .font(FBase(style: .subtitle1))
                            .foregroundColor(Color("Text600"))
                        Text("Please hold still")
                            .font(FBase(style: .subtitle2))
                            .foregroundColor(Color("Text400"))
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
        }.background(Color("Bg100"))
    }
}

func constructFrameCountMessage(captured: Int?, total: Int?) -> String {
    return "From "
         + String(captured ?? 0)
         + " / "
         + String(total ?? 0)
         + " captured frames"
}

/*
 struct CameraView_Previews: PreviewProvider {
 static var previews: some View {
 CameraView()
 }
 }*/
