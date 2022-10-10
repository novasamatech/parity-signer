//
//  CameraPermissionHandler.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 07/10/2022.
//

import AVKit

final class CameraPermissionHandler {
    func checkForPermissions(_ completion: @escaping (Bool) -> Void) {
        switch AVCaptureDevice.authorizationStatus(for: .video) {
        case .authorized:
            completion(true)
        case .notDetermined:
            AVCaptureDevice.requestAccess(for: .video) { _ in
                DispatchQueue.main.async {
                    completion(false)
                }
            }
        case .denied,
             .restricted:
            completion(false)
        @unknown default:
            completion(false)
        }
    }
}
