//
//  CameraPreview.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 20.7.2021.
//

import AVFoundation
import SwiftUI
import UIKit

/// UIKit -> SwiftUI bridge view to display and handle camera stream in UI layer
struct CameraPreview: UIViewRepresentable {
    typealias UIViewType = CameraPreviewUIView

    var session: AVCaptureSession

    static func dismantleUIView(_ uiView: CameraPreviewUIView, coordinator _: ()) {
        uiView.videoPreviewLayer = nil
    }

    func makeUIView(context _: UIViewRepresentableContext<CameraPreview>) -> CameraPreview.UIViewType {
        let uiView = CameraPreviewUIView()
        let previewLayer = AVCaptureVideoPreviewLayer(session: session)
        previewLayer.videoGravity = .resizeAspectFill
        uiView.layer.addSublayer(previewLayer)
        uiView.videoPreviewLayer = previewLayer
        return uiView
    }

    func updateUIView(_ uiView: CameraPreviewUIView, context _: UIViewRepresentableContext<CameraPreview>) {
        uiView.setContentHuggingPriority(.defaultHigh, for: .vertical)
        uiView.setContentHuggingPriority(.defaultLow, for: .horizontal)
    }
}
