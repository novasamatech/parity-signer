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
                        data.payloadStr = payload ?? ""
                        DispatchQueue.main.async {
                            data.parse()
                            print(String(describing: data.transactionState))
                        }
                        data.transactionState = .parsing
                    }
                })
                .onReceive(data.$resetCamera, perform: { resetCamera in
                    if resetCamera {
                        model.reset()
                        data.resetCamera = false
                    }
                })
            VStack {
                Spacer()
                ProgressView(value: min(Float(model.captured ?? 0)/(Float(model.total ?? -1) + 2), 1)).padding().background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
            }
        }.background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color.black/*@END_MENU_TOKEN@*/)
    }
}

/*
struct CameraView_Previews: PreviewProvider {
    static var previews: some View {
        CameraView()
    }
}*/
