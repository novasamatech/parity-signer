//
//  CameraService.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 20.7.2021.
//

/**
 * Logic behind the camera - boilerplate, parsing QRs into payloads,
 * collection of payloads and their transfer to Rust decoder
 * Some concurrency
 */

import AVKit
import Vision
import UIKit

public class CameraService: UIViewController, AVCaptureVideoDataOutputSampleBufferDelegate {
    @Published public var isCameraUnavailable = true
    @Published public var payload: String?
    @Published public var total: Int?
    @Published public var captured: Int?

    public let session = AVCaptureSession()
    var isSessionRunning = false
    var isConfigured = false
    var setupResult: SessionSetupResult = .success

    private let sessionQueue = DispatchQueue(label: "session queue")
    private let stitcherQueue = DispatchQueue(label: "stitcher queue")

    @objc dynamic var videoDeviceInput: AVCaptureDeviceInput!

    private let videoDataOutput = AVCaptureVideoDataOutput()
    private let videoDataOutputQueue = DispatchQueue(label: "qr code detection queue")

    private var detectionRequests: [VNDetectBarcodesRequest] = [VNDetectBarcodesRequest(
        completionHandler: { (request, error) in
            if error != nil {
                print("QR code detection error: \(String(describing: error))")
            }

            guard let barcodeDetectionRequest = request as? VNDetectBarcodesRequest,
                  let results = barcodeDetectionRequest.results else {
                      return
                  }
            barcodeDetectionRequest.symbologies = [VNBarcodeSymbology.qr]
        })]

    private var bucketCount = 0

    private var bucket: [String] = []

    public func configure() {
        sessionQueue.async {
            self.configureSession()
        }
    }

    public func checkForPermissions() {
        switch AVCaptureDevice.authorizationStatus(for: .video) {
        case .authorized:
            break
        case .notDetermined:
            sessionQueue.suspend()
            AVCaptureDevice.requestAccess(for: .video, completionHandler: { granted in
                if !granted {
                    self.setupResult = .notAuthorized
                }
                self.sessionQueue.resume()
            })
        default:
            setupResult = .notAuthorized

            DispatchQueue.main.async {
                self.isCameraUnavailable = true
            }
        }
    }

    public func start() {
        self.bucket = []
        sessionQueue.async {
            if !self.isSessionRunning && self.isConfigured {
                switch self.setupResult {
                case .success:
                    self.session.startRunning()
                    self.isSessionRunning = self.session.isRunning

                    if self.session.isRunning {
                        DispatchQueue.main.async {
                            self.isCameraUnavailable = false
                        }
                    }
                case .configurationFailed, .notAuthorized:
                    print("Camera configuration invalid")

                    DispatchQueue.main.sync {
                        self.isCameraUnavailable = true
                    }
                }
            }
        }
    }

    public func stop(completion: (() -> Void)? = nil) {
        sessionQueue.async {
            if self.isSessionRunning {
                if self.setupResult == .success {
                    self.session.stopRunning()
                    self.isSessionRunning = self.session.isRunning

                    if !self.session.isRunning {
                        DispatchQueue.main.async {
                            self.isCameraUnavailable = true
                            completion?()
                        }
                    }
                }
            }
        }
    }

    // swiftlint:disable:next function_body_length - because it's boilerplate
    private func configureSession() {
        if setupResult != .success {
            return
        }

        session.beginConfiguration()

        session.sessionPreset = .photo

        do {
            guard let videoDevice = AVCaptureDevice.default(
                .builtInWideAngleCamera,
                for: .video,
                position: .back
            ) else {
                print("Default camera is unavailable")
                setupResult = .configurationFailed
                session.commitConfiguration()
                return
            }

            try videoDevice.lockForConfiguration()
            videoDevice.focusMode = .autoFocus
            videoDevice.unlockForConfiguration()

            let videoDeviceInput = try AVCaptureDeviceInput(device: videoDevice)

            if session.canAddInput(videoDeviceInput) {
                session.addInput(videoDeviceInput)
                self.videoDeviceInput = videoDeviceInput
            } else {
                print("Couldn't add camera input to the session")
                setupResult = .configurationFailed
                session.commitConfiguration()
                return
            }
        } catch {
            print("Couldn't create video device input: \(error)")
            setupResult = .configurationFailed
            session.commitConfiguration()
            return
        }

        videoDataOutput.alwaysDiscardsLateVideoFrames = true
        videoDataOutput.setSampleBufferDelegate(self, queue: videoDataOutputQueue)

        if session.canAddOutput(videoDataOutput) {
            session.addOutput(videoDataOutput)

            videoDataOutput.connection(with: .video)?.isEnabled = true
        } else {
            print("Could not add metadata output to the session")
            setupResult = .configurationFailed
            session.commitConfiguration()
            return
        }

        session.commitConfiguration()

        self.isConfigured = true
        self.start()
    }

    /**
     * Callback for receiving buffer - payload assembly is fed from here
     */
    // TODO: move this to Rust
    // swiftlint:disable:next cyclomatic_complexity function_body_length
    public func captureOutput(
        _ output: AVCaptureOutput,
        didOutput sampleBuffer: CMSampleBuffer,
        from connection: AVCaptureConnection
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

        if let result = detectionRequests[0].results {
            if result.count != 0 {
                // uncomment to see how fast qr reader goes brrr
                // print(String(reflecting: result))
                if result.count>1 {
                    print("lagging!")
                    print(result.count)
                }
                if let descriptor = result[0].barcodeDescriptor as? CIQRCodeDescriptor {
                    // Actual QR handling starts here
                    let payloadStr = descriptor.errorCorrectedPayload.map {String(format: "%02x", $0)}.joined()
                    stitcherQueue.async {
                        if !self.bucket.contains(payloadStr) {
                            if self.total == nil { // init new collection of frames
                                do {
                                    let res = try qrparserGetPacketsTotal(data: payloadStr, cleaned: false)
                                    let proposeTotal = Int(res)
                                    if proposeTotal == 1 { // Special handling for 1-frame payloads
                                        let process = "[\"" + payloadStr + "\"]" // Decoder expects JSON array
                                        let res2 = try qrparserTryDecodeQrSequence(data: process, cleaned: false)
                                        DispatchQueue.main.async {
                                            self.payload = res2
                                            self.stop()
                                        }
                                    } else {
                                        DispatchQueue.main.async {
                                            self.bucket.append(payloadStr)
                                            self.total = proposeTotal
                                        }
                                    }
                                } catch {
                                    // reset camera on failure?
                                }
                            } else { // collect frames and attempt to decode if it seems that enough are collected
                                self.bucket.append(payloadStr)
                                DispatchQueue.main.async {
                                    self.captured = self.bucket.count
                                }
                                if (self.bucket.count + 1) >= self.total ?? 0 {
                                    do {
                                        let process = "[\"" +
                                        self.bucket.joined(separator: "\",\"") +
                                        "\"]" // Decoder expects JSON array
                                        let res = try qrparserTryDecodeQrSequence(data: process, cleaned: false)
                                        DispatchQueue.main.async {
                                            self.payload = res
                                            self.stop()
                                        }
                                    } catch {
                                        // give up when things go badly?
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /**
     * Empty bucket
     */
    func emptyBucket() {
        payload = nil
        total = nil
        captured = nil
        bucketCount = 0
        bucket = []
    }
}

/**
 * Standard boilerplate for camera init
 */
extension CameraService {
    enum SessionSetupResult {
        case success
        case notAuthorized
        case configurationFailed
    }
}
