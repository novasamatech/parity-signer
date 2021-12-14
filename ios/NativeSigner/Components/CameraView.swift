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

                ProgressView(value: min(Float(model.captured ?? 0)/(Float(model.total ?? -1) + 2), 1))
                    .border(Color("cryptoColor"))
                    .frame(height: 7.0)
                    .foregroundColor(Color("cryptoColor"))
                    .padding(8)
                    .background(Color("backgroundColor"))
                    
            }
        }.background(Color("backgroundColor"))
    }
}

/*
struct CameraView_Previews: PreviewProvider {
    static var previews: some View {
        CameraView()
    }
}*/
