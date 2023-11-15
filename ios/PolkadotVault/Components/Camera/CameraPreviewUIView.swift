//
//  CameraPreviewUIView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 06/10/2022.
//

import AVFoundation
import UIKit

/// UIKit wrapper for `AVCaptureSession` camera layer
final class CameraPreviewUIView: UIView {
    var videoPreviewLayer: AVCaptureVideoPreviewLayer?

    init() {
        super.init(frame: .zero)
    }

    @available(*, unavailable)
    required init?(coder _: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func layoutSubviews() {
        super.layoutSubviews()
        backgroundColor = .black
        videoPreviewLayer?.frame = bounds
        videoPreviewLayer?.connection?.videoOrientation = .portrait
    }
}
