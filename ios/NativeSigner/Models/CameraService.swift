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

final class CameraService: UIViewController, AVCaptureVideoDataOutputSampleBufferDelegate {
    @Published var isCameraUnavailable = true
    /// QR code payload decoded by Rust
    @Published var payload: String?
    /// Number of expected frames for given payload
    @Published var total: Int?
    /// Number of already captured frames for given payload
    @Published var captured: Int?
    /// Partial payload to decode, collection of payloads from individual QR codes
    private var bucket: [String] = []

    let session = AVCaptureSession()
    var isSessionRunning = false
    var isConfigured = false
    var setupResult: CameraSessionSetupResult = .success

    private let sessionQueue = DispatchQueue(label: "session queue")
    private let stitcherQueue = DispatchQueue(label: "stitcher queue")
    private let videoDataOutputQueue = DispatchQueue(label: "qr code detection queue")
    private let callbackQueue = DispatchQueue.main

    private var detectionRequests: [VNDetectBarcodesRequest] = [VNDetectBarcodesRequest(
        completionHandler: { request, error in
            if error != nil {
                print("QR code detection error: \(String(describing: error))")
            }

            guard let barcodeDetectionRequest = request as? VNDetectBarcodesRequest,
                  let results = barcodeDetectionRequest.results else {
                return
            }
            barcodeDetectionRequest.symbologies = [.qr]
        }
    )]

    func configure() {
        sessionQueue.async {
            self.configureSession()
        }
    }

    func start() {
        bucket = []
        sessionQueue.async {
            if !self.isSessionRunning, self.isConfigured {
                switch self.setupResult {
                case .success:
                    self.session.startRunning()
                    self.isSessionRunning = self.session.isRunning

                    if self.session.isRunning {
                        self.callbackQueue.async {
                            self.isCameraUnavailable = false
                        }
                    }
                case .configurationFailed,
                     .notAuthorized:
                    print("Camera configuration invalid")

                    self.callbackQueue.sync {
                        self.isCameraUnavailable = true
                    }
                }
            }
        }
    }

    func stop() {
        guard isSessionRunning,
              setupResult == .success else { return }
        sessionQueue.async {
            self.session.stopRunning()
            let isSessionRunning = self.session.isRunning
            self.isSessionRunning = isSessionRunning

            if !isSessionRunning {
                self.callbackQueue.async {
                    self.isCameraUnavailable = true
                }
            }
        }
    }

    /// Callback for receiving buffer - payload assembly is fed from here
    func captureOutput(
        _: AVCaptureOutput,
        didOutput sampleBuffer: CMSampleBuffer,
        from _: AVCaptureConnection
    ) {
        guard let pixelBuffer = CMSampleBufferGetImageBuffer(sampleBuffer) else {
            print("Failed to obtain pixelbuffer for this frame")
            return
        }

        let imageRequestHandler = VNImageRequestHandler(cvPixelBuffer: pixelBuffer, options: [:])

        do {
            try imageRequestHandler.perform(detectionRequests)
        } catch {
            print("Failed to handle \(error)")
        }

        guard
            let qrCodeDescriptor = detectionRequests.first?.results?.first?.barcodeDescriptor as? CIQRCodeDescriptor
        else { return }
        printDebugInformationOn(barcodeObservations: detectionRequests.first?.results)

        // Actual QR handling starts here
        let qrPayloadAsString = qrCodeDescriptor.errorCorrectedPayload.map { String(format: "%02x", $0) }.joined()
        print("Error corrected QR Code payload: \(qrPayloadAsString)")

        stitcherQueue.async {
            guard !self.bucket.contains(qrPayloadAsString) else { return }
            self.handleNew(qrCodePayload: qrPayloadAsString)
        }
    }

    func printDebugInformationOn(barcodeObservations: [VNBarcodeObservation]?) {
        #if DEBUG
            guard let barcodeObservations = barcodeObservations else { return }
            // Debug section, to be deleted
            // uncomment to see how fast qr reader goes brrr
            print(String(reflecting: barcodeObservations))
            if barcodeObservations.count > 1 {
                // Add additional handling for that case
                print("lagging!")
                print(barcodeObservations.count)
            }
        #endif
    }

    func handleNew(qrCodePayload: String) {
        if total == nil {
            handleNewOperation(with: qrCodePayload)
        } else {
            appendToCurrentBucket(qrCodePayload: qrCodePayload)
        }
    }

    /// If `total == nil`, treat `qrCodePayload` as either start of new video QR or single QR code
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
        DispatchQueue.main.async {
            self.captured = self.bucket.count
        }
        guard let total = total, bucket.count + 1 >= total else { return }
        decode(completeOperationPayload: bucket)
    }

    func decode(completeOperationPayload: [String]) {
        do {
            let parsedPayload = try qrparserTryDecodeQrSequence(data: completeOperationPayload, cleaned: false)
            DispatchQueue.main.async {
                self.payload = parsedPayload
                self.stop()
            }
        } catch {
            // give up when things go badly?
        }
    }

    /// Empty bucket
    func emptyBucket() {
        payload = nil
        total = nil
        captured = nil
        bucket = []
    }
}

private extension CameraService {
    func configureSession() {
        guard setupResult == .success else { return }
        guard let videoDevice = AVCaptureDevice.default(
            .builtInWideAngleCamera,
            for: .video,
            position: .back
        ) else {
            print("Default camera is unavailable")
            setupResult = .configurationFailed
            return
        }
        session.beginConfiguration()
        session.sessionPreset = .photo

        let videoInputConfigured = configureVideoInput(for: session, videoDevice: videoDevice)
        guard videoInputConfigured else {
            finaliseFailedConfiguration()
            return
        }

        let videoOutputConfigured = configureVideoOutput(for: session)
        guard videoOutputConfigured else {
            finaliseFailedConfiguration()
            return
        }

        session.commitConfiguration()
        isConfigured = true
        start()
    }

    func finaliseFailedConfiguration() {
        setupResult = .configurationFailed
        session.commitConfiguration()
    }

    func configureVideoInput(for session: AVCaptureSession, videoDevice: AVCaptureDevice) -> Bool {
        do {
            try videoDevice.lockForConfiguration()
            videoDevice.focusMode = .autoFocus
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

    func configureVideoOutput(for session: AVCaptureSession) -> Bool {
        let videoDataOutput = AVCaptureVideoDataOutput()
        videoDataOutput.alwaysDiscardsLateVideoFrames = true
        videoDataOutput.setSampleBufferDelegate(self, queue: videoDataOutputQueue)

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
