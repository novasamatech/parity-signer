//
//  Networks.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 26.7.2021.
//

import Foundation
import SwiftUI

class Networks: ObservableObject {
    @Published var data: [Network] = []
    var err = ExternError()
    
    init() {
        print("networks init")
        self.refresh()
    }
    
    func refresh() {
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        let dbName = NSHomeDirectory() + "/Documents/Database"
        let res = get_all_networks_for_network_selector(err_ptr, dbName)
        print("refresh call")
        if err_ptr.pointee.code == 0 {
            if let networksJSON = String(cString: res!).data(using: .utf8) {
            print(networksJSON)
                guard let networks = try? JSONDecoder().decode([Network].self, from: networksJSON) else {
                    print("JSON decoder failed")
                    print(networksJSON)
                    signer_destroy_string(res!)
                    return
                }
                self.data = networks
            } else {
                    print("metadataJSON corrupted")
                    print(String(cString: res!))
            }
            signer_destroy_string(res!)
        } else {
            print(String(cString: err_ptr.pointee.message))
            signer_destroy_string(err_ptr.pointee.message)
        }
    }
}

extension Networks {
    static var networksData: Networks {
        Networks()
    }
}
