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
    @ObservedObject var transaction: Transaction
    @Environment(\.presentationMode) var presentationMode: Binding<PresentationMode>
    var body: some View {
        ZStack {
            CameraPreview(session: model.session)
                .onAppear {
                    model.configure()
                }
                .onDisappear {
                    model.shutdown()
                }.onReceive(model.$payload, perform: { payload in
                    if payload != nil {
                        transaction.payloadStr = payload ?? ""
                        DispatchQueue.main.async {
                            transaction.parse()
                        }
                        transaction.state = .parsing
                    }
                })
            VStack {
                Spacer()
                ProgressView(value: min(Float(model.captured ?? 0)/(Float(model.total ?? -1) + 2), 1)).padding().background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
                Button(action: {
                        presentationMode.wrappedValue.dismiss()}) {
                    Text("Cancel")
                        .font(.largeTitle)
                }
            }
        }.background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color.black/*@END_MENU_TOKEN@*/)
    }
}

struct CameraView_Previews: PreviewProvider {
    static var previews: some View {
        CameraView(transaction: Transaction())
    }
}
