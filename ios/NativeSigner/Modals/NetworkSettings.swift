//
//  NetworkSettings.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 12.8.2021.
//

import Foundation

/**
 * Metadata descriptor
 */
struct MetaSpecsNS: Codable {
    var spec_version: String
    var meta_hash: String
}

/**
 * Detailed network settings
 */
struct NetworkSettings: Codable {
    var base58prefix: String
    var color: String
    var decimals: String
    var genesis_hash: String
    var logo: String
    var name: String
    var order: String
    var path_id: String
    var secondary_color: String
    var title: String
    var unit: String
    var verifier: String
    var meta: [MetaSpecsNS]
}

/**
 * Operations on network in Settings screen
 */
extension SignerDataModel {
    func getNetworkSettings() {
        let dbName = NSHomeDirectory() + "/Documents/Database"
        var err = ExternError()
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        let res = get_network_specs(err_ptr, self.selectedNetwork?.key, dbName)
        if err_ptr.pointee.code == 0 {
            if let specsJSON = String(cString: res!).data(using: .utf8) {
                guard let networkSettings = try? JSONDecoder().decode(NetworkSettings.self, from: specsJSON) else {
                    print("JSON decoder failed on network specs")
                    print(specsJSON)
                    self.networkSettings = nil
                    signer_destroy_string(res!)
                    return
                    }
                self.networkSettings = networkSettings
            } else {
                print("Network specs JSON corrupted!")
                print(String(cString: res!))
                self.networkSettings = nil
            }
            signer_destroy_string(res!)
        } else {
            self.networkSettings = nil
            self.lastError = String(cString: err_ptr.pointee.message)
            print("Rust returned error")
            print(self.lastError)
            signer_destroy_string(err_ptr.pointee.message)
        }
    }
}
