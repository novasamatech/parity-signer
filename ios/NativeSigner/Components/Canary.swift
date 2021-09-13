//
//  Canary.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 5.8.2021.
//

import Foundation
import Network

/**
 * This is background network indicator. It will paint the shield icon red and write to history
 * NOTE: This might sometimes crash transaction; it is intended although not defined behavior for now
 */
class Canary: ObservableObject {
    @Published var dead: Bool = false
    let monitor = NWPathMonitor()
    let queue = DispatchQueue.global(qos: .background)
    
    init() {
        self.monitor.pathUpdateHandler = {path in
            if path.availableInterfaces.count == 0 {
                DispatchQueue.main.async {
                    self.dead = false
                }
            } else {
                DispatchQueue.main.async {
                    let dbName = NSHomeDirectory() + "/Documents/Database"
                    device_was_online(nil, dbName)
                    self.dead = true
                }
            }
        }
        
        monitor.start(queue: self.queue)
    }
}
