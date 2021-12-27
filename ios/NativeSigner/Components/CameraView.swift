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
    var body: some View {
        ZStack {
            //VStack {
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
                                data.pushButton(buttonID: .TransactionFetched, details: payload ?? "")
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
                //.clipShape(RoundedRectangle(cornerRadius: 8, style: .continuous))
                    //.padding(.horizontal, 8)
                //.overlay(RoundedRectangle(cornerRadius: 8).stroke(Color("Crypto400")))
                
                
                if model.total ?? 0 > 0 {
                
                    MenuStack {
                        HeadingOverline(text: "Multipart data").padding(.top, 12)
                        ProgressView(value: min(Float(captured ?? 0)/(Float(total ?? -1) + 2), 1))
                            .border(Color("Crypto400"))
                            .foregroundColor(Color("Crypto400"))
                            .padding(.vertical, 8)
                        Text(String(model.captured ?? 0) + "/" + String(model.total ?? 0) + " complete")
                            .font(FBase(style: .subtitle1))
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
                    }
                }
            //}
        }.background(Color("Bg100"))
    }
}

/*
 struct CameraView_Previews: PreviewProvider {
 static var previews: some View {
 CameraView()
 }
 }*/
