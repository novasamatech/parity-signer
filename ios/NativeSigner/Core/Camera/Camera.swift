//
//  Camera.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 20.7.2021.
//

/// This contains standard Apple boilerplate to generate basic camera preview

import AVFoundation
import Combine
import Foundation

final class CameraViewModel: ObservableObject {
    private let service = CameraService()
    private let cameraPermissionHandler = CameraPermissionHandler()

    @Published var payload: String?
    @Published var captured: Int?
    @Published var total: Int?

    @Published var showAlertError = false

    var isFlashOn = false

    var session: AVCaptureSession

    private var subscriptions = Set<AnyCancellable>()

    init() {
        session = service.session

        service.$payload.sink { [weak self] payload in
            guard let value = payload else { return }
            self?.payload = value
        }
        .store(in: &subscriptions)

        service.$captured.sink { [weak self] captured in
            guard let value = captured else { return }
            self?.captured = value
        }
        .store(in: &subscriptions)

        service.$total.sink { [weak self] total in
            guard let value = total else { return }
            self?.total = value
        }
        .store(in: &subscriptions)
    }

    func configure() {
        cameraPermissionHandler.checkForPermissions { [weak self] isGranted in
            guard let self = self else { return }
            self.service.setupResult = isGranted ? .success : .notAuthorized
            if !isGranted {
                self.service.isCameraUnavailable = true
            }
            self.service.configure()
        }
    }

    func shutdown() {
        print(payload ?? "No payload catpured by camera")
        service.stop()
    }

    /// Clears recorded frames and starts anew
    func reset() {
        service.emptyBucket()
        captured = nil
        total = nil
    }
}
