//
//  CameraService.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 20.7.2021.
//

import AVKit
import SwiftUI
import UIKit

enum DecodedPayloadType: Equatable {
    case transaction
    case dynamicDerivations
    case dynamicDerivationsTransaction
}

struct DecodedPayload: Equatable {
    let payload: [String]
    let type: DecodedPayloadType
}

final class CameraService: ObservableObject {
    private enum CameraSessionSetupResult {
        case success
        case notAuthorized
        case configurationFailed
    }

    /// QR codes payloads for multiple transactions
    @Published var multipleTransactions: [String] = []
    /// Informs whether camera service should handle individual QR code batch scanning
    @Published var isMultipleTransactionMode: Bool = false

    /// QR code payload decoded by Rust
    @Published var payload: DecodedPayload?
    /// Number of expected frames for given payload
    @Published var total: Int = 0
    /// Number of already captured frames for given payload
    @Published var captured: Int = 0

    @Published var isTorchOn: Bool = false
    @Published var requestPassword: Bool = false

    /// Partial payload to decode, collection of payloads from individual QR codes
    @Published var bucket: [String] = []

    let session: AVCaptureSession
    private var isConfigured = false
    private var setupResult: CameraSessionSetupResult = .success

    private let sessionQueue = DispatchQueue(label: "session queue")
    private let stitcherQueue = DispatchQueue.global(qos: .userInitiated)
    private let videoDataOutputQueue = DispatchQueue.global(qos: .userInteractive)
    private let callbackQueue = DispatchQueue.main
    private let captureDeviceConfigurator: CaptureDeviceConfiguring
    private let cameraPermissionHandler: CameraPermissionHandler
    private let videoOutputDelegate: CameraVideoOutputDelegate

    init(
        session: AVCaptureSession = AVCaptureSession(),
        captureDeviceConfigurator: CaptureDeviceConfiguring = CaptureDeviceConfigurator(),
        cameraPermissionHandler: CameraPermissionHandler = CameraPermissionHandler(),
        videoOutputDelegate: CameraVideoOutputDelegate = CameraVideoOutputDelegate()
    ) {
        self.session = session
        self.captureDeviceConfigurator = captureDeviceConfigurator
        self.cameraPermissionHandler = cameraPermissionHandler
        self.videoOutputDelegate = videoOutputDelegate
        videoOutputDelegate.set(updateReceiver: self)
    }

    func configure() {
        cameraPermissionHandler.checkForPermissions { [weak self] isGranted in
            guard let self else { return }
            setupResult = isGranted ? .success : .notAuthorized
            sessionQueue.async(execute: configureSession)
        }
    }

    func start() {
        clearLocalState()
        sessionQueue.async(execute: startSession)
    }

    func shutdown() {
        guard session.isRunning, setupResult == .success else { return }
        sessionQueue.async(execute: stopSession)
    }

    func reset() {
        payload = nil
        clearLocalState()
    }

    func toggleTorch() {
        isTorchOn = captureDeviceConfigurator.toggleTorch()
    }
}

extension CameraService: QRPayloadUpdateReceiving {
    func didReceive(update qrCodePayload: String) {
        guard !bucket.contains(qrCodePayload) else { return }
        stitcherQueue.async { self.handleNew(qrCodePayload: qrCodePayload) }
    }
}

private extension CameraService {
    func handleNew(qrCodePayload: String) {
        if isMultipleTransactionMode {
//            multiTransactionOperation(with: qrCodePayload)
            return
        }
        if bucket.isEmpty {
            handleNewOperation(with: qrCodePayload)
        } else {
            appendToCurrentBucket(qrCodePayload: qrCodePayload)
        }
    }

//    func multiTransactionOperation(with qrCodePayload: String) {
//        guard let parsedPayload = try? qrparserTryDecodeQrSequence(data: [qrCodePayload], password: nil, cleaned:
//        false)
//        else { return }
//        callbackQueue.async {
//            guard !self.multipleTransactions.contains(parsedPayload) else { return }
//            self.multipleTransactions.append(parsedPayload)
//        }
//    }

    func handleNewOperation(with qrCodePayload: String) {
        do {
            let proposedTotalFrames = try Int(qrparserGetPacketsTotal(data: qrCodePayload, cleaned: false))
            switch proposedTotalFrames {
            case 1:
                decode(completeOperationPayload: [qrCodePayload])
            default:
                callbackQueue.async {
                    self.bucket.append(qrCodePayload)
                    self.captured = self.bucket.count
                    self.total = proposedTotalFrames
                }
            }
        } catch {
            // reset camera on failure?
        }
    }

    /// Collect frames and attempt to decode if it seems that enough are collected
    func appendToCurrentBucket(qrCodePayload: String) {
        callbackQueue.async {
            self.bucket.append(qrCodePayload)
            self.captured = self.bucket.count
        }
        guard total <= bucket.count else { return }
        decode(completeOperationPayload: bucket)
    }

    func decode(completeOperationPayload: [String]) {
        do {
            let result = try qrparserTryDecodeQrSequence(data: completeOperationPayload, password: nil, cleaned: false)
            callbackQueue.async {
                switch result {
                case let .bBananaSplitRecoveryResult(b: bananaResult):
                    switch bananaResult {
                    case .requestPassword:
                        self.requestPassword = true
                        self.shutdown()
                    case .recoveredSeed:
                        () // Invalid code path, BS can't be recovered without a password
                    }
                case let .dynamicDerivations(s: payload):
                    self.payload = .init(payload: [payload], type: .dynamicDerivations)
                    self.shutdown()
                case let .other(s: payload):
                    self.payload = .init(payload: [payload], type: .transaction)
                    self.shutdown()
                case let .dynamicDerivationTransaction(s: payload):
                    self.payload = .init(payload: payload, type: .dynamicDerivationsTransaction)
                    self.shutdown()
                }
            }
        } catch {
            print(error.localizedDescription)
        }
    }
}

private extension CameraService {
    func configureSession() {
        guard setupResult == .success else { return }
        let configurationResult = captureDeviceConfigurator.configure(
            session: session,
            with: videoOutputDelegate,
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
    }

    func startSession() {
        guard !session.isRunning, isConfigured else { return }
        if setupResult == .success {
            session.startRunning()
        }
    }

    func clearLocalState() {
        callbackQueue.async {
            self.requestPassword = false
            self.total = 0
            self.captured = 0
            self.bucket = []
        }
    }
}
