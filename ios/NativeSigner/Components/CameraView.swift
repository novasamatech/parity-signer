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
    var body: some View {
        ZStack {
            VStack {
                Spacer()
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
                    .padding(.horizontal, 8)
                //.overlay(RoundedRectangle(cornerRadius: 8).stroke(Color("cryptoColor")))
                
                
               // if model.total ?? 0 > 0 {
                    VStack {
                        Text("MULTIPART DATA")
                        ProgressView(value: min(Float(model.captured ?? 0)/(Float(model.total ?? -1) + 2), 1))
                            .border(Color("Crypto400"))
                            .frame(height: 7.0)
                            .foregroundColor(Color("Crypto400"))
                            .padding(8)
                            .background(Color("Bg100"))
                        Text(String(model.captured ?? 0) + "/" + String(model.total ?? 0))
                        Button (action: {
                            model.reset()
                        }) {
                            Text("Start over")
                        }
                    }
              //  }
            }
        }.background(Color("Bg100"))
    }
}

/*
 struct CameraView_Previews: PreviewProvider {
 static var previews: some View {
 CameraView()
 }
 }*/
