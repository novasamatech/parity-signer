//
//  CaptureDeviceConfigurator.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 11/10/2022.
//

import AVKit
import Foundation

protocol CaptureDeviceConfiguring: AnyObject {
    /// Configures given `session` with all
    /// - Parameters:
    ///   - session: capture session to configure
    ///   - delegate: delegate that will receive new video buffer samples
    ///   - videoOutputQueue: queue on which sample buffer delegate will be called
    /// - Returns: Whether capture device configuration was successful or not
    func configure(
        session: AVCaptureSession,
        with delegate: AVCaptureVideoDataOutputSampleBufferDelegate,
        videoOutputQueue: DispatchQueue
    ) -> Bool

    func toggleTorch() -> Bool
}

final class CaptureDeviceConfigurator: CaptureDeviceConfiguring {
    func configure(
        session: AVCaptureSession,
        with delegate: AVCaptureVideoDataOutputSampleBufferDelegate,
        videoOutputQueue: DispatchQueue
    ) -> Bool {
        defer { session.commitConfiguration() }
        guard let videoDevice = AVCaptureDevice.default(
            for: .video
        ) else {
            return false
        }
        session.beginConfiguration()
        session.sessionPreset = .high

        let videoInputConfigured = configureVideoInput(for: session, videoDevice: videoDevice)
        guard videoInputConfigured else {
            return false
        }

        let videoOutputConfigured = configureVideoOutput(
            for: session,
            delegate: delegate,
            videoOutputQueue: videoOutputQueue
        )
        guard videoOutputConfigured else {
            return false
        }

        return true
    }

    func toggleTorch() -> Bool {
        guard let camera = AVCaptureDevice.default(for: .video), camera.hasTorch else { return false }
        try? camera.lockForConfiguration()
        camera.torchMode = camera.torchMode == .off ? .on : .off
        camera.unlockForConfiguration()
        return camera.torchMode == .on
    }
}

private extension CaptureDeviceConfigurator {
    func configureVideoInput(for session: AVCaptureSession, videoDevice: AVCaptureDevice) -> Bool {
        do {
            try videoDevice.lockForConfiguration()
            videoDevice.focusMode = .continuousAutoFocus
            videoDevice.unlockForConfiguration()
            let videoDeviceInput = try AVCaptureDeviceInput(device: videoDevice)
            if session.canAddInput(videoDeviceInput) {
                session.addInput(videoDeviceInput)
                return true
            } else {
                return false
            }
        } catch {
            return false
        }
    }

    func configureVideoOutput(
        for session: AVCaptureSession,
        delegate: AVCaptureVideoDataOutputSampleBufferDelegate,
        videoOutputQueue: DispatchQueue
    ) -> Bool {
        let videoDataOutput = AVCaptureVideoDataOutput()
        videoDataOutput.alwaysDiscardsLateVideoFrames = true
        videoDataOutput.setSampleBufferDelegate(delegate, queue: videoOutputQueue)

        if session.canAddOutput(videoDataOutput) {
            session.addOutput(videoDataOutput)
            videoDataOutput.connection(with: .video)?.isEnabled = true
            return true
        } else {
            return false
        }
    }
}
