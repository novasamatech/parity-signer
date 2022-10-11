//
//  CameraService.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 20.7.2021.
//

/// Logic behind the camera - boilerplate, parsing QRs into payloads,
/// collection of payloads and their transfer to Rust decoder
/// Some concurrency

import AVKit
import UIKit
import Vision

enum CameraSessionSetupResult {
    case success
    case notAuthorized
    case configurationFailed
}

final class CameraService: NSObject {
    @Published var isCameraUnavailable = true
    /// QR code payload decoded by Rust
    @Published var payload: String?
    /// Number of expected frames for given payload
    @Published var total: Int = 0
    /// Number of already captured frames for given payload
    @Published var captured: Int = 0
    /// Partial payload to decode, collection of payloads from individual QR codes
    private var bucket: [String] = []

    let session = AVCaptureSession()
    var isSessionRunning = false
    var isConfigured = false
    var setupResult: CameraSessionSetupResult = .success

    private let sessionQueue = DispatchQueue(label: "session queue")
    private let stitcherQueue = DispatchQueue.global(qos: .userInitiated)
    private let videoDataOutputQueue = DispatchQueue.global(qos: .userInteractive)
    private let callbackQueue = DispatchQueue.main
    private let captureDeviceConfigurator: CaptureDeviceConfiguring = CaptureDeviceConfigurator()

    func configure() {
        sessionQueue.async(execute: configureSession)
    }

    func start() {
        bucket = []
        sessionQueue.async(execute: startSession)
    }

    func stop() {
        guard isSessionRunning, setupResult == .success else { return }
        sessionQueue.async(execute: stopSession)
    }

    /// Empty bucket
    func emptyBucket() {
        payload = nil
        total = 0
        captured = 0
        bucket = []
    }
}

extension CameraService: AVCaptureVideoDataOutputSampleBufferDelegate {
    func captureOutput(_: AVCaptureOutput, didOutput sampleBuffer: CMSampleBuffer, from _: AVCaptureConnection) {
        guard let pixelBuffer = CMSampleBufferGetImageBuffer(sampleBuffer) else {
            print("Failed to obtain pixelbuffer for this frame")
            return
        }

        let imageRequestHandler = VNImageRequestHandler(cvPixelBuffer: pixelBuffer, options: [:])

        do {
            let detectionRequest = VNDetectBarcodesRequest(completionHandler: qrCodeDetection)
            detectionRequest.symbologies = [.qr]
            try imageRequestHandler.perform([detectionRequest])
        } catch {
            print("Failed to handle \(error)")
        }
    }
}

private extension CameraService {
    func qrCodeDetection(request: VNRequest, error: Error?) {
        if error != nil {
            print("QR code detection error: \(String(describing: error))")
        }

        guard
            let qrCodeDescriptor = (request as? VNDetectBarcodesRequest)?.results?.first?
            .barcodeDescriptor as? CIQRCodeDescriptor
        else { return }

        let qrPayloadAsString = qrCodeDescriptor.errorCorrectedPayload.map { String(format: "%02x", $0) }.joined()

        stitcherQueue.async {
            guard !self.bucket.contains(qrPayloadAsString) else { return }
            self.handleNew(qrCodePayload: qrPayloadAsString)
        }
    }

    func handleNew(qrCodePayload: String) {
        if total == 0 {
            // If `total == 0`, treat `qrCodePayload` as either start of new video QR or single QR code
            handleNewOperation(with: qrCodePayload)
        } else {
            appendToCurrentBucket(qrCodePayload: qrCodePayload)
        }
    }

    func handleNewOperation(with qrCodePayload: String) {
        do {
            let proposedTotalFrames = Int(try qrparserGetPacketsTotal(data: qrCodePayload, cleaned: false))
            switch proposedTotalFrames {
            case 1:
                decode(completeOperationPayload: [qrCodePayload])
            default:
                callbackQueue.async {
                    self.bucket.append(qrCodePayload)
                    self.total = proposedTotalFrames
                }
            }
        } catch {
            // reset camera on failure?
        }
    }

    /// Collect frames and attempt to decode if it seems that enough are collected
    func appendToCurrentBucket(qrCodePayload: String) {
        bucket.append(qrCodePayload)
        callbackQueue.async {
            self.captured = self.bucket.count
        }
        guard bucket.count + 1 >= total else { return }
        decode(completeOperationPayload: bucket)
    }

    func decode(completeOperationPayload: [String]) {
        do {
            let parsedPayload = try qrparserTryDecodeQrSequence(data: completeOperationPayload, cleaned: false)
            callbackQueue.async {
                self.payload = parsedPayload
                self.stop()
            }
        } catch {
            // give up when things go badly?
        }
    }
}

private extension CameraService {
    func configureSession() {
        guard setupResult == .success else { return }
        let configurationResult = captureDeviceConfigurator.configure(
            session: session,
            with: self,
            videoOutputQueue: videoDataOutputQueue
        )
        if configurationResult {
            isConfigured = true
            start()
        } else {
            setupResult = .configurationFailed
        }
    }

    func stopSession() {
        session.stopRunning()
        isSessionRunning = session.isRunning
        guard !isSessionRunning else { return }
        callbackQueue.async {
            self.isCameraUnavailable = true
        }
    }

    func startSession() {
        guard !isSessionRunning, isConfigured else { return }
        if setupResult == .success {
            session.startRunning()
            isSessionRunning = session.isRunning
        }
        callbackQueue.async {
            self.isCameraUnavailable = self.setupResult != .success
        }
    }
}
