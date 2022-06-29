//
//  CameraPreview.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 20.7.2021.
//

import SwiftUI
import AVFoundation

struct CameraPreview: UIViewRepresentable {
    class VideoPreviewView: UIView {
        override class var layerClass: AnyClass {
            AVCaptureVideoPreviewLayer.self
        }

        var videoPreviewLayer: AVCaptureVideoPreviewLayer {
            return layer as! AVCaptureVideoPreviewLayer // swiftlint:disable:this force_cast
        }
    }

    let session: AVCaptureSession
    let size = UIScreen.main.bounds.size.width - 24

    func makeUIView(context: Context) -> VideoPreviewView {
        let view = VideoPreviewView()
        view.videoPreviewLayer.session = session
        view.videoPreviewLayer.connection?.videoOrientation = .portrait
        return view
    }

    func updateUIView(_ uiView: VideoPreviewView, context: Context) {
    }
}
