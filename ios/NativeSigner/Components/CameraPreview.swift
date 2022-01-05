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
            return layer as! AVCaptureVideoPreviewLayer
        }
    }
    
    let session: AVCaptureSession
    let size = UIScreen.main.bounds.size.width - 24
    
    func makeUIView(context: Context) -> VideoPreviewView {
        let view = VideoPreviewView()
        view.videoPreviewLayer.cornerRadius = 4
        view.videoPreviewLayer.session = session
        view.videoPreviewLayer.connection?.videoOrientation = .portrait
        view.videoPreviewLayer.borderColor = UIColor(Color("Crypto400")).cgColor
        view.videoPreviewLayer.borderWidth = 1
        view.videoPreviewLayer.bounds = CGRect(x: 0, y: 0, width: size, height: size)
        
        return view
    }
    
    func updateUIView(_ uiView: VideoPreviewView, context: Context) {
        
    }
}
