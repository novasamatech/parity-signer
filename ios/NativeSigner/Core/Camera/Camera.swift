//
//  Camera.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 20.7.2021.
//

import AVFoundation
import Combine
import Foundation

final class CameraViewModel: ObservableObject {
    private let service = CameraService()
    private let cameraPermissionHandler = CameraPermissionHandler()

    @Published var payload: String?
    @Published var captured: Int = 0
    @Published var total: Int = 0

    @Published var showAlertError = false

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
            guard let self = self else { return }
            self.captured = captured
        }
        .store(in: &subscriptions)

        service.$total.sink { [weak self] total in
            guard let self = self else { return }
            self.total = total
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

    func reset() {
        service.emptyBucket()
    }
}
