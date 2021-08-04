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
    var body: some View {
        VStack {
            CameraPreview(session: model.session)
                .onAppear {
                    model.configure()
                }
                .onDisappear {
                    model.shutdown()
                }
            Text(model.payload ?? "Nothing")
        }
    }
}

struct CameraView_Previews: PreviewProvider {
    static var previews: some View {
        NavigationView {
            CameraView()
        }
    }
}
