//
//  CameraPermissionHandler.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 07/10/2022.
//

import AVKit

/// Protocol defining the authorization functionalities of `AVCaptureDevice`.
///
/// This protocol abstracts the methods related to checking and requesting
/// authorization status for media types like video.
protocol AVCaptureDeviceProtocol {
    /// Returns the current authorization status for a specified media type.
    ///
    /// - Parameter mediaType: The media type for which to check the authorization status.
    /// - Returns: The current authorization status for the given media type.
    static func authorizationStatus(for mediaType: AVMediaType) -> AVAuthorizationStatus

    /// Requests access to a specified media type.
    ///
    /// This method requests the user's permission to access the specified media type.
    /// The completion handler is called once the user has responded.
    ///
    /// - Parameters:
    ///   - mediaType: The media type for which access is requested.
    ///   - completionHandler: The completion handler to call when the access request
    ///     has been completed. It takes a Boolean value indicating whether access
    ///     was granted.
    static func requestAccess(for mediaType: AVMediaType, completionHandler: @escaping (Bool) -> Void)
}

extension AVCaptureDevice: AVCaptureDeviceProtocol {}

final class CameraPermissionHandler {
    private let captureDevice: AVCaptureDeviceProtocol.Type
    private let dispatcher: Dispatching

    init(
        captureDevice: AVCaptureDeviceProtocol.Type = AVCaptureDevice.self,
        dispatcher: Dispatching = DispatchQueue.main
    ) {
        self.captureDevice = captureDevice
        self.dispatcher = dispatcher
    }

    func checkForPermissions(_ completion: @escaping (Bool) -> Void) {
        switch captureDevice.authorizationStatus(for: .video) {
        case .authorized:
            completion(true)
        case .notDetermined:
            captureDevice.requestAccess(for: .video) { [weak self] isGranted in
                self?.dispatcher.async {
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
