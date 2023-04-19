//
//  CameraPermissionHandler.swift
//  Polkadot Vault
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
            AVCaptureDevice.requestAccess(for: .video) { isGranted in
                DispatchQueue.main.async {
                    completion(isGranted)
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
