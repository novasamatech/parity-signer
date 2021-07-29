//
//  Camera.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 20.7.2021.
//

import Foundation
import Combine
import AVFoundation

final class CameraViewModel: ObservableObject {
    private let service = CameraService()
    
    @Published var payload: String?
    
    @Published var showAlertError = false
    
    @Published var isFlashOn = false
    
    var session: AVCaptureSession
    
    private var subscriptions = Set<AnyCancellable>()
    
    init() {
        self.session = service.session
        
        service.$payload.sink { [weak self] (payload) in
            guard let value = payload else {return}
            self?.payload = value
        }
        .store(in: &self.subscriptions)
    }
    
    func configure() {
        service.checkForPermissions()
        service.configure()
    }
}
