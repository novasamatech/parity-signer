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
}

final class CaptureDeviceConfigurator: CaptureDeviceConfiguring {
    func configure(
        session: AVCaptureSession,
        with delegate: AVCaptureVideoDataOutputSampleBufferDelegate,
        videoOutputQueue: DispatchQueue
    ) -> Bool {
        defer { session.commitConfiguration() }
        guard let videoDevice = AVCaptureDevice.default(
            .builtInWideAngleCamera,
            for: .video,
            position: .back
        ) else {
            print("Default camera is unavailable")
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
                print("Couldn't add camera input to the session")
                return false
            }
        } catch {
            print("Couldn't create video device input: \(error)")
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
            print("Could not add metadata output to the session")
            return false
        }
    }
}
